//! Core methods for the env module.

use core::{
    mem::size_of,
    ptr::{self, addr_of, addr_of_mut, copy_nonoverlapping, null_mut},
};

use crate::{
    debugln,
    kdef::{
        cp0reg::*,
        elf::{Elf32Ehdr, Elf32Phdr, PT_LOAD},
        env::{EnvData, EnvList, EnvNode, EnvStatus, EnvTailList, LOG2NENV, NENV},
        error::KError,
        mmu::{
            KSTACKTOP, NASID, PAGE_SIZE, PDSHIFT, PGSHIFT, PTE_G, PTE_V, UENVS, UPAGES, USTACKTOP,
            UTOP, UVPT,
        },
    },
    kern::{elf::elf_load_seg, sched::schedule, trap::TrapFrame},
    memory::{
        pmap::{
            page_alloc, page_decref, page_insert, page_remove, PageNode, Pde, Pte, CUR_PGDIR,
            NPAGE, PAGES,
        },
        tlbex::tlb_invalidate,
    },
    pa2page, page2kva, println, ENVX, KADDR, PADDR, PDX, PTE_ADDR, PTX, ROUND,
};

/// Wrapper to make it aligned to a page.
#[repr(align(4096))]
pub struct EnvsWrapper<T>([T; NENV]);

/// The global pgdir.
pub static mut BASE_PGDIR: *mut Pde = null_mut();

/// The current env.
pub static mut CUR_ENV: *mut EnvNode = null_mut();

/// Free env list.
pub static mut ENV_FREE_LIST: EnvList = EnvList::new();
/// Runnable env list.
pub static mut ENV_SCHE_LIST: EnvTailList = EnvTailList::new();
/// The envs array in *kernel*, mapped to the [UENVS] and used by the user
/// program.
pub static mut ENVS_DATA: EnvsWrapper<EnvData> = EnvsWrapper([EnvData::const_construct(); NENV]);
/// The envs array used in the *kernel* space. The element in it has the link
/// field to make it a link node.
pub static mut ENVS: EnvsWrapper<EnvNode> = EnvsWrapper([EnvNode::const_construct(); NENV]);

/// Bitmap for the asid allocatoin.
pub static mut ASID_BITMAP: [u32; (NASID / 32) as usize] = [0; (NASID / 32) as usize];

/// Alloc a new asid. Return an error if there is no asid remained.
fn asid_alloc() -> Result<u32, KError> {
    for i in 0..NASID as usize {
        let index = i >> 5;
        let inner = i & 31;
        unsafe {
            if ASID_BITMAP[index] & (1 << inner) == 0 {
                ASID_BITMAP[index] |= 1 << inner;
                return Ok(i as u32);
            }
        }
    }
    Err(KError::NoFreeEnv)
}

/// Free an asid.
fn asid_free(i: u32) {
    let index = i as usize >> 5;
    let inner = i & 31;
    unsafe { ASID_BITMAP[index] &= !(1 << inner) };
}

/// Map the [va, va + size) in virtual address space to the [pa, pa + size) in
/// physical address space if `pgdir`. This method will validate the entry perm.
///
/// # Panic
/// - Panic if one of the `pa`, `va`, `size` is not aligned to a [PAGE_SIZE].
/// - Panic if the [page_insert] failed.
///
/// # Safety
/// The `pgdir` **SHALL** be valid.
unsafe fn map_segment(pgdir: *mut Pde, asid: u32, pa: usize, va: usize, size: usize, perm: u32) {
    assert_eq!(0, pa % PAGE_SIZE);
    assert_eq!(0, va % PAGE_SIZE);
    assert_eq!(0, size % PAGE_SIZE);
    debugln!("> env.rs: map_segment() with size=0x{:x}", size);
    for i in (0..size).step_by(PAGE_SIZE) {
        unsafe {
            let pp = pa2page!(pa + i, PAGES; PageNode) as *mut PageNode;
            page_insert(pgdir, va + i, asid, perm, pp).unwrap();
        }
    }
}

/// The static variable used by [mkenvid];
static mut ENV_I: u32 = 0;

/// Get the envid with the env node.
fn mkenvid(e: *mut EnvNode) -> u32 {
    unsafe {
        ENV_I += 1;
        (ENV_I << (1 + LOG2NENV)) | (e.offset_from(addr_of_mut!(ENVS.0[0]))) as u32
    }
}

/// Init the env environment. Put the envs into the free list, and map the PAGES
/// and ENVS to base_pgdir's UPAGES and UENVS accordingly.
pub fn env_init() {
    unsafe {
        debugln!("> env_init: enable the tailq ENV_SCHE_LIST");
        ENV_SCHE_LIST.enable();
    }

    println!("Envs are to the memory 0x{:x}", unsafe {
        addr_of_mut!(ENVS) as usize
    });
    for i in (0..NENV).rev() {
        unsafe {
            ENVS.0[i].data = addr_of_mut!(ENVS_DATA.0[i]);
            (*ENVS.0[i].data).status = EnvStatus::Free; // Useless this line
            ENV_FREE_LIST.insert_head(addr_of_mut!(ENVS.0[i]));
        }
    }

    unsafe {
        let p = page_alloc().unwrap();
        (*p).data.pp_ref += 1;
        let base_pgdir = page2kva!(p, PAGES; PageNode) as *mut Pde;
        map_segment(
            base_pgdir,
            0,
            PADDR!(PAGES as usize),
            UPAGES,
            ROUND!(NPAGE * size_of::<PageNode>(); PAGE_SIZE),
            PTE_G,
        );
        map_segment(
            base_pgdir,
            0,
            PADDR!(addr_of_mut!(ENVS_DATA) as usize),
            UENVS,
            ROUND!(NENV * size_of::<EnvData>(); PAGE_SIZE),
            PTE_G,
        );
        BASE_PGDIR = base_pgdir;
    }

    debugln!("> env.rs: env init sucsess");
}

/// Setup the virtual memory of the new-born env.
///
/// # Return
/// A [KError] wrapper with `Err` will be returned if the page_alloc fails.
///
/// # Safety
/// The `env` **SHALL** be valid.
pub unsafe fn env_setup_vm(env: *mut EnvNode) -> Result<(), KError> {
    let p = page_alloc()?;

    (*p).data.pp_ref += 1;
    (*(*env).data).pgdir = page2kva!(p, PAGES; PageNode) as *mut Pde;

    // memcpy
    copy_nonoverlapping(
        BASE_PGDIR.wrapping_add(PDX!(UTOP)),
        (*(*env).data).pgdir.wrapping_add(PDX!(UTOP)),
        PDX!(UVPT) - PDX!(UTOP),
    );

    ptr::write(
        (*(*env).data).pgdir.wrapping_add(PDX!(UVPT)),
        PADDR!((*(*env).data).pgdir as u32) | PTE_V,
    );

    Ok(())
}

/// Alloc an `env` and setup its vm and PCB.
///
/// # Return
///
/// If there is at least one free env, and the [env_setup_vm] works well, the
/// raw pointer to the new env will be returned with a `Ok` wrapper.
///
/// Otherwise, a [KError] with a `Err` wrapper will be returned if there is no
/// free env ([KError::NoFreeEnv]) or `env_setup_vm` fails.
///
/// # Safety
/// The env got in the free list **SHALL** be valid.
pub unsafe fn env_alloc(parent_id: u32) -> Result<*mut EnvNode, KError> {
    let e = ENV_FREE_LIST.pop_head().ok_or(KError::NoFreeEnv)?;
    env_setup_vm(e)?;

    (*(*e).data).user_tlb_mod_entry = 0;
    (*(*e).data).env_runs = 0;
    (*(*e).data).id = mkenvid(e);
    (*(*e).data).asid = asid_alloc()?;
    (*(*e).data).parent_id = parent_id;
    (*(*e).data).trap_frame.cp0_status = STATUS_IM7 | STATUS_IE | STATUS_EXL | STATUS_UM;
    (*(*e).data).trap_frame.regs[29] = (USTACKTOP - size_of::<u32>() - size_of::<*mut u8>()) as u32;

    Ok(e)
}

/// Free an env, and remove all its pages. The TLB will be flushed after the
/// deletion of pages.
///
/// # Safety
/// The `env` and all its pages **SHALL** be valid.
pub unsafe fn env_free(env: *mut EnvNode) {
    debugln!(
        "% {}: Free env {}",
        if CUR_ENV.is_null() {
            0
        } else {
            (*(*CUR_ENV).data).id
        },
        (*(*env).data).id
    );

    for pdeno in 0..PDX!(UTOP) {
        if *((*(*env).data).pgdir.add(pdeno)) & PTE_V == 0 {
            continue;
        }

        let pa = PTE_ADDR!(*((*(*env).data).pgdir.add(pdeno)));
        let pt = KADDR!(pa) as *mut Pte;
        for pteno in 0..PTX!(!0) {
            if *(pt.add(pteno)) & PTE_V != 0 {
                page_remove(
                    (*(*env).data).pgdir,
                    (pdeno << PDSHIFT) | (pteno << PGSHIFT),
                    (*(*env).data).asid,
                );
            }
        }
        ptr::write((*(*env).data).pgdir.add(pdeno), 0);
        page_decref(&mut (pa2page!(pa, PAGES; PageNode) as *mut PageNode));
        tlb_invalidate((*(*env).data).asid, UVPT + (pdeno << PGSHIFT));
    }

    page_decref(
        &mut (pa2page!(PADDR!((*(*env).data).pgdir as usize), PAGES; PageNode) as *mut PageNode),
    );
    asid_free((*(*env).data).asid);
    tlb_invalidate((*(*env).data).asid, UVPT + (PDX!(UVPT) << PGSHIFT));
    (*(*env).data).status = EnvStatus::Free;

    ENV_SCHE_LIST.remove(env);
    ENV_FREE_LIST.insert_head(env);
}

/// Destory an env and free it. Re-schedule will be performed.
///
/// # Safety
/// The `env` and all its pages **SHALL** be valid.
pub unsafe fn env_destory(env: *mut EnvNode) {
    env_free(env);

    if CUR_ENV == env {
        CUR_ENV = null_mut();
        debugln!("% Killed.");
        schedule(true);
    }
}

/// Get the env's PCB by its id. If `checkperm` is set, the method will check
/// whether the env just is [CUR_ENV] or the child of CUR_ENV. If not,
/// [KError::BadEnv] will be returned with a `Err` wrapper.
///
/// # Safety
/// The `e` get by index **SHELL** be valid.
pub unsafe fn envid2env(envid: u32, checkperm: bool) -> Result<*mut EnvNode, KError> {
    if 0 == envid {
        return Ok(CUR_ENV);
    }
    let e = (addr_of_mut!(ENVS) as *mut EnvNode).add(ENVX!(envid as usize));
    if (*(*e).data).status == EnvStatus::Free || (*(*e).data).id != envid {
        return Err(KError::BadEnv);
    }

    // Need to check the perm
    if checkperm
        && (((*(*e).data).id != (*(*CUR_ENV).data).id
            && (*(*e).data).parent_id != (*(*CUR_ENV).data).id)
            || CUR_ENV.is_null())
    {
        Err(KError::BadEnv)
    } else {
        Ok(e)
    }
}

/// Load the icode for the `e`.
///
/// # Safety
/// The `binary` **SHALL** be readable for `size` bytes.
pub unsafe fn load_icode(e: *mut EnvNode, binary: *const u8, size: usize) {
    let mapper = |env: *const EnvNode,
                  va: usize,
                  offset: isize,
                  perm: u32,
                  src: *const u8,
                  len: usize|
     -> Result<(), KError> {
        let p = page_alloc()?;
        if !src.is_null() {
            copy_nonoverlapping(
                src,
                (page2kva!(p, PAGES; PageNode) as *mut u8).offset(offset),
                len,
            )
        }
        page_insert((*(*env).data).pgdir, va, (*(*env).data).asid, perm, p)
    };

    let ehdr = Elf32Ehdr::from(binary, size);
    if ehdr.is_null() {
        panic!("Bad elf detected!");
    }

    (*ehdr).foreach(|ph_off| {
        let ph = binary.add(ph_off as usize) as *const Elf32Phdr;
        if (*ph).stype == PT_LOAD {
            elf_load_seg(ph, binary.add((*ph).offset as usize), mapper, e).unwrap();
        }
    });

    (*(*e).data).trap_frame.cp0_epc = (*ehdr).entry;
}

/// Create an env and load the icode, set the priority. For **tests** mainly.
///
/// # Safety
/// The `binary` **SHALL** be readable for `size` bytes.
pub unsafe fn env_create(binary: *const u8, size: usize, priority: u32) -> Option<*mut EnvNode> {
    let e = env_alloc(0).ok()?;

    (*(*e).data).priority = priority;
    (*(*e).data).status = EnvStatus::Runnable;

    load_icode(e, binary, size);
    ENV_SCHE_LIST.insert_head(e);

    Some(e)
}

extern "C" {
    /// Recover from exception, load the specified **TrapFrame**.
    pub fn env_pop_tf(_1: *const TrapFrame, _2: u32) -> !;
}

/// Run before the `env_run` for **tests** only
pub static mut PRE_ENV_RUN: fn(*mut EnvNode) = |_| {};

/// Run the env. Save the [CUR_ENV]'s trapframe if `CUR_ENV` exists.
///
/// # Safety
/// The `env` **SHALL** be valid and runnable.
pub unsafe fn env_run(env: *mut EnvNode) -> ! {
    assert_eq!(
        EnvStatus::Runnable,
        (*(*env).data).status,
        "Id: {}",
        (*(*env).data).id
    );

    PRE_ENV_RUN(env);

    if !CUR_ENV.is_null() {
        (*(*CUR_ENV).data).trap_frame = *((KSTACKTOP as *const TrapFrame).sub(1));
    }

    CUR_ENV = env;
    (*(*CUR_ENV).data).env_runs += 1;

    CUR_PGDIR = (*(*CUR_ENV).data).pgdir;

    env_pop_tf(
        addr_of!((*(*CUR_ENV).data).trap_frame),
        (*(*CUR_ENV).data).asid,
    );
}

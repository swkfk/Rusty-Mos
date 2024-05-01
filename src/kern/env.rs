use core::{
    mem::size_of,
    ptr::{self, addr_of_mut, copy_nonoverlapping, null_mut},
};

use crate::{
    debugln,
    kdef::{
        cp0reg::*,
        env::{EnvList, EnvNode, EnvStatus, EnvTailList, LOG2NENV, NENV},
        error::KError,
        mmu::{
            NASID, PAGE_SIZE, PDSHIFT, PGSHIFT, PTE_G, PTE_V, UENVS, UPAGES, USTACKTOP, UTOP, UVPT,
        },
    },
    kern::{
        pmap::{page_decref, page_insert, page_remove, PageNode, Pte, NPAGE, PAGES},
        tlbex::tlb_invalidate,
    },
    pa2page, page2kva, println, ENVX, KADDR, PADDR, PDX, PTE_ADDR, PTX, ROUND,
};

use super::pmap::{page_alloc, Pde};

#[repr(align(4096))]
pub struct EnvsWrapper([EnvNode; NENV]);

pub static mut BASE_PGDIR: *mut Pde = null_mut();

pub static mut CUR_ENV: *mut EnvNode = null_mut();

pub static mut ENV_FREE_LIST: EnvList = EnvList::new();
pub static mut ENV_SCHE_LIST: EnvTailList = EnvTailList::new();
pub static mut ENVS: EnvsWrapper = EnvsWrapper([EnvNode::const_construct(); NENV]);

pub static mut ASID_BITMAP: [u32; (NASID / 32) as usize] = [0; (NASID / 32) as usize];

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

fn asid_free(i: u32) {
    let index = i as usize >> 5;
    let inner = i & 31;
    unsafe { ASID_BITMAP[index] &= !(1 << inner) };
}

/// # Safety
///
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

static mut ENV_I: u32 = 0;

fn mkenvid(e: *mut EnvNode) -> u32 {
    unsafe {
        ENV_I += 1;
        (ENV_I << (1 + LOG2NENV)) | (e.offset_from(addr_of_mut!(ENVS.0[0]))) as u32
    }
}

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
            ENVS.0[i].data.status = EnvStatus::Free; // Useless this line
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
            PADDR!(addr_of_mut!(ENVS) as usize),
            UENVS,
            ROUND!(NENV * size_of::<EnvNode>(); PAGE_SIZE),
            PTE_G,
        );
        BASE_PGDIR = base_pgdir;
    }

    debugln!("> env.rs: env init sucsess");
}

/// # Safety
///
pub unsafe fn env_setup_vm(env: *mut EnvNode) -> Result<(), KError> {
    let p = page_alloc()?;

    (*p).data.pp_ref += 1;
    (*env).data.pgdir = page2kva!(p, PAGES; PageNode) as *mut Pde;

    // memcpy
    copy_nonoverlapping(
        BASE_PGDIR.wrapping_add(PDX!(UTOP)),
        (*env).data.pgdir.wrapping_add(PDX!(UTOP)),
        PDX!(UVPT) - PDX!(UTOP),
    );

    ptr::write(
        (*env).data.pgdir.wrapping_add(PDX!(UVPT)),
        PADDR!((*env).data.pgdir as u32) | PTE_V,
    );

    Ok(())
}

/// # Safety
///
pub unsafe fn env_alloc(parent_id: u32) -> Result<*mut EnvNode, KError> {
    let e = ENV_FREE_LIST.pop_head().ok_or(KError::NoFreeEnv)?;
    env_setup_vm(e)?;

    (*e).data.user_tlb_mod_entry = 0;
    (*e).data.env_runs = 0;
    (*e).data.id = mkenvid(e);
    (*e).data.asid = asid_alloc()?;
    (*e).data.parent_id = parent_id;
    (*e).data.trap_frame.cp0_status = STATUS_IM7 | STATUS_IE | STATUS_EXL | STATUS_UM;
    (*e).data.trap_frame.regs[29] = (USTACKTOP - size_of::<u32>() - size_of::<*mut u8>()) as u32;

    Ok(e)
}

/// # Safety
///
pub unsafe fn env_free(env: *mut EnvNode) {
    println!(
        "% {}: Free env {}",
        if CUR_ENV.is_null() {
            0
        } else {
            (*CUR_ENV).data.id
        },
        (*env).data.id
    );

    for pdeno in 0..PDX!(UTOP) {
        if *((*env).data.pgdir.add(pdeno)) & PTE_V == 0 {
            continue;
        }

        let pa = PTE_ADDR!(*((*env).data.pgdir.add(pdeno)));
        let pt = KADDR!(pa) as *mut Pte;
        for pteno in 0..PTX!(!0) {
            if *(pt.add(pteno)) & PTE_V != 0 {
                page_remove(
                    (*env).data.pgdir,
                    (pdeno << PDSHIFT) | (pteno << PGSHIFT),
                    (*env).data.asid,
                );
            }
        }
        ptr::write((*env).data.pgdir.add(pdeno), 0);
        page_decref(&mut (pa2page!(pa, PAGES; PageNode) as *mut PageNode));
        tlb_invalidate((*env).data.asid, UVPT + (pdeno << PGSHIFT));
    }

    page_decref(
        &mut (pa2page!(PADDR!((*env).data.pgdir as usize), PAGES; PageNode) as *mut PageNode),
    );
    asid_free((*env).data.asid);
    tlb_invalidate((*env).data.asid, UVPT + (PDX!(UVPT) << PGSHIFT));
    (*env).data.status = EnvStatus::Free;

    ENV_FREE_LIST.insert_head(env);
    ENV_SCHE_LIST.remove(env);
}

/// # Safety
///
pub unsafe fn envid2env(envid: u32, checkperm: bool) -> Result<*mut EnvNode, KError> {
    if 0 == envid {
        return Ok(CUR_ENV);
    }
    let e = (addr_of_mut!(ENVS) as *mut EnvNode).add(ENVX!(envid as usize));
    if (*e).data.status == EnvStatus::Free || (*e).data.id != envid {
        return Err(KError::BadEnv);
    }

    // Need to check the perm
    if checkperm
        && (((*e).data.id != (*CUR_ENV).data.id && (*e).data.parent_id != (*CUR_ENV).data.id)
            || CUR_ENV.is_null())
    {
        Err(KError::BadEnv)
    } else {
        Ok(e)
    }
}

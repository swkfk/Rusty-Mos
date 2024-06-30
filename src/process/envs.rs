//! Core methods for the env module.

use core::{
    mem::size_of,
    ptr::{self, addr_of, copy_nonoverlapping, null_mut},
    sync::atomic::{AtomicU32, AtomicUsize, Ordering::SeqCst},
};

use crate::{
    arch_mipsel::trap::TrapFrame,
    consts::{cp0reg::*, error::KError},
    debugln,
    memory::{
        pmap::{
            page_alloc, page_decref, page_insert, page_remove, PageNode, Pde, Pte, CUR_PGDIR,
            NPAGE, PAGES,
        },
        regions::{
            KSTACKTOP, NASID, PAGE_SIZE, PDSHIFT, PGSHIFT, PTE_G, PTE_V, UENVS, UPAGES, USTACKTOP,
            UTOP, UVPT,
        },
        shared_pool::MEMORY_POOL,
        tlbex::tlb_invalidate,
    },
    pa2page, page2kva, println,
    process::{
        elf_loader::{elf_load_seg, Elf32Ehdr, Elf32Phdr, PT_LOAD},
        scheduler::schedule,
    },
    utils::{
        array_based_list::{Aligned, ArrayLinkedList},
        sync_ref_cell::SyncImplRef as SyncRef,
    },
    ENVX, KADDR, PADDR, PDX, PTE_ADDR, ROUND,
};

/// The env status enum. Compatible with the C-Like memory structure.
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum EnvStatus {
    #[default]
    /// The env is not used (free).
    Free = 0,
    /// The env is running or to be run.
    Runnable,
    /// The env is blocked.
    NotRunnable,
}

/// The IPC data collected together. Compatible with the C-Like memory
/// structure.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IpcData {
    /// The value passed directly.
    pub value: u32,
    /// The sender's env id.
    pub from_id: u32,
    /// Mark this env's receiving status.
    pub receiving: bool,
    /// The target virtual address.
    pub dstva: u32,
    /// The page permission.
    pub perm: u32,
}

impl IpcData {
    /// Used for the static construction. All members are filled with zero.
    pub const fn const_construct() -> Self {
        IpcData {
            value: 0,
            from_id: 0,
            receiving: false,
            dstva: 0,
            perm: 0,
        }
    }
}

/// The PCB struct. Compatible with the C-Like memory structure and the MOS.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EnvData {
    /// Trap Frame stored in the PCB.
    pub trap_frame: TrapFrame,
    /// Unused placeholder item.
    _place_holder_env_link: [u32; 2],
    /// The env id.
    pub id: u32,
    /// The asid for TLB.
    pub asid: u32,
    /// The env's parent env's id;
    pub parent_id: u32,
    /// The running status of this env.
    pub status: EnvStatus,
    /// The page directory address of this env.
    pub pgdir: *mut Pde,
    /// Unused placeholder item.
    _place_holder_env_sched_link: [u32; 2],
    /// The priority of this env.
    pub priority: u32,
    /// The IPC data collected.
    pub ipc_data: IpcData,
    /// The entry of the tlb mod handler in user space.
    pub user_tlb_mod_entry: u32,
    /// Used in Lab 6. ///
    pub env_runs: u32,
}

impl Default for EnvData {
    fn default() -> Self {
        Self::new()
    }
}

impl EnvData {
    /// Used for the static construction. All members are filled with zero.
    pub const fn new() -> Self {
        Self {
            trap_frame: TrapFrame::const_construct(),
            id: 0,
            asid: 0,
            pgdir: null_mut(),
            parent_id: 0,
            status: EnvStatus::Free,
            priority: 0,
            ipc_data: IpcData::const_construct(),
            user_tlb_mod_entry: 0,
            env_runs: 0,
            _place_holder_env_link: [0, 0],
            _place_holder_env_sched_link: [0, 0],
        }
    }
}

/// The log of [NENV].
pub const LOG2NENV: u8 = 10;
/// The count of the envs.
pub const NENV: usize = 1 << LOG2NENV;

/// Spawn the index of the given `envid`.
#[macro_export]
macro_rules! ENVX {
    ($envid: expr) => {
        $envid & ($crate::process::envs::NENV - 1)
    };
}

/// Wrapper to make it aligned to a page.
#[repr(align(4096))]
pub struct EnvsWrapper<T>([T; NENV]);

/// The global pgdir.
pub static BASE_PGDIR: SyncRef<*mut Pde> = SyncRef::new(null_mut());

/// The current env.
pub static CUR_ENV_IDX: AtomicUsize = AtomicUsize::new(NENV);

/// Free env list.
pub static ENV_FREE_LIST: SyncRef<ArrayLinkedList<NENV>> = SyncRef::new(ArrayLinkedList::new());
/// Runnable env list.
pub static ENV_SCHE_LIST: SyncRef<ArrayLinkedList<NENV>> = SyncRef::new(ArrayLinkedList::new());
/// The envs array in *kernel*, mapped to the [UENVS] and used by the user
/// program.
pub static ENVS_DATA: SyncRef<Aligned<EnvData, NENV>> =
    SyncRef::new(Aligned([EnvData::new(); NENV]));
/// The envs array used in the *kernel* space. The element in it has the link
/// field to make it a link node.
// pub static mut ENVS: EnvsWrapper<EnvNode> = EnvsWrapper([EnvNode::const_construct(); NENV]);

/// Bitmap for the asid allocatoin.
pub static ASID_BMAP: SyncRef<[u32; NASID as usize >> 5]> = SyncRef::new([0; NASID as usize >> 5]);

/// Alloc a new asid. Return an error if there is no asid remained.
fn asid_alloc() -> Result<u32, KError> {
    for i in 0..NASID as usize {
        let index = i >> 5;
        let inner = i & 31;
        if ASID_BMAP.borrow()[index] & (1 << inner) == 0 {
            ASID_BMAP.borrow_mut()[index] |= 1 << inner;
            return Ok(i as u32);
        }
    }
    Err(KError::NoFreeEnv)
}

/// Free an asid.
fn asid_free(i: u32) {
    let index = i as usize >> 5;
    let inner = i & 31;
    ASID_BMAP.borrow_mut()[index] &= !(1 << inner);
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
fn map_segment(pgdir: *mut Pde, asid: u32, pa: usize, va: usize, size: usize, perm: u32) {
    assert_eq!(0, pa % PAGE_SIZE);
    assert_eq!(0, va % PAGE_SIZE);
    assert_eq!(0, size % PAGE_SIZE);
    debugln!("> env.rs: map_segment() with size=0x{:x}", size);
    for i in (0..size).step_by(PAGE_SIZE) {
        let pp = pa2page!(pa + i, *PAGES.borrow(); PageNode) as *mut PageNode;
        page_insert(pgdir, va + i, asid, perm, pp).unwrap();
    }
}

/// The static variable used by [mkenvid];
static ENV_I: AtomicU32 = AtomicU32::new(1);

/// Get the envid with the env node.
fn mkenvid(index: usize) -> u32 {
    let env_i = ENV_I.fetch_add(1, SeqCst);
    (env_i << (1 + LOG2NENV)) | index as u32
}

/// Init the env environment. Put the envs into the free list, and map the PAGES
/// and ENVS to base_pgdir's UPAGES and UENVS accordingly.
pub fn env_init() {
    // unsafe {
    //     debugln!("> env_init: enable the tailq ENV_SCHE_LIST");
    //     ENV_SCHE_LIST.enable();
    // }

    println!(
        "Envs are to the memory 0x{:x}",
        addr_of!(ENVS_DATA.borrow().0[0]) as usize
    );
    for i in (0..NENV).rev() {
        ENVS_DATA.borrow_mut().0[i].status = EnvStatus::Free; // Useless this line
        ENV_FREE_LIST.borrow_mut().insert_head(i);
    }

    let p = page_alloc().unwrap();

    unsafe { (*p).data.pp_ref += 1 }
    let base_pgdir = page2kva!(p, *PAGES.borrow(); PageNode) as *mut Pde;
    map_segment(
        base_pgdir,
        0,
        PADDR!(*PAGES.borrow() as usize),
        UPAGES,
        ROUND!(*NPAGE.borrow() * size_of::<PageNode>(); PAGE_SIZE),
        PTE_G,
    );
    map_segment(
        base_pgdir,
        0,
        PADDR!(addr_of!(ENVS_DATA.borrow().0[0]) as usize),
        UENVS,
        ROUND!(NENV * size_of::<EnvData>(); PAGE_SIZE),
        PTE_G,
    );
    *BASE_PGDIR.borrow_mut() = base_pgdir;

    debugln!("> env.rs: env init sucsess");
}

/// Setup the virtual memory of the new-born env.
///
/// # Return
/// A [KError] wrapper with `Err` will be returned if the page_alloc fails.
///
/// # Safety
/// The `env` **SHALL** be valid.
pub fn env_setup_vm(env_index: usize) -> Result<(), KError> {
    let p = page_alloc()?;

    unsafe { (*p).data.pp_ref += 1 }
    ENVS_DATA.borrow_mut().0[env_index].pgdir = page2kva!(p, *PAGES.borrow(); PageNode) as *mut Pde;

    let env_pdgir = ENVS_DATA.borrow_mut().0[env_index].pgdir;

    unsafe {
        // memcpy
        copy_nonoverlapping(
            BASE_PGDIR.borrow().wrapping_add(PDX!(UTOP)),
            env_pdgir.wrapping_add(PDX!(UTOP)),
            PDX!(UVPT) - PDX!(UTOP),
        );

        ptr::write(
            env_pdgir.wrapping_add(PDX!(UVPT)),
            PADDR!(env_pdgir as u32) | PTE_V,
        );
    }

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
pub fn env_alloc(parent_id: u32) -> Result<usize, KError> {
    let e = ENV_FREE_LIST
        .borrow_mut()
        .pop_head()
        .ok_or(KError::NoFreeEnv)?;
    env_setup_vm(e)?;

    ENVS_DATA.borrow_mut().0[e].user_tlb_mod_entry = 0;
    ENVS_DATA.borrow_mut().0[e].env_runs = 0;
    ENVS_DATA.borrow_mut().0[e].id = mkenvid(e);
    ENVS_DATA.borrow_mut().0[e].asid = asid_alloc()?;
    ENVS_DATA.borrow_mut().0[e].parent_id = parent_id;
    ENVS_DATA.borrow_mut().0[e].trap_frame.cp0_status =
        STATUS_IM7 | STATUS_IE | STATUS_EXL | STATUS_UM;
    ENVS_DATA.borrow_mut().0[e].trap_frame.regs[29] =
        (USTACKTOP - size_of::<u32>() - size_of::<*mut u8>()) as u32;

    Ok(e)
}

/// Free an env, and remove all its pages. The TLB will be flushed after the
/// deletion of pages.
///
/// # Safety
/// The `env` and all its pages **SHALL** be valid.
pub fn env_free(env_index: usize) {
    let cur_index = CUR_ENV_IDX.load(SeqCst);
    debugln!(
        "% {}: Free env {}",
        if cur_index == NENV {
            0
        } else {
            ENVS_DATA.borrow().0[cur_index].id
        },
        ENVS_DATA.borrow().0[env_index].id
    );

    for pdeno in 0..PDX!(UTOP) {
        if unsafe { *(ENVS_DATA.borrow().0[env_index].pgdir.add(pdeno)) } & PTE_V == 0 {
            continue;
        }

        let pa = unsafe { PTE_ADDR!(*(ENVS_DATA.borrow().0[env_index].pgdir.add(pdeno))) };
        let pt = KADDR!(pa) as *mut Pte;
        for pteno in 0..PAGE_SIZE / size_of::<Pte>() {
            if unsafe { *(pt.add(pteno)) } & PTE_V != 0 {
                page_remove(
                    ENVS_DATA.borrow().0[env_index].pgdir,
                    (pdeno << PDSHIFT) | (pteno << PGSHIFT),
                    ENVS_DATA.borrow().0[env_index].asid,
                );
            }
        }
        unsafe { ptr::write(ENVS_DATA.borrow().0[env_index].pgdir.add(pdeno), 0) };
        page_decref(&mut (pa2page!(pa, *PAGES.borrow(); PageNode) as *mut PageNode));
        tlb_invalidate(
            ENVS_DATA.borrow().0[env_index].asid,
            UVPT + (pdeno << PGSHIFT),
        );
    }

    page_decref(
        &mut (pa2page!(PADDR!(ENVS_DATA.borrow().0[env_index].pgdir as usize), *PAGES.borrow(); PageNode)
            as *mut PageNode),
    );
    asid_free(ENVS_DATA.borrow().0[env_index].asid);
    tlb_invalidate(
        ENVS_DATA.borrow().0[env_index].asid,
        UVPT + (PDX!(UVPT) << PGSHIFT),
    );
    ENVS_DATA.borrow_mut().0[env_index].status = EnvStatus::Free;

    if ENV_SCHE_LIST.borrow_mut().contains(env_index) {
        ENV_SCHE_LIST.borrow_mut().remove(env_index);
    }
    ENV_FREE_LIST.borrow_mut().insert_head(env_index);

    MEMORY_POOL
        .borrow_mut()
        .destory_env(ENVS_DATA.borrow().0[env_index].id as usize);
}

/// Destory an env and free it. Re-schedule will be performed.
///
/// # Safety
/// The `env` and all its pages **SHALL** be valid.
pub fn env_destory(env_index: usize) {
    env_free(env_index);

    if CUR_ENV_IDX.load(SeqCst) == env_index {
        CUR_ENV_IDX.store(NENV, SeqCst);
        debugln!("% Killed.");

        schedule(true);
    }
}

/// Get the env's PCB by its id. If `checkperm` is set, the method will check
/// whether the env just is the *current* env or the child of it. If not,
/// [KError::BadEnv] will be returned with a `Err` wrapper.
///
/// # Safety
/// The `e` get by index **SHELL** be valid.
pub fn envid2env(envid: u32, checkperm: bool) -> Result<usize, KError> {
    let cur_index = CUR_ENV_IDX.load(SeqCst);
    if 0 == envid {
        return Ok(cur_index);
    }
    let e = ENVX!(envid as usize);
    let t_id = ENVS_DATA.borrow().0[e].id;
    let p_id = ENVS_DATA.borrow().0[e].parent_id;

    if ENVS_DATA.borrow().0[e].status == EnvStatus::Free || t_id != envid {
        return Err(KError::BadEnv);
    }

    // Need to check the perm
    if checkperm
        && ((t_id != ENVS_DATA.borrow().0[cur_index].id
            && p_id != ENVS_DATA.borrow().0[cur_index].id)
            || cur_index == NENV)
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
fn load_icode(e: usize, binary: *const u8, size: usize) {
    let mapper = |env_index: usize,
                  va: usize,
                  offset: isize,
                  perm: u32,
                  src: *const u8,
                  len: usize|
     -> Result<(), KError> {
        let p = page_alloc()?;
        if !src.is_null() {
            unsafe {
                copy_nonoverlapping(
                    src,
                    (page2kva!(p, *PAGES.borrow(); PageNode) as *mut u8).offset(offset),
                    len,
                )
            }
        }
        page_insert(
            ENVS_DATA.borrow().0[env_index].pgdir,
            va,
            ENVS_DATA.borrow().0[env_index].asid,
            perm,
            p,
        )
    };

    let ehdr = Elf32Ehdr::from(binary, size);
    if ehdr.is_null() {
        panic!("Bad elf detected!");
    }

    unsafe { *ehdr }.foreach(|ph_off| {
        let ph = binary.wrapping_add(ph_off as usize) as *const Elf32Phdr;
        let phdr = unsafe { *ph };
        if phdr.stype == PT_LOAD {
            elf_load_seg(ph, binary.wrapping_add(phdr.offset as usize), mapper, e).unwrap();
        }
    });

    ENVS_DATA.borrow_mut().0[e].trap_frame.cp0_epc = unsafe { (*ehdr).entry };
}

/// Create an env and load the icode, set the priority. For **tests** mainly.
///
/// # Safety
/// The `binary` **SHALL** be readable for `size` bytes.
pub fn env_create(binary: *const u8, size: usize, priority: u32) -> Option<usize> {
    let e = env_alloc(0).ok()?;

    ENVS_DATA.borrow_mut().0[e].priority = priority;
    ENVS_DATA.borrow_mut().0[e].status = EnvStatus::Runnable;

    load_icode(e, binary, size);
    ENV_SCHE_LIST.borrow_mut().insert_head(e);

    Some(e)
}

extern "C" {
    /// Recover from exception, load the specified **TrapFrame**.
    pub fn env_pop_tf(_1: *const TrapFrame, _2: u32) -> !;
}

/// Run the env. Save the *current* env's trapframe if it exists.
pub fn env_run(env_index: usize) -> ! {
    let mut env_data = ENVS_DATA.borrow_mut();
    assert_eq!(
        EnvStatus::Runnable,
        env_data.0[env_index].status,
        "Id: {}",
        env_data.0[env_index].id
    );

    // PRE_ENV_RUN(env);
    let cur_env_idx = CUR_ENV_IDX.load(SeqCst);

    if cur_env_idx != NENV {
        unsafe {
            env_data.0[cur_env_idx].trap_frame = *((KSTACKTOP as *const TrapFrame).sub(1));
        }
    }

    CUR_ENV_IDX.store(env_index, SeqCst);
    env_data.0[env_index].env_runs += 1;

    *CUR_PGDIR.borrow_mut() = env_data.0[env_index].pgdir;

    let tf = addr_of!(env_data.0[env_index].trap_frame);
    let asid = env_data.0[env_index].asid;
    drop(env_data);
    unsafe {
        env_pop_tf(tf, asid);
    }
}

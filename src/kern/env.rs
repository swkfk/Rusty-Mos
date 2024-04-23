use core::{mem::size_of, ptr::addr_of_mut};

use crate::{
    debugln,
    kdef::{
        env::{EnvList, EnvNode, EnvStatus, EnvTailList, NENV},
        error::KError,
        mmu::{NASID, PAGE_SIZE, PTE_G, UENVS, UPAGES},
    },
    kern::pmap::{page_insert, PageNode, PAGES},
    pa2page, page2kva, println, PADDR, ROUND,
};

use super::pmap::{page_alloc, Pde};

#[repr(align(4096))]
pub struct EnvsWrapper([EnvNode; NENV]);

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
            page_insert(pgdir, va, asid, perm, pp).unwrap();
        }
    }
}

pub fn env_init(npage: usize) {
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
            ROUND!(npage * size_of::<PageNode>(); PAGE_SIZE),
            PTE_G,
        );
        map_segment(
            base_pgdir,
            0,
            PADDR!(addr_of_mut!(ENVS) as usize),
            UENVS,
            ROUND!(NENV * size_of::<EnvNode>(); PAGE_SIZE),
            PTE_G,
        )
    }

    debugln!("> env.rs: env init sucsess");
}

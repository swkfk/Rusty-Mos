use crate::{
    kdef::mmu::{NASID, PAGE_SIZE, PGSHIFT, PTE_D, UENVS, ULIM, UPAGES, USTACKTOP, UTEMP, UVPT},
    GEN_MASK, PTE_ADDR,
};

use crate::kern::pmap::{
    page_alloc, page_insert, page_lookup, PageList, PageNode, Pde, Pte, CUR_PGDIR,
};

static mut PAGES: *mut PageNode = core::ptr::null_mut();
static mut PAGE_FREE_LIST: *mut PageList = core::ptr::null_mut();

extern "C" {
    pub fn tlb_out(entryhi: u32);
}

pub fn tlb_init_global_vars(page_free_list: &mut PageList, pages: &*mut PageNode) {
    unsafe {
        PAGES = *pages;
        PAGE_FREE_LIST = core::ptr::addr_of_mut!(*page_free_list);
    }
}

pub fn tlb_invalidate(asid: u32, va: usize) {
    unsafe {
        tlb_out((va & !GEN_MASK!(PGSHIFT, 0)) as u32 | (asid & (NASID - 1)));
    }
}

fn passive_alloc(va: usize, pgdir: *mut Pde, asid: u32) {
    unsafe {
        if PAGES.is_null() || PAGE_FREE_LIST.is_null() {
            panic!("Tlb cannot access to the global vars!")
        }
    }
    if va < UTEMP {
        panic!("Address too low");
    }

    if (USTACKTOP..USTACKTOP + PAGE_SIZE).contains(&va) {
        panic!("Invalid memory");
    }

    if (UENVS..UPAGES).contains(&va) {
        panic!("Envs zero");
    }

    if (UPAGES..UVPT).contains(&va) {
        panic!("Pages zero");
    }

    if va >= ULIM {
        panic!("Kernel address");
    }
    unsafe {
        let pages = PAGES;
        let pp = page_alloc(&mut *PAGE_FREE_LIST, &pages).unwrap();
        page_insert(
            pgdir,
            PTE_ADDR!(va),
            asid,
            if (UVPT..ULIM).contains(&va) { 0 } else { PTE_D },
            pp,
            &mut *PAGE_FREE_LIST,
            &pages,
        )
        .unwrap();
    }
}

#[no_mangle]
pub fn _do_tlb_refill(pentrylo: &mut [u32; 2], va: usize, asid: u32) {
    tlb_invalidate(asid, va);

    unsafe {
        let pages = PAGES;
        loop {
            match page_lookup(CUR_PGDIR, va, &mut *PAGE_FREE_LIST, &pages) {
                None => passive_alloc(va, CUR_PGDIR, asid),
                Some((_, ppte)) => {
                    let ppte = (ppte as u32 & !0x7) as *mut Pte;
                    pentrylo[0] = *ppte >> 6;
                    pentrylo[1] = *((ppte as usize + 4) as *mut Pte) >> 6;
                    break;
                }
            }
        }
    }
}

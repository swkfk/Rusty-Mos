use crate::{
    kdef::mmu::{NASID, PAGE_SIZE, PGSHIFT, PTE_D, UENVS, ULIM, UPAGES, USTACKTOP, UTEMP, UVPT},
    GEN_MASK, PTE_ADDR,
};

use crate::kern::pmap::{page_alloc, page_insert, page_lookup, Pde, Pte, CUR_PGDIR};

extern "C" {
    pub fn tlb_out(entryhi: u32);
}

pub fn tlb_invalidate(asid: u32, va: usize) {
    unsafe {
        tlb_out((va & !GEN_MASK!(PGSHIFT, 0)) as u32 | (asid & (NASID - 1)));
    }
}

fn passive_alloc(va: usize, pgdir: *mut Pde, asid: u32) {
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
        let pp = page_alloc().unwrap();
        page_insert(
            pgdir,
            PTE_ADDR!(va),
            asid,
            if (UVPT..ULIM).contains(&va) { 0 } else { PTE_D },
            pp,
        )
        .unwrap();
    }
}

#[no_mangle]
pub fn _do_tlb_refill(pentrylo: &mut [u32; 2], va: usize, asid: u32) {
    tlb_invalidate(asid, va);

    unsafe {
        loop {
            match page_lookup(CUR_PGDIR, va) {
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

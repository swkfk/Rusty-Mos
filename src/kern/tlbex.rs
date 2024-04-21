use crate::{
    kdef::mmu::{NASID, PGSHIFT},
    GEN_MASK,
};

extern "C" {
    pub fn tlb_out(entryhi: u32);
}

pub fn tlb_invalidate(asid: u32, va: usize) {
    unsafe {
        tlb_out((va & !GEN_MASK!(PGSHIFT, 0)) as u32 | (asid & (NASID - 1)));
    }
}

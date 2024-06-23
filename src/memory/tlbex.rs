use crate::{
    memory::regions::{
        NASID, PAGE_SIZE, PGSHIFT, PTE_D, UENVS, ULIM, UPAGES, USTACKTOP, UTEMP, UVPT, UXSTACKTOP,
    },
    process::envs::{CUR_ENV_IDX, ENVS_DATA},
    GEN_MASK, PTE_ADDR,
};
use core::mem::{size_of, size_of_val};
use core::sync::atomic::Ordering::SeqCst;

use crate::memory::pmap::{page_alloc, page_insert, page_lookup, Pde, Pte, CUR_PGDIR};

use crate::arch_mipsel::trap::TrapFrame;

extern "C" {
    pub fn tlb_out(entryhi: u32);
}

pub fn tlb_invalidate(asid: u32, va: usize) {
    let entryhi = (va & !GEN_MASK!(PGSHIFT, 0)) as u32 | (asid & (NASID - 1));
    unsafe {
        tlb_out(entryhi);
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

#[no_mangle]
pub fn _do_tlb_refill(pentrylo: &mut [u32; 2], va: usize, asid: u32) {
    tlb_invalidate(asid, va);

    loop {
        let cur_pgdir = *CUR_PGDIR.borrow();
        match page_lookup(cur_pgdir, va) {
            None => passive_alloc(va, cur_pgdir, asid),
            Some((_, ppte)) => {
                let ppte = (ppte as u32 & !0x7) as *mut Pte;

                pentrylo[0] = unsafe { *ppte } >> 6;
                pentrylo[1] = unsafe { *ppte.wrapping_add(1) } >> 6;

                break;
            }
        }
    }
}

/// # Safety
///
#[no_mangle]
pub fn do_tlb_mod(trapframe: *mut TrapFrame) {
    let tf = trapframe; // deceits
    let stored_trapframe = unsafe { *tf };
    let mut modified_trapframe = unsafe { *tf };
    if !(USTACKTOP..UXSTACKTOP).contains(&(modified_trapframe.regs[29] as usize)) {
        modified_trapframe.regs[29] = UXSTACKTOP as u32;
    }
    modified_trapframe.regs[29] -= size_of::<TrapFrame>() as u32;
    let target = modified_trapframe.regs[29] as *mut TrapFrame;
    unsafe { target.write(stored_trapframe) };
    // Pte was ignored in the C-Edition Mos
    let _ = page_lookup(
        *CUR_PGDIR.borrow(),
        modified_trapframe.cp0_badvaddr as usize,
    );
    let tlb_entry = ENVS_DATA.borrow().0[CUR_ENV_IDX.load(SeqCst)].user_tlb_mod_entry;
    if tlb_entry != 0 {
        modified_trapframe.regs[4] = modified_trapframe.regs[29];
        modified_trapframe.regs[29] -= size_of_val(&modified_trapframe.regs[4]) as u32;
        modified_trapframe.cp0_epc = tlb_entry;
    } else {
        panic!("TLB Mod but no user handler registered")
    }
    unsafe { core::ptr::write_volatile(tf, modified_trapframe) }
}

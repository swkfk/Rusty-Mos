use core::mem::{size_of, size_of_val};

use crate::{
    kdef::mmu::{
        NASID, PAGE_SIZE, PGSHIFT, PTE_D, UENVS, ULIM, UPAGES, USTACKTOP, UTEMP, UVPT, UXSTACKTOP,
    },
    GEN_MASK, PTE_ADDR,
};

use crate::memory::pmap::{page_alloc, page_insert, page_lookup, Pde, Pte, CUR_PGDIR};

use crate::kern::{env::CUR_ENV, trap::TrapFrame};

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

    loop {
        let cur_pgdir = *CUR_PGDIR.borrow();
        match page_lookup(cur_pgdir, va) {
            None => passive_alloc(va, cur_pgdir, asid),
            Some((_, ppte)) => {
                let ppte = (ppte as u32 & !0x7) as *mut Pte;
                unsafe {
                    pentrylo[0] = *ppte >> 6;
                    pentrylo[1] = *((ppte as usize + 4) as *mut Pte) >> 6;
                }
                break;
            }
        }
    }
}

/// # Safety
///
#[no_mangle]
pub unsafe fn do_tlb_mod(trapframe: *mut TrapFrame) {
    let stored_trapframe = *trapframe;
    if !(USTACKTOP..UXSTACKTOP).contains(&((*trapframe).regs[29] as usize)) {
        (*trapframe).regs[29] = UXSTACKTOP as u32;
    }
    (*trapframe).regs[29] -= size_of::<TrapFrame>() as u32;
    ((*trapframe).regs[29] as *mut TrapFrame).write(stored_trapframe);
    // Pte was ignored in the C-Edition Mos
    let _ = page_lookup(*CUR_PGDIR.borrow(), (*trapframe).cp0_badvaddr as usize);
    if (*(*CUR_ENV).data).user_tlb_mod_entry != 0 {
        (*trapframe).regs[4] = (*trapframe).regs[29];
        (*trapframe).regs[29] -= size_of_val(&(*trapframe).regs[4]) as u32;
        (*trapframe).cp0_epc = (*(*CUR_ENV).data).user_tlb_mod_entry;
    } else {
        panic!("TLB Mod but no user handler registered")
    }
}

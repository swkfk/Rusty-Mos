use core::ptr;

use crate::kdef::mmu::KSEG1;

pub fn iowrite_from_va<T>(dst_paddr: usize, src_addr: usize) {
    unsafe {
        ptr::write_volatile(
            (dst_paddr | KSEG1) as *mut T,
            ptr::read_volatile(src_addr as *const T),
        )
    };
}

pub fn ioread_into_va<T>(src_paddr: usize, dst_addr: usize) {
    unsafe {
        ptr::write_volatile(
            dst_addr as *mut T,
            ptr::read_volatile((src_paddr | KSEG1) as *const T),
        )
    }
}

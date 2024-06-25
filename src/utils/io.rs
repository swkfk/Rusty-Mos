//! Just read/write from the device address.
//!
//! We use the K-Seg memory traits to write or read to or from the device
//! address.

use core::ptr;

use crate::memory::regions::KSEG1;

/// Write the content in `src_addr` to the `dst_paddr`.
pub fn iowrite_from_va<T>(dst_paddr: usize, src_addr: usize) {
    unsafe {
        ptr::write_volatile(
            (dst_paddr | KSEG1) as *mut T,
            ptr::read_volatile(src_addr as *const T),
        )
    };
}

/// Read the content from `src_paddr` and write it to the `dst_addr`.
pub fn ioread_into_va<T>(src_paddr: usize, dst_addr: usize) {
    unsafe {
        ptr::write_volatile(
            dst_addr as *mut T,
            ptr::read_volatile((src_paddr | KSEG1) as *const T),
        )
    }
}

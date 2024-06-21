//! Definitions of the conversion macros for the pages and memory.

/// Get the page number through the page object.
#[macro_export]
macro_rules! page2ppn {
    ($pp:expr, $pages:expr; $t:ty) => {{
        (($pp as usize - $pages as usize) / core::mem::size_of::<$t>()) as usize
    }};
}

/// Get the physical address of the page object.
///
/// See also: [pa2page](crate::pa2page)
#[macro_export]
macro_rules! page2pa {
    ($pp:expr, $pages:expr; $t:ty) => {{
        $crate::page2ppn!($pp, $pages; $t) << $crate::kdef::mmu::PGSHIFT
    }};
}

/// Get the kernel virtual address of the page object.
#[macro_export]
macro_rules! page2kva {
    ($pp:expr, $pages:expr; $t:ty) => {{
        $crate::KADDR!($crate::page2pa!($pp, $pages; $t))
    }};
}

/// Get the page object of the phisical address.
///
/// See also: [page2pa](crate::page2pa)
#[macro_export]
macro_rules! pa2page {
    ($pa:expr, $pages:expr; $t:ty) => {{
        let ppn = $crate::PPN!($pa) as usize;
        // assert!(ppn >= npage);
        $pages as usize + ppn * core::mem::size_of::<$t>()
    }};
}

/// Get the physical address of the given virtual address. This macro will
/// look up the page table to do the transmition.
///
/// `-1` or `0xffffffff` will be spawned if the address is not found or
/// the page table entry is invalid.
///
/// **ATTENTION**! The address spawned will ignore the in-page offset.
#[macro_export]
macro_rules! va2pa {
    ($pgdir:expr, $va:expr) => {{
        let pgdir = $crate::ARRAY_PTR!($pgdir; $crate::PDX!($va), $crate::kern::pmap::Pde);
        if 0 == (*pgdir & $crate::kdef::mmu::PTE_V) {
            !0
        } else {
            let p = $crate::KADDR!($crate::PTE_ADDR!(*pgdir)) as *mut $crate::kern::pmap::Pte;
            if 0 == (*$crate::ARRAY_PTR!(p; $crate::PTX!($va), $crate::kern::pmap::Pte) & $crate::kdef::mmu::PTE_V) {
                !0
            } else {
                $crate::PTE_ADDR!(*$crate::ARRAY_PTR!(p; $crate::PTX!($va), $crate::kern::pmap::Pte) as u32) as usize
            }
        }
    }};
}

/// Get the physical address of the virtual address in **kernel segment**
///
/// Someway the opposite of [KADDR](crate::KADDR)
///
/// # Panic
/// Assertion will fail if the virtual address is not in the **kernel
/// segment**.
#[macro_export]
macro_rules! PADDR {
    ($x: expr) => {{
        assert!($x >= 0x80000000);
        $x - 0x80000000
    }};
}

/// Get the virtual address (in **kernel segment**) from the physical address
///
/// Someway the opposite of [PADDR](crate::PADDR)
///
/// # Panic
/// **NOT IMPLEMENTED**
/// Assertion will fail if the virtual address is not in the **kernel
/// segment**.
#[macro_export]
macro_rules! KADDR {
    ($x: expr) => {{
        // assert!(($x >> 12) < npage);
        $x + 0x80000000
    }};
}

/// Get the address(or the frame number etc.) from the page table entry (PTE)
#[macro_export]
macro_rules! PTE_ADDR {
    ($pte: expr) => {{
        $pte & !0xFFF
    }};
}

/// Get the **Page Directory Offset** from the virtual address
///
/// See also: [PTX](crate::PTX)
#[macro_export]
macro_rules! PDX {
    ($va: expr) => {{
        ($va >> $crate::kdef::mmu::PDSHIFT) & 0x03FF
    }};
}

/// Get the **Page Table Offset** from the virtual address
///
/// See also: [PDX](crate::PDX)
#[macro_export]
macro_rules! PTX {
    ($va: expr) => {{
        ($va >> $crate::kdef::mmu::PGSHIFT) & 0x03FF
    }};
}

/// Get the Page Number from the physical address
#[macro_export]
macro_rules! PPN {
    ($pa: expr) => {
        $pa >> $crate::kdef::mmu::PGSHIFT
    };
}

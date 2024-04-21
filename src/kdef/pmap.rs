//! Definitions of the conversion macros for the page

/// Get the page number through the page object
#[macro_export]
macro_rules! page2ppn {
    ($pp:expr, $pages:expr; $t:ty) => {{
        (($pp as usize - $pages as usize) / core::mem::size_of::<$t>()) as usize
    }};
}

/// Get the physical address of the page object
///
/// See also: [pa2page](crate::pa2page)
#[macro_export]
macro_rules! page2pa {
    ($pp:expr, $pages:expr; $t:ty) => {{
        $crate::page2ppn!($pp, $pages; $t) << $crate::kdef::mmu::PGSHIFT
    }};
}

/// Get the kernel virtual address of the page object
#[macro_export]
macro_rules! page2kva {
    ($pp:expr, $pages:expr; $t:ty) => {{
        $crate::KADDR!($crate::page2pa!($pp, $pages; $t))
    }};
}

/// Get the page object of the phisical address
///
/// See also: [page2pa](crate::page2pa)
#[macro_export]
macro_rules! pa2page {
    ($pa:expr, $pages:expr; $t:ty) => {{
        let ppn = $crate::PPN!($pa) as usize;
        // assert!(ppn >= npage);
        $pages as usize + ppn * size_of::<$t>()
    }};
}

#[macro_export]
macro_rules! page2ppn {
    ($pp:expr, $pages:expr; $t:ty) => {{
        (($pp as usize - $pages as usize) / size_of::<$t>()) as usize
    }};
}

#[macro_export]
macro_rules! page2pa {
    ($pp:expr, $pages:expr; $t:ty) => {{
        $crate::page2ppn!($pp, $pages; $t) << PAGE_SHIFT
    }};
}

#[macro_export]
macro_rules! page2kva {
    ($pp:expr, $pages:expr; $t:ty) => {{
        $crate::KADDR!($crate::page2pa!($pp, $pages; $t))
    }};
}

#[macro_export]
macro_rules! pa2page {
    ($pa:expr, $pages:expr; $t:ty) => {{
        let ppn = $crate::PPN!($pa) as usize;
        // assert!(ppn >= npage);
        $pages as usize + ppn * size_of::<$t>()
    }};
}

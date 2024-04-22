mod test_memory;

#[macro_export]
macro_rules! CALL_TEST {
    ($func: ident; ($($args:expr),*)) => {
        $crate::ktests::$func($($args,)*);
    };
}

macro_rules! MAKE_TEST {
    ($ktest: expr, $func: ident; $field:tt :: $item:tt; ($($args:ident : $ty:ty),*)) => {
        #[cfg(ktest_item = $ktest)]
        pub fn $func($($args: $ty,)*) {
            crate::debugln!("$ Test {}({}) Begin!", $ktest, core::stringify!($item));
            $field::$item($($args,)*);
            crate::debugln!("$ Test {}({}) Passed!", $ktest, core::stringify!($item));
        }
        #[cfg(not(ktest_item = $ktest))]
        pub fn $func($($args: $ty,)*) {}
    };
}

MAKE_TEST!("memory", test_page; test_memory::test_page; (
    _page_free_list: &mut crate::kern::pmap::PageList,
    _pages: &mut *mut crate::kern::pmap::PageNode
));
MAKE_TEST!("memory", test_page_strong; test_memory::test_page_strong; (
    _page_free_list: &mut crate::kern::pmap::PageList,
    _pages: &mut *mut crate::kern::pmap::PageNode
));
MAKE_TEST!("memory", test_tlb_refill; test_memory::test_tlb_refill; (
    _page_free_list: &mut crate::kern::pmap::PageList,
    _pages: &mut *mut crate::kern::pmap::PageNode
));
MAKE_TEST!("memory", test_linklist; test_memory::test_linklist; ());

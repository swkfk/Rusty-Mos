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

MAKE_TEST!("memory", test_memory_normal; test_memory::test_physical_memory_manage; (_1: &mut u8));

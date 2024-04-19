mod test_demo;

#[cfg(ktest)]
macro_rules! MAKE_TEST {
    ($($field:tt :: $item:tt),*; $ktest: expr) => {
        if cfg!(ktest_item = $ktest) {
            $(
                debugln!("$ Test {}({}) Begin!", $ktest, stringify!($item));
                $field::$item();
            )*
            debugln!("$ Test {} Passed!", $ktest);
        }
    };
}

#[cfg(ktest)]
pub fn test() {
    use crate::debugln;
    use core::stringify;

    MAKE_TEST!(test_demo::test_hello_world, test_demo::test_hello_world; "hello");
}

#[cfg(not(ktest))]
pub fn test() {}

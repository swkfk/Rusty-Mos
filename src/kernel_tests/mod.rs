use crate::println;

mod test_array_link_list;
mod test_buddy_alloc;
mod test_memory_pool;

pub(crate) fn slash_print(s: &str) {
    println!("===== \x1b[33m{}\x1b[0m =====", s);
}

macro_rules! TEST_FN {
    ($e:block) => {
        pub(crate) fn test()
            $e
    };
}

macro_rules! TEST_CALL {
    ($m:tt) => {
        slash_print(stringify!($m));
        $m::test();
    };
}

pub(crate) use TEST_FN;

pub fn unit_test() {
    TEST_CALL!(test_buddy_alloc);
    TEST_CALL!(test_array_link_list);
    TEST_CALL!(test_memory_pool);
}

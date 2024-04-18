mod test_demo;

#[cfg(ktest)]
pub fn test() {
    use crate::debugln;

    debugln!("> ktests:mod.rs: Test Begin!");
    if cfg!(ktest_item = "hello") {
        test_demo::test_hello_world();
    }
}

#[cfg(not(ktest))]
pub fn test() {}

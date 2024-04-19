#[cfg(ktest_item = "memory")]
pub fn test_physical_memory_manage(num: &mut u8) {
    *num += 1;
    assert_eq!(*num, 2);
}

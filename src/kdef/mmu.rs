#[macro_export]
macro_rules! PADDR {
    ($x: expr) => {{
        assert!($x >= 0x80000000);
        $x - 0x80000000
    }};
}

#[macro_export]
macro_rules! KADDR {
    ($x: expr) => {{
        // assert!(($x >> 12) < npage);
        $x + 0x80000000
    }};
}

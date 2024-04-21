#[macro_export]
macro_rules! GEN_MASK {
    ($h: expr, $l: expr) => {{
        (0xffffffff << $l) & (0xffffffff >> (31 - $h))
    }};
}

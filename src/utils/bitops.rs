//! Bit operations.

/// Gen a mask from the $h-bit (MSB) to $l-bit (LSB) with 1.
///
/// # Examples
///
/// ```
/// assert_eq!(GEN_MASK(30, 21), 0x7fe00000);
/// ```
#[macro_export]
macro_rules! GEN_MASK {
    ($h: expr, $l: expr) => {{
        (0xffffffff << $l) & (0xffffffff >> (31 - $h))
    }};
}

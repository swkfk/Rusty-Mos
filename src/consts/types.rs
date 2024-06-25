//! Definition of some misc macros about type and offset.

/// Round-up to the specified fractor.
///
/// **REQUIREMENT**: `$n` is **the power of 2**, such 4, 8, 16, etc.
#[macro_export]
macro_rules! ROUND {
    ($x: expr; $n: expr) => {
        ($x + $n - 1) & !($n - 1)
    };
}

/// Round-down to the specified fractor.
///
/// /// **REQUIREMENT**: `$n` is **the power of 2**, such 4, 8, 16, etc.
#[macro_export]
macro_rules! ROUNDDOWN {
    ($x: expr; $n: expr) => {
        $x & !($n - 1)
    };
}

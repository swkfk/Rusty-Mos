//! Definition of some misc macros about type and offset

/// Round-up to the specified fractor
///
/// **REQUIREMENT**: `$n` is **the power of 2**, such 4, 8, 16, etc.
#[macro_export]
macro_rules! ROUND {
    ($x: expr; $n: expr) => {
        ($x + $n - 1) & !($n - 1)
    };
}

#[macro_export]
macro_rules! ROUNDDOWN {
    ($x: expr; $n: expr) => {
        $x & !($n - 1)
    };
}

/// Get the array\[i\] via the raw pointer
///
/// The `$i` is the index of the target element, starting from **zero**.
/// The `$t` is the array elements' type.
#[macro_export]
macro_rules! ARRAY_PTR {
    ($array: expr; $i: expr, $t: ty) => {
        (($array as usize) + $i * core::mem::size_of::<$t>()) as *mut $t
    };
}

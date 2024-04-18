#[macro_export]
macro_rules! ROUND {
    ($x: expr; $n: expr) => {
        ($x + $n - 1) & !($n - 1)
    };
}

#[macro_export]
macro_rules! ARRAY_PTR {
    ($array: expr; $i: expr, $t: ty) => {
        (($array as usize) + $i * size_of::<$t>()) as *mut $t
    };
}

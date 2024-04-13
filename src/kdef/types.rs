#[macro_export]
macro_rules! ROUND {
    ($x: expr; $n: expr) => {
        ($x + $n - 1) & !($n - 1)
    };
}

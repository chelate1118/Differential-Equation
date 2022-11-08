#[macro_export]
macro_rules! assert_feq {
    ($x: expr, $y: expr) => { assert!((($x) - ($y)).abs() < 1e-10) }
}
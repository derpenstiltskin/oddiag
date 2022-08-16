#[macro_export]
macro_rules! is_bit_set {
    ($a:expr, $b:expr) => {
        if ($a & $b) != 0 {
            true
        } else {
            false
        }
    }
}
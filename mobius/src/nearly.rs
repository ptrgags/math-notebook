pub const EPSILON: f64 = 1e-15;

pub fn is_nearly(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}
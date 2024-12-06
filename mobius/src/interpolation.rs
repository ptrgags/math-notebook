use crate::Complex;

pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    (1.0 - t) * a + t * b
}

pub fn lerp_complex(a: Complex, b: Complex, t: f64) -> Complex {
    a * (1.0 - t).into() + b * t.into()
}

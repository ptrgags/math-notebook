use crate::Complex;

use super::Geometry;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Line {
    pub unit_normal: Complex,
    pub distance: f64,
}

impl Line {
    /// Create a line from a normal and distance offset
    /// This will automatically normalize the normal
    pub fn new(normal: Complex, distance: f64) -> Result<Self, String> {
        match normal.normalize() {
            Some(unit_normal) => Ok(Self {
                unit_normal,
                distance,
            }),
            None => Err(String::from("Normal must be finite and non-zero")),
        }
    }

    pub fn real_axis() -> Self {
        Self {
            unit_normal: Complex::I,
            distance: 0.0,
        }
    }

    pub fn imag_axis() -> Self {
        Self {
            unit_normal: Complex::ONE,
            distance: 0.0,
        }
    }
}

impl Geometry for Line {}

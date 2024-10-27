use std::fmt::Display;

use crate::Complex;

use super::Geometry;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Circle {
    pub center: Complex,
    pub radius: f64,
}

impl Circle {
    pub fn unit_circle() -> Self {
        Self {
            center: Complex::Zero,
            radius: 1.0,
        }
    }

    pub fn new(center: Complex, radius: f64) -> Self {
        Self { center, radius }
    }

    pub fn get_point(&self, theta: f64) -> Complex {
        self.center + Complex::from_polar(self.radius, theta)
    }

    pub fn get_angle(&self, point: Complex) -> Option<f64> {
        (point - self.center).arg()
    }
}

impl Geometry for Circle {}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle({}, {:.3})", self.center, self.radius)
    }
}

use std::fmt::Display;

use rendering::{RenderPrimitive, Renderable};

use crate::{nearly::is_nearly, Complex};

use super::Geometry;

#[derive(Clone, Copy, Debug)]
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

    pub fn point_inside(&self, point: Complex) -> bool {
        (point - self.center).norm() <= self.radius * self.radius
    }
}

impl Renderable for Circle {
    fn render(&self) -> Result<RenderPrimitive, Box<dyn std::error::Error>> {
        Ok(RenderPrimitive::Circle {
            x: self.center.real(),
            y: self.center.imag(),
            radius: self.radius,
        })
    }
}

impl PartialEq for Circle {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center && is_nearly(self.radius, other.radius)
    }
}

impl Geometry for Circle {}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle({}, {:.3})", self.center, self.radius)
    }
}

use crate::{complex_error::ComplexError, unit_complex::UnitComplex, Complex};

use super::{ray::Ray, DirectedEdge, Geometry};

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct DoubleRay(pub Ray, pub Ray);

impl DoubleRay {
    pub fn from_points(a: Complex, b: Complex) -> Result<Self, ComplexError> {
        ComplexError::require_finite("a", a)?;
        ComplexError::require_finite("b", b)?;

        let ab = UnitComplex::normalize(b - a)?;
        let ray_a = Ray::new(a, -ab)?;
        let ray_b = Ray::new(b, ab)?;
        Ok(Self(ray_a, ray_b))
    }

    pub fn reverse(&self) -> Self {
        let &Self(ray_a, ray_b) = self;
        Self(ray_b, ray_a)
    }
}

impl Geometry for DoubleRay {}
impl DirectedEdge for DoubleRay {
    fn start(&self) -> Complex {
        let Self(a, _) = self;
        a.start
    }

    fn end(&self) -> Complex {
        let Self(_, b) = self;
        b.start
    }
}

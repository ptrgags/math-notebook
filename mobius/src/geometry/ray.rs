use rendering::RenderPrimitive;

use crate::{complex_error::ComplexError, unit_complex::UnitComplex, Complex};

use super::{DirectedEdge, Geometry};

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Ray {
    pub start: Complex,
    pub unit_dir: UnitComplex,
}

impl Ray {
    pub fn new(start: Complex, unit_dir: UnitComplex) -> Result<Self, ComplexError> {
        ComplexError::require_finite("start", start)?;
        Ok(Self { start, unit_dir })
    }

    pub fn to_primitive(&self) -> RenderPrimitive {
        const FAR_AWAY: f64 = 10000.0;
        let &Ray { start, unit_dir } = self;
        let end = *unit_dir.get() * FAR_AWAY.into();

        RenderPrimitive::LineSegment {
            x1: start.real(),
            y1: start.imag(),
            x2: end.real(),
            y2: end.imag(),
        }
    }
}

impl Geometry for Ray {}
impl DirectedEdge for Ray {
    fn start(&self) -> Complex {
        self.start
    }

    fn end(&self) -> Complex {
        Complex::Infinity
    }
}

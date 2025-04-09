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

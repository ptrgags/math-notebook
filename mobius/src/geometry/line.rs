use std::fmt::Display;

use thiserror::Error;

use crate::{
    complex_error::ComplexError, float_error::FloatError, nearly::is_nearly,
    unit_complex::UnitComplex, Complex,
};

use super::{Geometry, LineSegment};

#[derive(Debug, Error)]
pub enum LineError {
    #[error("{0}")]
    InvalidComplexParam(#[from] ComplexError),
    #[error("{0}")]
    InvalidFloatParam(#[from] FloatError),
    #[error("a and b must be distinct points: {0}")]
    DuplicatePoints(Complex),
}

#[derive(Clone, Copy, Debug)]
pub struct Line {
    pub unit_normal: UnitComplex,
    pub distance: f64,
}

impl Line {
    pub fn to_primitive(&self) -> RenderPrimitive {
        const FAR_AWAY: f64 = 10000.0;
        let far_away: Complex = FAR_AWAY.into();
        let tangent = self.unit_normal.rot90();
        let center: Complex = self.unit_normal * self.distance;
        let start: Complex = center + tangent * far_away;
        let end: Complex = center - tangent * far_away;

        RenderPrimitive::LineSegment {
            x1: start.real(),
            y1: start.imag(),
            x2: start.real(),
            y2: start.imag(),
        }
    }

    /// Create a line with the given unit normal and distance
    pub fn new(unit_normal: UnitComplex, distance: f64) -> Result<Self, LineError> {
        FloatError::require_finite("distance", distance)?;
        Ok(Self {
            unit_normal,
            distance,
        })
    }

    /// Compute a line through a and b
    pub fn from_points(a: Complex, b: Complex) -> Result<Self, LineError> {
        ComplexError::require_finite("a", a)?;
        ComplexError::require_finite("b", b)?;

        if a == b {
            return Err(LineError::DuplicatePoints(a));
        }

        // The checks above mean this vector is nonzero and finite, so
        // this operation will always work.
        let unit_tangent = UnitComplex::normalize(b - a)?;
        let unit_normal = unit_tangent.rot90();

        // Distance along the normal direction
        let distance = Complex::dot(a, *unit_normal.get());

        Ok(Line {
            unit_normal,
            distance,
        })
    }

    pub fn real_axis() -> Self {
        Self {
            unit_normal: UnitComplex::I,
            distance: 0.0,
        }
    }

    pub fn imag_axis() -> Self {
        Self {
            unit_normal: UnitComplex::ONE,
            distance: 0.0,
        }
    }
}

impl From<LineSegment> for Line {
    fn from(value: LineSegment) -> Self {
        let LineSegment { start, end } = value;

        Self::from_points(start, end).unwrap()
    }
}

impl Geometry for Line {}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            unit_normal,
            distance,
        } = self;
        write!(f, "Line(n={}, d={})", unit_normal, distance)
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        // Lines are only unique up to a scalar multiple. Since we require
        // the normal to have unit length, the scalar could be either -1 or 1
        // so check both.
        (self.unit_normal == other.unit_normal && is_nearly(self.distance, other.distance))
            || (self.unit_normal == -other.unit_normal && is_nearly(self.distance, -other.distance))
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    use test_case::test_case;

    #[test_case(Complex::new(1.0, 1.0), Complex::new(5.0, 1.0), Line::new(UnitComplex::I, 1.0).unwrap(); "horizontal_line")]
    pub fn from_points_with_valid_points_computes_correct_line(
        a: Complex,
        b: Complex,
        expected: Line,
    ) {
        let result = Line::from_points(a, b);

        assert!(result.is_ok_and(|x| x == expected));
    }

    #[test]
    pub fn missing_tests() {
        todo!("test new, more line cases, invalid lines, distance value");
    }
}

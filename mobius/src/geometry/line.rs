use std::fmt::Display;

use thiserror::Error;

use crate::{complex_error::ComplexError, float_error::FloatError, nearly::is_nearly, Complex};

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
    pub unit_normal: Complex,
    pub distance: f64,
}

impl Line {
    /// Create a line from a normal and distance offset
    /// This will automatically normalize the normal
    pub fn new(normal: Complex, distance: f64) -> Result<Self, LineError> {
        ComplexError::require_finite_nonzero("normal", normal)?;
        FloatError::require_finite("distance", distance)?;

        let magnitude = normal.mag();
        let distance = 1.0 / magnitude;

        let unit_normal = normal * distance.into();

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
        let unit_tangent = (b - a).normalize().unwrap();

        // In 2D, we just rotate the tangent to get the normal
        let unit_normal = Complex::I * unit_tangent;
        // Distance along the
        let distance = Complex::dot(a, unit_normal);

        Ok(Line {
            unit_normal,
            distance,
        })
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

    #[test]
    pub fn new_normalizes_parameters_correctly() {
        let not_normalized = Complex::new(2.0, 2.0);
        let distance = 1.0;

        let result = Line::new(not_normalized, distance).unwrap();

        // |2 + 2i| = 2 sqrt(2)
        // The normalized form will be 1/sqrt(2)(1 + i)
        // the distance value also needs to be divided by this magnitude,
        // so it's 1 / (2 sqrt(2))
        let normalized_component = 1.0 / (2.0f64).sqrt();
        let distance = normalized_component / 2.0;
        let expected = Line {
            unit_normal: Complex::new(normalized_component, normalized_component),
            distance,
        };
        assert_eq!(result, expected);
    }

    #[test_case(Complex::new(1.0, 1.0), Complex::new(5.0, 1.0), Line::new(Complex::I, 1.0).unwrap(); "horizontal_line")]
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

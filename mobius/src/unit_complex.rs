use std::ops::Neg;

use crate::{complex_error::ComplexError, Complex};

/// A complex number restricted so |z| = 1
#[derive(PartialEq, Clone, Copy, Debug, derive_more::Display)]
pub struct UnitComplex(Complex);

impl UnitComplex {
    pub const I: Self = Self(Complex::I);
    pub const ONE: Self = Self(Complex::ONE);

    /// Constructor that takes a finite, nonzero complex number and
    /// normalizes it so it has magnitude 1.
    pub fn normalize(z: Complex) -> Result<Self, ComplexError> {
        ComplexError::require_finite_nonzero("z", z)?;
        let a = z.real();
        let b = z.imag();
        let r = z.mag();
        Ok(Self(Complex::Finite(a / r, b / r)))
    }

    /// A unit complex number can be determined uniquely by an angle
    pub fn from_angle(theta: f64) -> Self {
        Self(Complex::from_polar(1.0, theta))
    }

    /// Rotate the complex number a quarter turn counterclockwise. This
    /// is used to compute a normal from a tangent
    pub fn rot90(&self) -> Self {
        let &Self(z) = self;
        let a = z.real();
        let b = z.imag();

        Self(Complex::new(-b, a))
    }

    /// Get the underlying complex number
    pub fn get(&self) -> &Complex {
        &self.0
    }
}

impl Neg for UnitComplex {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use super::*;

    #[test]
    pub fn normalize_with_zero_returns_error() {
        let result = UnitComplex::normalize(Complex::Zero);

        assert!(matches!(result, Err(ComplexError::NotFiniteNonzero(_, _))))
    }

    #[test]
    pub fn normalize_with_infinity_returns_error() {
        let result = UnitComplex::normalize(Complex::Infinity);

        assert!(matches!(result, Err(ComplexError::NotFiniteNonzero(_, _))))
    }

    #[test]
    pub fn normalize_with_valid_complex_normalizes_result() -> Result<(), ComplexError> {
        let result = UnitComplex::normalize(Complex::new(3.0, 4.0))?;

        let expected = UnitComplex::normalize(Complex::new(3.0 / 5.0, 4.0 / 5.0))?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    pub fn from_angle_computes_correct_direction() -> Result<(), ComplexError> {
        let result = UnitComplex::from_angle(4.0 * PI / 6.0);

        // cos(4pi/6) = -1/2
        // sin(4pi/6) = sqrt(3)/2
        let z = Complex::new(-0.5, 0.5 * (3.0f64).sqrt());
        let expected = UnitComplex::normalize(z)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    pub fn rot90_rotates_vector_ccw() {
        let n = UnitComplex::from_angle(PI / 4.0);

        let result = n.rot90();

        let expected = UnitComplex::from_angle(3.0 * PI / 4.0);
        assert_eq!(result, expected);
    }

    #[test]
    pub fn rot90_has_order_4() {
        let n = UnitComplex::from_angle(PI / 3.0);

        let result = n.rot90().rot90().rot90().rot90();

        assert_eq!(result, n);
    }

    #[test]
    pub fn neg_negates_components() {
        let n = UnitComplex::from_angle(PI / 6.0);

        let result = -n;

        let expected = UnitComplex::from_angle(7.0 * PI / 6.0);
        assert_eq!(result, expected);
    }

    #[test]
    pub fn rot90_twice_same_as_neg() {
        let n = UnitComplex::from_angle(PI / 6.0);

        let rot180 = n.rot90().rot90();
        let neg = -n;

        assert_eq!(rot180, neg);
    }
}

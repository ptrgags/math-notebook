use std::ops::Neg;

use crate::{complex_error::ComplexError, Complex};

#[derive(PartialEq, Clone, Copy, Debug, derive_more::Display)]
pub struct UnitComplex(Complex);

impl UnitComplex {
    pub const I: Self = Self(Complex::I);
    pub const ONE: Self = Self(Complex::ONE);

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

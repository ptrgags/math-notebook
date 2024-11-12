use thiserror::Error;

use crate::Complex;

#[derive(Debug, Error)]
pub enum ComplexError {
    #[error("value must be finite: {0} = {1}")]
    NotFinite(String, Complex),
    #[error("value must be finite and non-zero: {0} = {1}")]
    NotFiniteNonzero(String, Complex),
}

impl ComplexError {
    /// Require a complex number to be 0 or finite, but not Infinity
    pub fn require_finite(label: &str, x: Complex) -> Result<(), Self> {
        match x {
            Complex::Zero => Ok(()),
            Complex::Finite(_, _) => Ok(()),
            Complex::Infinity => Err(Self::NotFinite(String::from(label), x)),
        }
    }

    /// Require a complex number to be neither 0 nor infinity. For example,
    /// in order for a vector to be normalized, it can't be zero or infinity.
    pub fn require_finite_nonzero(label: &str, x: Complex) -> Result<(), Self> {
        match x {
            Complex::Finite(_, _) => Ok(()),
            _ => Err(Self::NotFiniteNonzero(String::from(label), x)),
        }
    }
}

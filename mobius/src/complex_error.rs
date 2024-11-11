use std::{error::Error, fmt::Display};

use crate::Complex;

#[derive(Debug)]
pub enum ComplexError {
    NotFinite(String, Complex),
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

impl Display for ComplexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFinite(var, val) => write!(f, "value must be finite: {} = {}", var, val),
            Self::NotFiniteNonzero(var, val) => {
                write!(f, "value must be finite and non-zero: {} = {}", var, val)
            }
        }
    }
}

impl Error for ComplexError {}

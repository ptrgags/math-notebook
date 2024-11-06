use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum FloatError {
    NonFinite(String, f64),
}

impl FloatError {
    /// Require a float value to be a finite value, not 0 or infinity
    pub fn require_finite(label: &str, x: f64) -> Result<(), Self> {
        if !x.is_finite() {
            Err(Self::NonFinite(String::from(label), x))
        } else {
            Ok(())
        }
    }
}

impl Display for FloatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonFinite(var, val) => write!(f, "value must be finite: {} = {}", var, val),
        }
    }
}

impl Error for FloatError {}

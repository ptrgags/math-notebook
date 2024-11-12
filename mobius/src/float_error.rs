use thiserror::Error;

#[derive(Debug, Error)]
pub enum FloatError {
    #[error("value must be finite: {0} = {1}")]
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

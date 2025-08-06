use crate::{multivector::Multivector, rational_poly::RationalPolynomial};

pub type SymbolicMultivector = Multivector<RationalPolynomial>;

impl SymbolicMultivector {
    pub fn even(symbol: &str) -> Self {
        Self::zero()
    }
    pub fn odd(symbol: &str) -> Self {
        Self::zero()
    }
}

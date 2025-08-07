use crate::{multivector::Multivector, rational_poly::RationalPolynomial};

pub type SymbolicMultivector<const P: u8, const N: u8, const Z: u8> =
    Multivector<P, N, Z, RationalPolynomial>;

impl<const P: u8, const N: u8, const Z: u8> SymbolicMultivector<P, N, Z> {
    pub fn even(symbol: &str) -> Self {
        Self::zero()
    }
    pub fn odd(symbol: &str) -> Self {
        Self::zero()
    }
}

pub type SymVGA2 = SymbolicMultivector<2, 0, 0>;
pub type SymVGA3 = SymbolicMultivector<3, 0, 0>;
pub type SymPGA2 = SymbolicMultivector<2, 0, 1>;
pub type SymPGA3 = SymbolicMultivector<3, 0, 1>;
pub type SymCGA2 = SymbolicMultivector<3, 1, 0>;
pub type SymCGA3 = SymbolicMultivector<4, 1, 0>;

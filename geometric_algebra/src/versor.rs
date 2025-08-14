use std::{fmt::Display, ops::Mul};

use crate::{
    basis_blade::BasisBlade, field::Field, multivector::Multivector,
    rational_poly::RationalPolynomial, symbolic_multivector::SymbolicMultivector,
};

#[derive(Clone, PartialEq)]
pub enum Versor<const P: u8, const N: u8, const Z: u8, F: Field> {
    Even(Multivector<P, N, Z, F>),
    Odd(Multivector<P, N, Z, F>),
}

impl<const P: u8, const N: u8, const Z: u8, F: Field> Versor<P, N, Z, F> {
    /// Get all of the possible terms for this versor
    pub fn get_all_terms(&self) -> Vec<(BasisBlade, F)> {
        match self {
            Versor::Even(v) => v.get_all_even_terms(),
            Versor::Odd(v) => v.get_all_odd_terms(),
        }
    }
}

impl<const P: u8, const N: u8, const Z: u8, F: Field> Mul for Versor<P, N, Z, F> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        use Versor::*;
        match (self, rhs) {
            (Even(a), Even(b)) => Even(a * b),
            (Even(a), Odd(b)) => Odd(a * b),
            (Odd(a), Even(b)) => Odd(a * b),
            (Odd(a), Odd(b)) => Even(a * b),
        }
    }
}

impl<const P: u8, const N: u8, const Z: u8, F: Field + Display> Display for Versor<P, N, Z, F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Versor::Even(multivector) => multivector.fmt(f),
            Versor::Odd(multivector) => multivector.fmt(f),
        }
    }
}

pub type SymbolicVersor<const P: u8, const N: u8, const Z: u8> =
    Versor<P, N, Z, RationalPolynomial>;

impl<const P: u8, const N: u8, const Z: u8> SymbolicVersor<P, N, Z> {
    pub fn even(symbol: &str) -> Self {
        let multivector = SymbolicMultivector::even(symbol);
        Self::Even(multivector)
    }

    pub fn odd(symbol: &str) -> Self {
        let multivector = SymbolicMultivector::odd(symbol);
        Self::Odd(multivector)
    }

    pub fn generic_vector(symbol: &str) -> Self {
        let vector = SymbolicMultivector::generic_vector(symbol);
        Self::Odd(vector)
    }
}

pub type SymVersorVGA2 = SymbolicVersor<2, 0, 0>;
pub type SymVersorVGA3 = SymbolicVersor<3, 0, 0>;
pub type SymVersorPGA2 = SymbolicVersor<2, 0, 1>;
pub type SymVersorPGA3 = SymbolicVersor<3, 0, 1>;
pub type SymVersorCGA2 = SymbolicVersor<3, 1, 0>;
pub type SymVersorCGA3 = SymbolicVersor<4, 1, 0>;

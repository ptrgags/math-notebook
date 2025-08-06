use std::{fmt::Display, ops::Mul};

use crate::{
    field::Field, multivector::Multivector, rational_poly::RationalPolynomial,
    symbolic_multivector::SymbolicMultivector,
};

pub enum Versor<F: Field> {
    Even(Multivector<F>),
    Odd(Multivector<F>),
}

impl<F: Field> Mul for Versor<F> {
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

impl<F: Field> Display for Versor<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Versor::Even(multivector) => multivector.fmt(f),
            Versor::Odd(multivector) => multivector.fmt(f),
        }
    }
}

pub type SymbolicVersor = Versor<RationalPolynomial>;

impl SymbolicVersor {
    pub fn even(symbol: &str) -> Self {
        let multivector = SymbolicMultivector::even(symbol);
        Self::Even(multivector)
    }

    pub fn odd(symbol: &str) -> Self {
        let multivector = SymbolicMultivector::even(symbol);
        Self::Even(multivector)
    }
}

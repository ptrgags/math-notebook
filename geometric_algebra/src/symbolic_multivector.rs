use std::collections::HashMap;

use crate::{
    basis_blade::BasisBlade,
    format_basis_blade::{format_basis_blade, ScalarFormat},
    multivector::Multivector,
    polynomial::Polynomial,
    rational_poly::RationalPolynomial,
};

pub type SymbolicMultivector<const P: u8, const N: u8, const Z: u8> =
    Multivector<P, N, Z, RationalPolynomial>;

impl<const P: u8, const N: u8, const Z: u8> SymbolicMultivector<P, N, Z> {
    fn from_blades(symbol: &str, blades: &[BasisBlade]) -> Self {
        let mut terms = HashMap::new();

        for blade in blades.iter() {
            let blade_label = format_basis_blade::<P, N, Z>(blade, ScalarFormat::Variable);
            let coeff_label = format!("{}_{}", symbol, blade_label);

            terms.insert(
                *blade,
                RationalPolynomial::from(Polynomial::var(&coeff_label)),
            );
        }

        Self::new(terms)
    }

    pub fn even(symbol: &str) -> Self {
        Self::from_blades(symbol, &BasisBlade::get_even_blades(Self::dimension()))
    }

    pub fn odd(symbol: &str) -> Self {
        Self::from_blades(symbol, &BasisBlade::get_odd_blades(Self::dimension()))
    }

    pub fn generic_vector(symbol: &str) -> Self {
        Self::from_blades(
            symbol,
            &BasisBlade::get_blades_for_grade(Self::dimension(), 1),
        )
    }
}

pub type SymVGA2 = SymbolicMultivector<2, 0, 0>;
pub type SymVGA3 = SymbolicMultivector<3, 0, 0>;
pub type SymPGA2 = SymbolicMultivector<2, 0, 1>;
pub type SymPGA3 = SymbolicMultivector<3, 0, 1>;
pub type SymCGA2 = SymbolicMultivector<3, 1, 0>;
pub type SymCGA3 = SymbolicMultivector<4, 1, 0>;

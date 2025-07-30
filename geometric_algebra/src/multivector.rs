use std::{
    collections::HashMap,
    ops::{Add, Mul, Sub},
};

use crate::{basis_blade::BasisBlade, field::Field, real::Real};

#[derive(Debug, Clone)]
pub struct Multivector<F> {
    terms: HashMap<BasisBlade, F>,
}

impl<F: Field> Multivector<F> {
    /// the zero multivector. This is equivalent to the
    /// zero vector, bivector, etc.
    pub fn zero() -> Self {
        Self {
            terms: HashMap::new(),
        }
    }

    /// The multiplicative identity, 1.
    pub fn one() -> Self {
        Self::from(F::one())
    }

    pub fn new(terms: HashMap<BasisBlade, F>) -> Self {
        let zero = F::zero();
        let nonzero_terms = terms
            .into_iter()
            .filter(|(_, coeff)| *coeff != zero)
            .collect();

        Self {
            terms: nonzero_terms,
        }
    }

    /// A single basis vector e_index
    pub fn vector(index: u8) -> Self {
        Self::from(BasisBlade::vector(index))
    }

    // A bivector e_i ^ e_j
    pub fn bivector(i: u8, j: u8) -> Self {
        Self::from(BasisBlade::bivector(i, j))
    }

    // A trivector e_i ^ e_j ^ e_k
    pub fn trivector(i: u8, j: u8, k: u8) -> Self {
        Self::from(BasisBlade::trivector(i, j, k))
    }

    // A quadvector e_i ^ e_j ^ e_k ^ e_l
    pub fn quadvector(i: u8, j: u8, k: u8, l: u8) -> Self {
        Self::from(BasisBlade::quadvector(i, j, k, l))
    }

    // A pentavector e_i ^ e_j ^ e_k ^ e_l ^ e_m
    pub fn pentavector(i: u8, j: u8, k: u8, l: u8, m: u8) -> Self {
        Self::from(BasisBlade::pentavector(i, j, k, l, m))
    }
}

impl<F: Field> From<F> for Multivector<F> {
    fn from(value: F) -> Self {
        // If the value is 0, return the empty map
        if value == F::zero() {
            return Self::zero();
        }

        let mut terms = HashMap::new();
        terms.insert(BasisBlade::scalar(), value);

        Self { terms }
    }
}

/// Shorthand for creating a scalar
impl From<f64> for Multivector<Real> {
    fn from(value: f64) -> Self {
        Self::from(Real::from(value))
    }
}

impl<F: Field> From<BasisBlade> for Multivector<F> {
    fn from(value: BasisBlade) -> Self {
        let mut terms = HashMap::new();
        terms.insert(value, F::one());

        Self { terms }
    }
}

impl<F: Field> Add for Multivector<F> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut terms = self.terms.clone();

        for (blade, coeff) in rhs.terms.iter() {
            terms
                .entry(*blade)
                .and_modify(|e| *e = *e + *coeff)
                .or_insert(*coeff);
        }

        Self::new(terms)
    }
}

impl<F: Field> Sub for Multivector<F> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut terms = self.terms.clone();

        for (blade, coeff) in rhs.terms.iter() {
            terms
                .entry(*blade)
                .and_modify(|e| *e = *e - *coeff)
                .or_insert(*coeff);
        }

        Self::new(terms)
    }
}

impl<F: Field> Mul for Multivector<F> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut terms = HashMap::new();

        for (blade_a, coeff_a) in self.terms.iter() {
            for (blade_b, coeff_b) in rhs.terms.iter() {
                let key = blade_a.symmetric_diff(blade_b);
                let coeff = *coeff_a * *coeff_b;
                // Also need to account for negative signs from swapping
                // also need to account for negative signs from the signature

                terms
                    .entry(key)
                    .and_modify(|e| *e = *e * coeff)
                    .or_insert(coeff);
            }
        }

        Self::new(terms)
    }
}

impl<F: Field> PartialEq for Multivector<F> {
    fn eq(&self, other: &Self) -> bool {
        if self.terms.len() != other.terms.len() {
            return false;
        }

        for (blade, coeff) in self.terms.iter() {
            if other.terms[blade] != *coeff {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use crate::real::Real;

    use super::*;

    #[test]
    pub fn test_zero_is_additive_identity() {
        let x = Multivector::from(3.0);
        let zero = Multivector::zero();

        assert_eq!(zero.clone() + x.clone(), x);
        assert_eq!(x.clone() + zero, x);
    }

    #[test]
    pub fn test_element_sub_self_is_zero() {
        let x = Multivector::from(3.4);

        assert_eq!(x.clone() - x, Multivector::zero());
    }

    #[test]
    pub fn test_one_is_multiplicative_identity() {
        let one = Multivector::one();
        let x = Multivector::from(2.4);

        assert_eq!(one.clone() * x.clone(), x);
        assert_eq!(x.clone() * one, x);
    }

    #[test]
    pub fn test_scalar_multiplication_distributes() {
        let scalar = Multivector::from(2.0);
        let one = Multivector::one();
        let vector = Multivector::vector(0);
        let bivector = Multivector::bivector(2, 3);

        // 2(1 + x + zw)
        let sum = one.clone() + vector.clone() + bivector.clone();
        let scaled = scalar.clone() * sum;

        // is equivalent to 2 + 2x + zw
        let weighted_sum =
            scalar.clone() * one + scalar.clone() * vector + scalar.clone() * bivector;

        assert_eq!(scaled, weighted_sum);
    }

    #[test]
    pub fn test_multiply_two_vectors_gives_bivector() {
        let x: Multivector<Real> = Multivector::vector(0);
        let y: Multivector<Real> = Multivector::vector(1);

        let product = x * y;
        let xy = Multivector::bivector(0, 1);

        assert_eq!(product, xy);
    }
}

use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Add, Mul, Sub},
};

use crate::{basis_blade::BasisBlade, field::Field, real::Real};

#[derive(Debug, Clone)]
pub struct Multivector<const P: u8, const N: u8, const Z: u8, F> {
    terms: HashMap<BasisBlade, F>,
}

impl<const P: u8, const N: u8, const Z: u8, F: Field> Multivector<P, N, Z, F> {
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

impl<const P: u8, const N: u8, const Z: u8, F: Field> From<F> for Multivector<P, N, Z, F> {
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
impl<const P: u8, const N: u8, const Z: u8, F> From<f64> for Multivector<P, N, Z, F>
where
    F: Field + From<f64>,
{
    fn from(value: f64) -> Self {
        Self::from(F::from(value))
    }
}

impl<const P: u8, const N: u8, const Z: u8, F: Field> From<BasisBlade> for Multivector<P, N, Z, F> {
    fn from(value: BasisBlade) -> Self {
        let mut terms = HashMap::new();
        terms.insert(value, F::one());

        Self { terms }
    }
}

impl<const P: u8, const N: u8, const Z: u8, F: Field> Add for Multivector<P, N, Z, F> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut terms = self.terms.clone();

        for (blade, coeff) in rhs.terms.iter() {
            terms
                .entry(*blade)
                .and_modify(|e| *e = e.clone() + coeff.clone())
                .or_insert(coeff.clone());
        }

        Self::new(terms)
    }
}

impl<const P: u8, const N: u8, const Z: u8, F: Field> Sub for Multivector<P, N, Z, F> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut terms = self.terms.clone();

        for (blade, coeff) in rhs.terms.iter() {
            terms
                .entry(*blade)
                .and_modify(|e| *e = e.clone() - coeff.clone())
                .or_insert(coeff.clone());
        }

        Self::new(terms)
    }
}

impl<const P: u8, const N: u8, const Z: u8, F: Field> Mul for Multivector<P, N, Z, F> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut terms = HashMap::new();

        for (blade_a, coeff_a) in self.terms.iter() {
            for (blade_b, coeff_b) in rhs.terms.iter() {
                let key = blade_a.symmetric_diff(blade_b);

                let swap_count = BasisBlade::product_swap_count(blade_a, blade_b);
                let mut swap_sign = F::one();
                if swap_count % 2 == 1 {
                    swap_sign = -swap_sign;
                }
                // also need to account for negative signs from the signature

                let coeff = coeff_a.clone() * coeff_b.clone() * swap_sign;

                terms
                    .entry(key)
                    .and_modify(|e: &mut F| *e = e.clone() + coeff.clone())
                    .or_insert(coeff);
            }
        }

        Self::new(terms)
    }
}

impl<const P: u8, const N: u8, const Z: u8, F: Field> Mul<F> for Multivector<P, N, Z, F> {
    type Output = Self;

    fn mul(self, rhs: F) -> Self::Output {
        self * Multivector::from(rhs)
    }
}

impl<const P: u8, const N: u8, const Z: u8, F: Field> PartialEq for Multivector<P, N, Z, F> {
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

impl<const P: u8, const N: u8, const Z: u8, F: Field> Display for Multivector<P, N, Z, F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<TODO Multivector>")
    }
}

pub type VGA2<F> = Multivector<2, 0, 0, F>;
pub type VGA3<F> = Multivector<3, 0, 0, F>;
pub type PGA2<F> = Multivector<2, 0, 1, F>;
pub type PGA3<F> = Multivector<3, 0, 1, F>;
pub type CGA2<F> = Multivector<3, 1, 0, F>;
pub type CGA3<F> = Multivector<4, 1, 0, F>;

#[cfg(test)]
mod test {
    use crate::real::Real;

    use super::*;

    #[test]
    pub fn test_zero_is_additive_identity() {
        let x: VGA3<Real> = VGA3::from(3.0);
        let zero = Multivector::zero();

        assert_eq!(zero.clone() + x.clone(), x);
        assert_eq!(x.clone() + zero, x);
    }

    #[test]
    pub fn test_element_sub_self_is_zero() {
        let x: VGA3<Real> = VGA3::from(3.4);

        assert_eq!(x.clone() - x, Multivector::zero());
    }

    #[test]
    pub fn test_one_is_multiplicative_identity() {
        let one = VGA3::one();
        let x: VGA3<Real> = VGA3::from(2.4);

        assert_eq!(one.clone() * x.clone(), x);
        assert_eq!(x.clone() * one, x);
    }

    #[test]
    pub fn test_scalar_multiplication_distributes() {
        let scalar: VGA3<Real> = VGA3::from(2.0);
        let one = VGA3::one();
        let vector = VGA3::vector(0);
        let bivector = VGA3::bivector(2, 3);

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
        let x: VGA3<Real> = VGA3::vector(0);
        let y: VGA3<Real> = VGA3::vector(1);

        let product = x * y;
        let xy = Multivector::bivector(0, 1);

        assert_eq!(product, xy);
    }

    #[test]
    pub fn test_mutliply_vectors_reversed_gives_negative_bivector() {
        let x: VGA3<Real> = Multivector::vector(0);
        let y: VGA3<Real> = Multivector::vector(1);

        let product = y * x;
        let neg_xy = Multivector::bivector(0, 1) * Real::from(-1.0);

        assert_eq!(product, neg_xy);
    }
}

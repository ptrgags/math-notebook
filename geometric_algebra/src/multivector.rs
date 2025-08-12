use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Add, Mul, Sub},
};

use crate::{basis_blade::BasisBlade, field::Field, format_basis_blade::format_basis_blade};

#[derive(Debug, Clone)]
pub struct Multivector<const P: u8, const N: u8, const Z: u8, F> {
    terms: HashMap<BasisBlade, F>,
}

impl<const P: u8, const N: u8, const Z: u8, F: Field> Multivector<P, N, Z, F> {
    /// Get the dimension, i.e. total number of basis vectors in the algebra.
    pub const fn dimension() -> u8 {
        P + N + Z
    }

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

    /// A bivector e_i ^ e_j
    pub fn bivector(i: u8, j: u8) -> Self {
        Self::from(BasisBlade::bivector(i, j))
    }

    /// A trivector e_i ^ e_j ^ e_k
    pub fn trivector(i: u8, j: u8, k: u8) -> Self {
        Self::from(BasisBlade::trivector(i, j, k))
    }

    /// A quadvector e_i ^ e_j ^ e_k ^ e_l
    pub fn quadvector(i: u8, j: u8, k: u8, l: u8) -> Self {
        Self::from(BasisBlade::quadvector(i, j, k, l))
    }

    /// A pentavector e_i ^ e_j ^ e_k ^ e_l ^ e_m
    pub fn pentavector(i: u8, j: u8, k: u8, l: u8, m: u8) -> Self {
        Self::from(BasisBlade::pentavector(i, j, k, l, m))
    }

    fn get_terms(&self, desired_blades: &[BasisBlade]) -> Vec<(BasisBlade, F)> {
        let zero = F::zero();
        desired_blades
            .iter()
            .map(|blade| {
                let coeff = self.terms.get(&blade).unwrap_or(&zero);
                (*blade, coeff.clone())
            })
            .collect()
    }

    /// Iterate over all the possible even terms for this signature
    /// and get a list of (basis_blade, coefficient) pairs
    pub fn get_all_even_terms(&self) -> Vec<(BasisBlade, F)> {
        self.get_terms(&BasisBlade::get_even_blades(Self::dimension()))
    }

    /// Iterate over all possible odd terms for this signature
    /// and get a list of (basis_blade, coefficient) pairs
    pub fn get_all_odd_terms(&self) -> Vec<(BasisBlade, F)> {
        self.get_terms(&BasisBlade::get_odd_blades(Self::dimension()))
    }

    /// Given the basis vector e_i, get the value of e_i * e_i which is
    /// either 1, -1, or 0 depending on the signature of the algebra.
    /// All the positive vectors go first, then the negative ones, then the
    /// zero ones in this impl.
    pub fn get_signature(index: u8) -> F {
        if index < P {
            // the first P vectors square to +1
            F::one()
        } else if index < P + N {
            // the next N vectors square to -1
            -F::one()
        } else {
            F::zero()
        }
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

                let mut squared_sign = F::one();
                let BasisBlade(overlap) = BasisBlade::intersection(blade_a, blade_b);
                for i in 0..8 {
                    if overlap >> i & 1 == 1 {
                        squared_sign = squared_sign * Self::get_signature(i);
                    }
                }

                let coeff = coeff_a.clone() * coeff_b.clone() * swap_sign * squared_sign;

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

impl<const P: u8, const N: u8, const Z: u8, F: Field + Display> Display
    for Multivector<P, N, Z, F>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let terms: Vec<String> = self
            .terms
            .iter()
            .map(|(blade, coeff)| format!("{}{}", coeff, format_basis_blade::<P, N, Z>(&blade)))
            .collect();

        let sum = terms.join(" + ");
        write!(f, "{}", sum)
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
        let xy = VGA3::bivector(0, 1);

        assert_eq!(product, xy);
    }

    #[test]
    pub fn test_mutliply_vectors_reversed_gives_negative_bivector() {
        let x: VGA3<Real> = VGA3::vector(0);
        let y: VGA3<Real> = VGA3::vector(1);

        let product = y * x;
        let neg_xy = VGA3::bivector(0, 1) * Real::from(-1.0);

        assert_eq!(product, neg_xy);
    }

    #[test]
    pub fn test_pga_multiplication_has_null_terms() {
        let x = PGA2::vector(0);
        let y = PGA2::vector(1);
        let o = PGA2::vector(2);

        // (x + y + o)(2x + o)
        let a = x.clone() + y + o.clone();
        let b = x * Real::from(2.0) + o;

        let result = a * b;

        // (2xx + xo + 2yx + yo + 2ox + oo)
        // (2 + xo - 2xy + yo - 2xo)
        // (2 - 2xy - xo - yo)
        let xy = PGA2::bivector(0, 1);
        let xo = PGA2::bivector(0, 2);
        let yo = PGA2::bivector(1, 2);
        let two = PGA2::from(2.0);
        let expected = two.clone() + xy * two - xo - yo;

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_cga_multiplication_has_negative_terms() {
        let x = CGA2::vector(0);
        let p = CGA2::vector(2);
        let m = CGA2::vector(3);

        // (x + p + m)(2p + m)
        let a = x + p.clone() + m.clone();
        let b = p * Real::from(2.0) + m;

        let result = a * b;

        // (2xp + xm + 2pp + pm + 2mp + mm)
        // 2xp + xm + 2 + pm - 2pm - 1
        // 1 + 2xp + xm - pm
        let xp = CGA2::bivector(0, 2);
        let xm = CGA2::bivector(0, 3);
        let pm = CGA2::bivector(2, 3);
        let expected = CGA2::from(1.0) + xp * CGA2::from(2.0) + xm - pm;

        assert_eq!(result, expected);
    }
}

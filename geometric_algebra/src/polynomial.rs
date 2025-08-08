use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Add, Mul, Neg, Sub},
};

use floats::nearly::is_nearly;

use crate::monomial::Monomial;

#[derive(Debug, Clone)]
pub struct Polynomial {
    terms: HashMap<Monomial, f64>,
}

impl Polynomial {
    pub fn zero() -> Self {
        Self {
            terms: HashMap::new(),
        }
    }

    pub fn one() -> Self {
        Self::from(1.0)
    }

    /// Shorthand for Self::term(1.0, label, 1)
    pub fn var(label: &str) -> Self {
        Self::term(1.0, label, 1)
    }

    pub fn term(coeff: f64, label: &str, power: usize) -> Self {
        let mono = Monomial::power(label, power);
        let mut terms = HashMap::new();
        terms.insert(mono, coeff);

        Self { terms }
    }

    pub fn new(terms: HashMap<Monomial, f64>) -> Self {
        let nonzero_terms = terms
            .into_iter()
            .filter(|(_, coeff)| !is_nearly(*coeff, 0.0))
            .collect();

        Self {
            terms: nonzero_terms,
        }
    }
}

impl From<f64> for Polynomial {
    fn from(value: f64) -> Self {
        if is_nearly(value, 0.0) {
            return Polynomial::zero();
        }

        let mut terms = HashMap::new();
        terms.insert(Monomial::one(), value);

        Self { terms }
    }
}

impl From<Monomial> for Polynomial {
    fn from(value: Monomial) -> Self {
        let mut terms = HashMap::new();
        terms.insert(value, 1.0);

        Self { terms }
    }
}

impl Add for Polynomial {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut terms = self.terms.clone();

        for (monomial, coeff) in rhs.terms.iter() {
            terms
                .entry(monomial.clone())
                .and_modify(|e| *e += *coeff)
                .or_insert(*coeff);
        }

        Self::new(terms)
    }
}

impl Sub for Polynomial {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut terms = self.terms.clone();

        for (monomial, coeff) in rhs.terms.iter() {
            terms
                .entry(monomial.clone())
                .and_modify(|e| *e -= *coeff)
                .or_insert(*coeff);
        }

        Self::new(terms)
    }
}

impl Neg for Polynomial {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let terms = self
            .terms
            .iter()
            .map(|(monomial, coeff)| (monomial.clone(), -coeff))
            .collect();

        Self { terms }
    }
}

impl Mul for Polynomial {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut terms = HashMap::new();

        for (mono_a, coeff_a) in self.terms.iter() {
            for (mono_b, coeff_b) in rhs.terms.iter() {
                let key = mono_a.clone() * mono_b.clone();
                let coeff = *coeff_a * *coeff_b;

                terms
                    .entry(key)
                    .and_modify(|e| *e *= coeff)
                    .or_insert(coeff);
            }
        }

        Self::new(terms)
    }
}

impl PartialEq for Polynomial {
    fn eq(&self, other: &Self) -> bool {
        if self.terms.len() != other.terms.len() {
            return false;
        }

        for (mono, coeff) in self.terms.iter() {
            if other.terms[mono] != *coeff {
                return false;
            }
        }

        true
    }
}

impl Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let terms: Vec<String> = self
            .terms
            .iter()
            .map(|(mono, coeff)| format!("{}{}", coeff, mono))
            .collect();
        let sum = terms.join(" + ");
        write!(f, "{}", sum)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_zero_is_additive_identity() {
        let x = Polynomial::var("x");
        let zero = Polynomial::zero();

        assert_eq!(zero.clone() + x.clone(), x);
        assert_eq!(x.clone() + zero, x);
    }

    #[test]
    pub fn test_element_sub_self_is_zero() {
        let x = Polynomial::from(3.4);

        assert_eq!(x.clone() - x, Polynomial::zero());
    }

    #[test]
    pub fn test_one_is_multiplicative_identity() {
        let one = Polynomial::one();
        let x = Polynomial::var("x");

        assert_eq!(one.clone() * x.clone(), x);
        assert_eq!(x.clone() * one, x);
    }

    #[test]
    pub fn test_scalar_multiplication_distributes() {
        let scalar = Polynomial::from(2.0);
        let one = Polynomial::one();
        let x = Polynomial::term(1.0, "x", 1);
        let zw2 = Polynomial::from(Monomial::var("z") * Monomial::power("w", 2));

        // 2(1 + x + zw^2)
        let sum = one.clone() + x.clone() + zw2.clone();
        let scaled = scalar.clone() * sum;

        // is equivalent to 2 + 2x + zw^2
        let weighted_sum = scalar.clone() * one + scalar.clone() * x + scalar.clone() * zw2;

        assert_eq!(scaled, weighted_sum);
    }

    #[test]
    pub fn test_product_distributes() {
        // let's compute (a + b)(b + c) = (ab + ac + b^2 + bc)
        let a = Polynomial::var("a");
        let b = Polynomial::var("b");
        let c = Polynomial::var("c");

        // (a + b)(b + c)
        let sum_ab = a.clone() + b.clone();
        let sum_bc = b.clone() + c.clone();
        let result = sum_ab * sum_bc;

        // ab + ac + b^2 + bc
        let expected = a.clone() * b.clone()
            + a.clone() * c.clone()
            + b.clone() * b.clone()
            + b.clone() * c.clone();

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_product_commutates() {
        let a = Polynomial::var("a");
        let b = Polynomial::var("b");
        let c = Polynomial::var("c");

        // (a + bc)(a + b + c) = (a + b + c) = (a + bc)
        let poly_a = a.clone() + b.clone() * c.clone();
        let poly_b = a + b + c;

        let ab = poly_a.clone() * poly_b.clone();
        let ba = poly_b * poly_a;

        assert_eq!(ab, ba);
    }
}

use std::ops::Add;

use crate::polynomial::Polynomial;

pub struct RationalPolynomial {
    numerator: Polynomial,
    denominator: Polynomial,
}

impl RationalPolynomial {
    pub fn new(numerator: Polynomial, denominator: Polynomial) -> Self {
        // Consider reducing the polynomials

        Self {
            numerator,
            denominator,
        }
    }
}

impl From<Polynomial> for RationalPolynomial {
    fn from(value: Polynomial) -> Self {
        Self::new(value, Polynomial::one())
    }
}

impl Add for RationalPolynomial {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // Naive method, no computing lcm
        // a/b + c/d = (ad + bc)/bd

        let Self {
            numerator: a,
            denominator: b,
        } = self;
        let Self {
            numerator: c,
            denominator: d,
        } = rhs;

        let denominator = b.clone() * d.clone();
        let numerator = a * d + b * c;

        Self {
            numerator,
            denominator,
        }
    }
}

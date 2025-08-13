use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::{field::Field, polynomial::Polynomial};

#[derive(Clone, PartialEq, Debug)]
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

        Self::new(numerator, denominator)
    }
}

impl Neg for RationalPolynomial {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.numerator.clone(), self.denominator.clone())
    }
}

impl Sub for RationalPolynomial {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        // Naive method, no computing lcm
        // a/b - c/d = (ad - bc)/bd

        let Self {
            numerator: a,
            denominator: b,
        } = self;
        let Self {
            numerator: c,
            denominator: d,
        } = rhs;

        let denominator = b.clone() * d.clone();
        let numerator = a * d - b * c;

        Self::new(numerator, denominator)
    }
}

impl Mul for RationalPolynomial {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        // a/b * c/d = (ac)/(bd)
        let Self {
            numerator: a,
            denominator: b,
        } = self;
        let Self {
            numerator: c,
            denominator: d,
        } = rhs;

        Self::new(a * c, b * d)
    }
}

impl Div for RationalPolynomial {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        // (a/b) / (c/d) = (ad)/(bc)
        let Self {
            numerator: a,
            denominator: b,
        } = self;
        let Self {
            numerator: c,
            denominator: d,
        } = rhs;

        Self::new(a * d, b * c)
    }
}

impl Field for RationalPolynomial {
    fn zero() -> Self {
        Self::from(Polynomial::zero())
    }

    fn one() -> Self {
        Self::from(Polynomial::one())
    }

    /// The inverse of (a/b) is the reciprocal b/a
    fn inverse(&self) -> Self {
        Self {
            numerator: self.denominator.clone(),
            denominator: self.numerator.clone(),
        }
    }
}

impl Display for RationalPolynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            numerator,
            denominator,
        } = self;

        if *denominator == Polynomial::one() {
            return numerator.fmt(f);
        }

        write!(f, "{}/{}", numerator, denominator)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn formats_zero_as_zero() {
        let zero = RationalPolynomial::zero();

        let result = format!("{}", zero);

        assert_eq!(result, "0");
    }

    #[test]
    pub fn formats_one_as_1() {
        let one = RationalPolynomial::one();

        let result = format!("{}", one);

        assert_eq!(result, "1");
    }

    #[test]
    pub fn formats_other_denominator_as_fraction() {
        let numerator = Polynomial::var("x");
        let denominator = Polynomial::from(2.0) * Polynomial::one();
        let fraction = RationalPolynomial::new(numerator, denominator);

        let result = format!("{}", fraction);

        assert_eq!(result, "x/2");
    }

    #[test]
    pub fn formats_multiple_terms_with_parentheses() {
        let numerator = Polynomial::var("x") + Polynomial::one();
        let denominator = Polynomial::from(3.0) + Polynomial::var("y");
        let fraction = RationalPolynomial::new(numerator, denominator);

        let result = format!("{}", fraction);

        assert_eq!(result, "(x + 1)/(3 + y)");
    }
}

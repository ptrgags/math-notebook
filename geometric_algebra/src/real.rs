use std::ops::{Add, Div, Mul, Sub};

use crate::field::Field;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Real(f64);

impl From<f64> for Real {
    fn from(value: f64) -> Self {
        Real(value)
    }
}

impl Add for Real {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Mul for Real {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Sub for Real {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Div for Real {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl Field for Real {
    fn zero() -> Self {
        Self(0.0)
    }

    fn one() -> Self {
        Self(1.0)
    }

    fn inverse(&self) -> Self {
        Self(1.0 / self.0)
    }
}

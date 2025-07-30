use std::ops::{Add, Div, Mul, Sub};

use crate::field::Field;

#[derive(Clone, Copy, Debug)]
pub struct Real(f64);

impl From<f64> for Real {
    fn from(value: f64) -> Self {
        Real(value)
    }
}

// This is duplicated in mobius...
const EPSILON: f64 = 1e-15;
fn is_nearly(a: f64, b: f64) -> bool {
    // based on https://stackoverflow.com/a/28751714
    let diff = (a - b).abs();

    if diff <= EPSILON {
        true
    } else {
        diff <= EPSILON * a.abs().max(b.abs())
    }
}

/// Due to floating point rounding, use approximate equality
impl PartialEq for Real {
    fn eq(&self, other: &Self) -> bool {
        is_nearly(self.0, other.0)
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

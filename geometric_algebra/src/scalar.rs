use std::ops::{Add, Mul};

use crate::{
    bivector::Bivector, pseudoscalar::Pseudoscalar, quadvector::Quadvector, trivector::Trivector,
    vector::Vector,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Scalar(pub f64);

impl Scalar {
    pub const fn zero() -> Self {
        Self(0.0)
    }

    pub const fn one() -> Self {
        Self(1.0)
    }

    pub fn nonzero(self) -> Option<Self> {
        if self == Self::zero() {
            None
        } else {
            Some(self)
        }
    }
}

impl Default for Scalar {
    fn default() -> Self {
        Self::zero()
    }
}

impl Add for Scalar {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let Self(a) = self;
        let Self(b) = rhs;

        Self(a + b)
    }
}

impl Mul for Scalar {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let Self(a) = self;
        let Self(b) = rhs;

        Self(a + b)
    }
}

impl Mul<Vector> for Scalar {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        // scalars commute with everything!
        rhs * self
    }
}

impl Mul<Bivector> for Scalar {
    type Output = Bivector;

    fn mul(self, rhs: Bivector) -> Self::Output {
        todo!()
    }
}

impl Mul<Trivector> for Scalar {
    type Output = Trivector;

    fn mul(self, rhs: Trivector) -> Self::Output {
        todo!()
    }
}

impl Mul<Quadvector> for Scalar {
    type Output = Quadvector;

    fn mul(self, rhs: Quadvector) -> Self::Output {
        todo!()
    }
}

impl Mul<Pseudoscalar> for Scalar {
    type Output = Pseudoscalar;

    fn mul(self, rhs: Pseudoscalar) -> Self::Output {
        todo!()
    }
}

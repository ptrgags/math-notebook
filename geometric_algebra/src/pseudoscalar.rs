use std::ops::Mul;

use crate::{
    bivector::Bivector, quadvector::Quadvector, scalar::Scalar, trivector::Trivector,
    vector::Vector,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pseudoscalar(pub f64);

impl Pseudoscalar {
    pub const fn zero() -> Self {
        Self(0.0)
    }

    pub fn nonzero(self) -> Option<Self> {
        if self == Self::zero() {
            None
        } else {
            Some(self)
        }
    }
}

impl Default for Pseudoscalar {
    fn default() -> Self {
        Self::zero()
    }
}

impl Mul<Vector> for Pseudoscalar {
    type Output = Quadvector;

    fn mul(self, rhs: Vector) -> Self::Output {
        todo!()
    }
}

impl Mul<Trivector> for Pseudoscalar {
    type Output = Bivector;

    fn mul(self, rhs: Trivector) -> Self::Output {
        todo!()
    }
}

impl Mul for Pseudoscalar {
    type Output = Scalar;

    fn mul(self, rhs: Pseudoscalar) -> Self::Output {
        todo!()
    }
}

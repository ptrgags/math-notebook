use std::ops::{Add, Mul};

use crate::{
    bivector::Bivector, pseudoscalar::Pseudoscalar, scalar::Scalar, trivector::Trivector,
    vector::Vector,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Quadvector {
    pub xyzp: f64,
    pub xyzn: f64,
    pub xypn: f64,
    pub xzpn: f64,
    pub yzpn: f64,
}

impl Quadvector {
    pub const fn zero() -> Self {
        Self {
            xyzp: 0.0,
            xyzn: 0.0,
            xypn: 0.0,
            xzpn: 0.0,
            yzpn: 0.0,
        }
    }

    pub fn nonzero(self) -> Option<Self> {
        if self == Self::zero() {
            None
        } else {
            Some(self)
        }
    }
}

impl Default for Quadvector {
    fn default() -> Self {
        Self::zero()
    }
}

impl Add for Quadvector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let Self {
            xyzp: axyzp,
            xyzn: axyzn,
            xypn: axypn,
            xzpn: axzpn,
            yzpn: ayzpn,
        } = self;

        let Self {
            xyzp: bxyzp,
            xyzn: bxyzn,
            xypn: bxypn,
            xzpn: bxzpn,
            yzpn: byzpn,
        } = rhs;

        Self {
            xyzp: axyzp + bxyzp,
            xyzn: axyzn + bxyzn,
            xypn: axypn + bxypn,
            xzpn: axzpn + bxzpn,
            yzpn: ayzpn + byzpn,
        }
    }
}

impl Mul<Scalar> for Quadvector {
    type Output = Quadvector;

    fn mul(self, rhs: Scalar) -> Self::Output {
        todo!()
    }
}

impl Mul<Vector> for Quadvector {
    type Output = (Trivector, Pseudoscalar);

    fn mul(self, rhs: Vector) -> Self::Output {
        todo!()
    }
}

impl Mul<Bivector> for Quadvector {
    type Output = (Bivector, Quadvector);

    fn mul(self, rhs: Bivector) -> Self::Output {
        todo!()
    }
}

impl Mul<Trivector> for Quadvector {
    type Output = (Vector, Trivector);

    fn mul(self, rhs: Trivector) -> Self::Output {
        todo!()
    }
}

impl Mul for Quadvector {
    type Output = (Scalar, Bivector);

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Mul<Pseudoscalar> for Quadvector {
    type Output = Vector;

    fn mul(self, rhs: Pseudoscalar) -> Self::Output {
        // Q * P = rev(rev(P) * rev(Q))
        // rev(P) = P
        // rev(Q) = Q
        // so we have rev(P * Q)
        // result is v, rev(v) = v
        // so Q and P commute
        rhs * self
    }
}

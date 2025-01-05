use std::ops::{Add, Mul};

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

impl Add for Pseudoscalar {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let Self(a) = self;
        let Self(b) = rhs;

        Self(a + b)
    }
}

impl Mul<Scalar> for Pseudoscalar {
    type Output = Pseudoscalar;

    fn mul(self, rhs: Scalar) -> Self::Output {
        let Self(p) = self;
        let Scalar(s) = rhs;

        Pseudoscalar(p * s)
    }
}

impl Mul<Vector> for Pseudoscalar {
    type Output = Quadvector;

    fn mul(self, rhs: Vector) -> Self::Output {
        let Pseudoscalar(ps) = self;
        let Vector { x, y, z, p, n } = rhs;

        Quadvector {
            // xyzpn * n = xyzpnn = -xyzp so - (backwards because n^2 = -1)
            xyzp: ps * -n,
            // xyzpn * p = -xyzppn = -xyzn so -
            xyzn: ps * -p,
            // xyzpn * z = xyzzpn = xypn so +
            xypn: ps * z,
            // xyzpn * y = -xyyzpn = -xzpn so -
            xzpn: ps * -y,
            // xyzpn * x = xxyzpn = yzpn so +
            yzpn: ps * x,
        }
    }
}

impl Mul<Bivector> for Pseudoscalar {
    type Output = Trivector;

    fn mul(self, rhs: Bivector) -> Self::Output {
        todo!()
    }
}

impl Mul<Trivector> for Pseudoscalar {
    type Output = Bivector;

    fn mul(self, rhs: Trivector) -> Self::Output {
        todo!()
    }
}

impl Mul<Quadvector> for Pseudoscalar {
    type Output = Vector;

    fn mul(self, rhs: Quadvector) -> Self::Output {
        todo!()
    }
}

impl Mul for Pseudoscalar {
    type Output = Scalar;

    fn mul(self, rhs: Pseudoscalar) -> Self::Output {
        todo!()
    }
}

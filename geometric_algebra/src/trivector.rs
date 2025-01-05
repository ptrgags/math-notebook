use std::ops::{Add, Mul, Neg};

use crate::{
    bivector::Bivector, pseudoscalar::Pseudoscalar, quadvector::Quadvector, scalar::Scalar,
    vector::Vector,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Trivector {
    pub xyz: f64,
    pub xyp: f64,
    pub xyn: f64,
    pub xzp: f64,
    pub xzn: f64,
    pub xpn: f64,
    pub yzp: f64,
    pub yzn: f64,
    pub ypn: f64,
    pub zpn: f64,
}

impl Trivector {
    pub const fn zero() -> Self {
        Self {
            xyz: 0.0,
            xyp: 0.0,
            xyn: 0.0,
            xzp: 0.0,
            xzn: 0.0,
            xpn: 0.0,
            yzp: 0.0,
            yzn: 0.0,
            ypn: 0.0,
            zpn: 0.0,
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

impl Default for Trivector {
    fn default() -> Self {
        Self::zero()
    }
}

impl Neg for Trivector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            xyz: -self.xyz,
            xyp: -self.xyp,
            xyn: -self.xyn,
            xzp: -self.xzp,
            xzn: -self.xzn,
            xpn: -self.xpn,
            yzp: -self.yzp,
            yzn: -self.yzn,
            ypn: -self.ypn,
            zpn: -self.zpn,
        }
    }
}

impl Add for Trivector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let Self {
            xyz: axyz,
            xyp: axyp,
            xyn: axyn,
            xzp: axzp,
            xzn: axzn,
            xpn: axpn,
            yzp: ayzp,
            yzn: ayzn,
            ypn: aypn,
            zpn: azpn,
        } = self;

        let Self {
            xyz: bxyz,
            xyp: bxyp,
            xyn: bxyn,
            xzp: bxzp,
            xzn: bxzn,
            xpn: bxpn,
            yzp: byzp,
            yzn: byzn,
            ypn: bypn,
            zpn: bzpn,
        } = rhs;

        Self {
            xyz: axyz + bxyz,
            xyp: axyp + bxyp,
            xyn: axyn + bxyn,
            xzp: axzp + bxzp,
            xzn: axzn + bxzn,
            xpn: axpn + bxpn,
            yzp: ayzp + byzp,
            yzn: ayzn + byzn,
            ypn: aypn + bypn,
            zpn: azpn + bzpn,
        }
    }
}

impl Mul<Vector> for Trivector {
    type Output = (Bivector, Quadvector);

    fn mul(self, rhs: Vector) -> Self::Output {
        todo!()
    }
}

impl Mul for Trivector {
    type Output = (Scalar, Bivector, Quadvector);

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Mul<Pseudoscalar> for Trivector {
    type Output = Bivector;

    fn mul(self, rhs: Pseudoscalar) -> Self::Output {
        todo!()
    }
}

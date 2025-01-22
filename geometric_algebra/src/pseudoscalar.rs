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
        let Pseudoscalar(ps) = self;
        let Bivector {
            xy,
            xz,
            xp,
            xn,
            yz,
            yp,
            yn,
            zp,
            zn,
            pn,
        } = rhs;

        Trivector {
            xyz: todo!(),
            xyp: todo!(),
            xyn: todo!(),
            xzp: todo!(),
            xzn: todo!(),
            xpn: todo!(),
            yzp: todo!(),
            yzn: todo!(),
            ypn: todo!(),
            zpn: todo!(),
        }
    }
}

impl Mul<Trivector> for Pseudoscalar {
    type Output = Bivector;

    fn mul(self, rhs: Trivector) -> Self::Output {
        let Pseudoscalar(ps) = self;
        let Trivector {
            xyz,
            xyp,
            xyn,
            xzp,
            xzn,
            xpn,
            yzp,
            yzn,
            ypn,
            zpn,
        } = rhs;

        Bivector {
            xy: todo!(),
            xz: todo!(),
            xp: todo!(),
            xn: todo!(),
            yz: todo!(),
            yp: todo!(),
            yn: todo!(),
            zp: todo!(),
            zn: todo!(),
            pn: todo!(),
        }
    }
}

impl Mul<Quadvector> for Pseudoscalar {
    type Output = Vector;

    fn mul(self, rhs: Quadvector) -> Self::Output {
        let Pseudoscalar(ps) = self;
        let Quadvector {
            xyzp,
            xyzn,
            xypn,
            xzpn,
            yzpn,
        } = rhs;

        Vector {
            x: todo!(),
            y: todo!(),
            z: todo!(),
            p: todo!(),
            n: todo!(),
        }
    }
}

impl Mul for Pseudoscalar {
    type Output = Scalar;

    fn mul(self, rhs: Pseudoscalar) -> Self::Output {
        let Pseudoscalar(a) = self;
        let Pseudoscalar(b) = rhs;

        // xyzpn * xyzpn = yzpn * yzpn = -zpn * zpn = -pn * pn = n^2 = -1
        Scalar(-a * b)
    }
}

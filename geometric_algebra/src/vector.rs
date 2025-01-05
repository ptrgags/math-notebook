use std::ops::{Add, Mul};

use crate::{
    bivector::Bivector, pseudoscalar::Pseudoscalar, quadvector::Quadvector, scalar::Scalar,
    trivector::Trivector,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub p: f64,
    pub n: f64,
}

impl Vector {
    pub const fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            n: 0.0,
            p: 0.0,
        }
    }

    pub const fn x() -> Self {
        let mut v = Self::zero();
        v.x = 1.0;

        v
    }

    pub const fn y() -> Self {
        let mut v = Self::zero();
        v.y = 1.0;

        v
    }

    pub const fn z() -> Self {
        let mut v = Self::zero();
        v.z = 1.0;

        v
    }

    pub const fn p() -> Self {
        let mut v = Self::zero();
        v.p = 1.0;

        v
    }

    pub const fn n() -> Self {
        let mut v = Self::zero();
        v.n = 1.0;

        v
    }

    pub fn nonzero(self) -> Option<Self> {
        if self == Self::zero() {
            None
        } else {
            Some(self)
        }
    }
}

impl Default for Vector {
    fn default() -> Self {
        Self::zero()
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let Self {
            x: ax,
            y: ay,
            z: az,
            p: ap,
            n: an,
        } = self;
        let Self {
            x: bx,
            y: by,
            z: bz,
            p: bp,
            n: bn,
        } = rhs;

        Self {
            x: ax + bx,
            y: ay + by,
            z: az + bz,
            p: ap + bp,
            n: an + bn,
        }
    }
}

impl Mul for Vector {
    type Output = (Scalar, Bivector);

    fn mul(self, rhs: Self) -> Self::Output {
        let Self {
            x: ax,
            y: ay,
            z: az,
            n: an,
            p: ap,
        } = self;
        let Self {
            x: bx,
            y: by,
            z: bz,
            n: bn,
            p: bp,
        } = rhs;
        let s = ax * bx + ay * by + az * bz + ap * bp - an * bn;

        let xy = ax * by - ay * bx;
        let xz = ax * bz - az * bx;
        let xp = ax * bp - ap * bx;
        let xn = ax * bn - an * bx;
        let yz = ay * bz - az * by;
        let yp = ay * bp - ap * by;
        let yn = ay * bn - an * by;
        let zp = az * bp - ap * bz;
        let zn = az * bn - an - bz;
        let pn = ap * bn - an * bp;

        (
            Scalar(s),
            Bivector {
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
            },
        )
    }
}

impl Mul<Scalar> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Scalar) -> Self::Output {
        let Scalar(s) = rhs;
        Self {
            x: s * self.x,
            y: s * self.y,
            z: s * self.z,
            p: s * self.p,
            n: s * self.n,
        }
    }
}

impl Mul<Trivector> for Vector {
    type Output = (Bivector, Quadvector);

    fn mul(self, rhs: Trivector) -> Self::Output {
        todo!()
    }
}

impl Mul<Pseudoscalar> for Vector {
    type Output = Quadvector;

    fn mul(self, rhs: Pseudoscalar) -> Self::Output {
        let Self { x, y, z, p, n } = self;
        let Pseudoscalar(ps) = rhs;

        Quadvector {
            // n * xyzpn = xyzpnn = -xyzp so - (backwards because n^2 = -1)
            xyzp: ps * -n,
            // p * xyzpn = -xyzppn = -xyzn so -
            xyzn: ps * -p,
            // z * xyzpn = xyzzpn = xypn so +
            xypn: ps * z,
            // y * xyzpn = -xyyzpn = -xzpn so -
            xzpn: ps * -y,
            // x * xyzpn = yzpn so +
            yzpn: ps * x,
        }
    }
}

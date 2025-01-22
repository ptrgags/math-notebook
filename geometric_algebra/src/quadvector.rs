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
        let Quadvector {
            xyzp,
            xyzn,
            xypn,
            xzpn,
            yzpn,
        } = self;

        let Scalar(r) = rhs;

        Quadvector {
            xyzp: r * xyzp,
            xyzn: r * xyzn,
            xypn: r * xypn,
            xzpn: r * xzpn,
            yzpn: r * yzpn,
        }
    }
}

impl Mul<Vector> for Quadvector {
    type Output = (Trivector, Pseudoscalar);

    fn mul(self, rhs: Vector) -> Self::Output {
        let Quadvector {
            xyzp,
            xyzn,
            xypn,
            xzpn,
            yzpn,
        } = self;

        let Vector { x, y, z, p, n } = rhs;

        // 1-overlap part (trivector)
        let trivec_part = Trivector {
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
        };

        // 0-overlap part (pseudoscalar)
        let ps_part = Pseudoscalar(todo!());

        (trivec_part, ps_part)
    }
}

impl Mul<Bivector> for Quadvector {
    type Output = (Bivector, Quadvector);

    fn mul(self, rhs: Bivector) -> Self::Output {
        let Quadvector {
            xyzp,
            xyzn,
            xypn,
            xzpn,
            yzpn,
        } = self;

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

        // 2-overlap part (bivector)
        let bivec_part = Bivector {
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
        };

        // 1-overlap part (quadvector)
        let ps_part = Quadvector {
            xyzp: todo!(),
            xyzn: todo!(),
            xypn: todo!(),
            xzpn: todo!(),
            yzpn: todo!(),
        };

        // 0-overlap part (hexavector) - NA in 5D

        (bivec_part, ps_part)
    }
}

impl Mul<Trivector> for Quadvector {
    type Output = (Vector, Trivector);

    fn mul(self, rhs: Trivector) -> Self::Output {
        let Quadvector {
            xyzp,
            xyzn,
            xypn,
            xzpn,
            yzpn,
        } = self;

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

        // 3-overlap part (vector)
        let vec_part = Vector {
            x: todo!(),
            y: todo!(),
            z: todo!(),
            p: todo!(),
            n: todo!(),
        };

        // 2-overlap part (trivector)
        let trivec_part = Trivector {
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
        };

        // 1-overlap part (pentavector) - NA This overlap requires 6+ dimensions
        // 0-overlap part (heptavector) - NA in 5D

        (vec_part, trivec_part)
    }
}

impl Mul for Quadvector {
    type Output = (Scalar, Bivector);

    fn mul(self, rhs: Self) -> Self::Output {
        let Quadvector {
            xyzp: axyzp,
            xyzn: axyzn,
            xypn: axypn,
            xzpn: axzpn,
            yzpn: ayzpn,
        } = self;
        let Quadvector {
            xyzp: bxyzp,
            xyzn: bxyzn,
            xypn: bxypn,
            xzpn: bxzpn,
            yzpn: byzp,
        } = rhs;

        // 4-overlap part (scalar)
        let scalar_part = Scalar(todo!());

        // 3-overlap part (bivector)
        let bivec_part = Bivector {
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
        };

        // 2-overlap part (quadvector) - NA because this kind of overlap would require 6 dimensions
        // 1-overlap part (hexavector) - NA in 5D
        // 0-overlap part (octavector) - NA in 5D
        (scalar_part, bivec_part)
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

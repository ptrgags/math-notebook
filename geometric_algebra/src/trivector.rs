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

impl Mul<Scalar> for Trivector {
    type Output = Trivector;

    fn mul(self, rhs: Scalar) -> Self::Output {
        let Scalar(s) = rhs;

        Self {
            xyz: s * self.xyz,
            xyp: s * self.xyp,
            xyn: s * self.xyn,
            xzp: s * self.xzp,
            xzn: s * self.xzn,
            xpn: s * self.xpn,
            yzp: s * self.yzp,
            yzn: s * self.yzn,
            ypn: s * self.ypn,
            zpn: s * self.zpn,
        }
    }
}

impl Mul<Vector> for Trivector {
    type Output = (Bivector, Quadvector);

    fn mul(self, rhs: Vector) -> Self::Output {
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
        } = self;
        let Vector { x, y, z, p, n } = rhs;

        // 10 x 5 = 50 terms

        // 1-overlap part (bivector) - 10 x ??? terms
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

        // 0-overlap part (quadvector) - 5 x ??? terms
        let quadvec_part = Quadvector {
            xyzp: todo!(),
            xyzn: todo!(),
            xypn: todo!(),
            xzpn: todo!(),
            yzpn: todo!(),
        };

        (bivec_part, quadvec_part)
    }
}

impl Mul<Bivector> for Trivector {
    type Output = (Vector, Trivector, Pseudoscalar);

    fn mul(self, rhs: Bivector) -> Self::Output {
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

        // 10 x 10 = 100 terms

        // 2-overlap (vector part) - 5 * 6 terms = 30
        let vec_part = Vector {
            x: -xyz * yz - xyp * yp + xyn * yn - xzp * zp + xzn * zn + xpn * pn,
            y: todo!(),
            z: todo!(),
            p: todo!(),
            n: todo!(),
        };

        // 1-overlap (trivector part) - 10 * 6 terms = 60
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

        // 0-overlap (pseudoscalar part) - 1 x 10 terms = 10
        let ps_part = Pseudoscalar(
            xyz * pn - xyp * zn + xyn * zp + xzp * yn - xzn * yp + xpn * yz - yzp * xn + yzn * xp
                - ypn * xz
                + zpn * xy,
        );

        (vec_part, trivec_part, ps_part)
    }
}

impl Mul for Trivector {
    type Output = (Scalar, Bivector, Quadvector);

    fn mul(self, rhs: Self) -> Self::Output {
        let Trivector {
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
        let Trivector {
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

        // 10 x 10 = 100 terms

        // 3-overlap part (scalar) - 1 x 10 terms = 10
        let scalar_part = Scalar(todo!());

        // 2-overlap part (bivector) - 10 x ??? terms
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

        // 1-overlap part (quadvector) - 5 x ??? terms
        let quadvec_part = Quadvector {
            xyzp: todo!(),
            xyzn: todo!(),
            xypn: todo!(),
            xzpn: todo!(),
            yzpn: todo!(),
        };

        // 0-overlap part (hexavector) - N/A, we only have 5 dimensions!

        (scalar_part, bivec_part, quadvec_part)
    }
}

impl Mul<Quadvector> for Trivector {
    type Output = (Vector, Trivector);

    fn mul(self, rhs: Quadvector) -> Self::Output {
        // only one blade odd, so even overlap parts commute

        // 3-overlap part: vector (anticommute)
        // 2-overlap part: trivector (commute)
        // 1-overlap part: N/A in 5D
        // 0-overlap part: N/A in 5D
        let (v, t) = rhs * self;
        (-v, t)
    }
}

impl Mul<Pseudoscalar> for Trivector {
    type Output = Bivector;

    fn mul(self, rhs: Pseudoscalar) -> Self::Output {
        // both odd, so _odd_ overlap parts commute

        // 3-overlap part: bivector (commute)
        // 2-overlap part: N/A in 5D
        // 1-overlap part: N/A in 5D
        // 0-overlap part: N/A in 5D
        rhs * self
    }
}

use std::ops::{Add, Mul, Neg};

use crate::{
    pseudoscalar::Pseudoscalar, quadvector::Quadvector, scalar::Scalar, trivector::Trivector,
    vector::Vector,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Bivector {
    pub xy: f64,
    pub xz: f64,
    pub xp: f64,
    pub xn: f64,
    pub yz: f64,
    pub yp: f64,
    pub yn: f64,
    pub zp: f64,
    pub zn: f64,
    pub pn: f64,
}

impl Bivector {
    pub const fn zero() -> Self {
        Self {
            xy: 0.0,
            xz: 0.0,
            xp: 0.0,
            xn: 0.0,
            yz: 0.0,
            yp: 0.0,
            yn: 0.0,
            zp: 0.0,
            zn: 0.0,
            pn: 0.0,
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

impl Default for Bivector {
    fn default() -> Self {
        Self::zero()
    }
}

impl Add for Bivector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let Self {
            xy: axy,
            xz: axz,
            xp: axp,
            xn: axn,
            yz: ayz,
            yp: ayp,
            yn: ayn,
            zp: azp,
            zn: azn,
            pn: apn,
        } = self;

        let Self {
            xy: bxy,
            xz: bxz,
            xp: bxp,
            xn: bxn,
            yz: byz,
            yp: byp,
            yn: byn,
            zp: bzp,
            zn: bzn,
            pn: bpn,
        } = rhs;

        Self {
            xy: axy + bxy,
            xz: axz + bxz,
            xp: axp + bxp,
            xn: axn + bxn,
            yz: ayz + byz,
            yp: ayp + byp,
            yn: ayn + byn,
            zp: azp + bzp,
            zn: azn + bzn,
            pn: apn + bpn,
        }
    }
}

impl Neg for Bivector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            xy: -self.xy,
            xz: -self.xz,
            xp: -self.xp,
            xn: -self.xn,
            yz: -self.yz,
            yp: -self.yp,
            yn: -self.yn,
            zp: -self.zp,
            zn: -self.zn,
            pn: -self.pn,
        }
    }
}

impl Mul<Scalar> for Bivector {
    type Output = Bivector;

    fn mul(self, rhs: Scalar) -> Self::Output {
        let Scalar(r) = rhs;
        Self {
            xy: r * self.xy,
            xz: r * self.xz,
            xp: r * self.xp,
            xn: r * self.xn,
            yz: r * self.yz,
            yp: r * self.yp,
            yn: r * self.yn,
            zp: r * self.zp,
            zn: r * self.zn,
            pn: r * self.pn,
        }
    }
}

impl Mul<Vector> for Bivector {
    type Output = (Vector, Trivector);

    fn mul(self, rhs: Vector) -> Self::Output {
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
        } = self;
        let Vector { x, y, z, p, n } = rhs;

        // There are 10 * 5 = 50 terms in total

        // 1-overlap part - One vector cancels and you have one vector
        // remaining. This feels similar to a dot product, but beware! it
        // _anticommutes_!
        // This is 5 * 4 = 20 terms
        let vec_part = Vector {
            x: xy * y + xz * z + xp * p - xn * n,
            y: -xy * x + yz * z + yp * p - yn * n,
            z: -xz * x - yz * y + zp * p - zn * n,
            p: -xp * x - yp * y - zp * z - pn * n,
            n: -xn * x - yn * y - zn * z - pn * p,
        };

        // 0-overlap part - the blades wedge into a trivector
        // This is 10 * 3 = 30 terms
        // 30 + 20 = 50
        let trivec_part = Trivector {
            xyz: xy * z - xz * y + yz * x,
            xyp: xy * p - xp * y + yp * x,
            xyn: xy * n - xn * y + yn * x,
            xzp: xz * p - xp * z + zp * x,
            xzn: xz * n - xn * z + zn * x,
            xpn: xp * n - xn * p + pn * x,
            yzp: yz * p - yp * z + zp * y,
            yzn: yz * n - yn * z + zn * y,
            ypn: yp * n - yn * p + pn * y,
            zpn: zp * n - zn * p + pn * z,
        };

        (vec_part, trivec_part)
    }
}

impl Mul for Bivector {
    type Output = (Scalar, Bivector, Quadvector);

    fn mul(self, rhs: Self) -> Self::Output {
        let Self {
            xy: axy,
            xz: axz,
            xp: axp,
            xn: axn,
            yz: ayz,
            yp: ayp,
            yn: ayn,
            zp: azp,
            zn: azn,
            pn: apn,
        } = self;
        let Self {
            xy: bxy,
            xz: bxz,
            xp: bxp,
            xn: bxn,
            yz: byz,
            yp: byp,
            yn: byn,
            zp: bzp,
            zn: bzn,
            pn: bpn,
        } = rhs;

        // 10 x 10 = 100 terms

        // 2-overlap part (scalar) - 1 x 10 terms = 10
        let scalar_part = Scalar(
            -axy * bxy
                + -axz * bxz
                + -axp * bxp
                + axn * bxn
                + -ayz * byz
                + -ayp * byp
                + ayn * byn
                + -azp * bzp
                + azn * bzn
                + apn * bpn,
        );
        // 1-overlap part (bivector) - 10 x 6 terms = 60
        // I'm noticing a pattern: each row has exactly 3 minus signs
        let bivec_part = Bivector {
            xy: -axz * byz - axp * byp + axn * bpn + ayz * bxz + ayp * bxp - ayn * bxn,
            xz: axy * byz - axp * bzp + axn * bzn - ayz * bxy + azp * bxp - azn * bxn,
            xp: axy * byp + axz * bzp + axn * bpn - ayp * bxy - azp * bxz - apn * bxn,
            xn: axy * byn + axz * bzn + axp * bpn - ayn * bxy - azn * bxz - apn * bxp,
            yz: -axy * bxz + axz * bxy - ayp * bzp + ayn * bzn + azp * byp - azn * byn,
            yp: -axy * bxp + axp * bxy + ayz * bzp + ayn * bpn - azp * byz - apn * byn,
            yn: -axy * bxn + axn * bxy + ayz * bzn + ayp * bpn - azn * byz - apn * byp,
            zp: -axz * bxp + axp * bxz - ayz * byp + ayp * byz + azn * bpn - apn * bzn,
            zn: -axz * bxn + axn * bxz - ayz * byn + ayn * byz + azp * bpn - apn * bzp,
            pn: -axp * bxn + axn * bxp - ayp * byn + ayn * byp - azp * bzn + azn * bzp,
        };

        // 0-overlap part (quadvector) - 5 x 6 terms = 30
        // The minus signs are due to swaps only, so there's two for each row and follow
        // the same pattern
        let quadvec_part = Quadvector {
            xyzp: axy * bzp - axz * byp + axp * byz + ayz * bxp - ayp * bxz + azp * bxy,
            xyzn: axy * bzn - axz * byn + axn * byz + ayz * bxn - ayn * bxz + azn * bxy,
            xypn: axy * bpn - axp * byn + axn * byp + ayp * bxn - ayn * bxp + apn * bxy,
            xzpn: axz * bpn - axp * bzn + axn * bzp + azp * bxn - azn * bxp + apn * bxz,
            yzpn: ayz * bpn - ayp * bzn + ayn * bzp + azp * byn - azn * byp + apn * byz,
        };

        (scalar_part, bivec_part, quadvec_part)
    }
}

impl Mul<Trivector> for Bivector {
    type Output = (Vector, Trivector, Pseudoscalar);

    fn mul(self, rhs: Trivector) -> Self::Output {
        // even overlap parts commute

        // 2-overlap part: vector (commute)
        // 1-overlap part: trivector (anticommute)
        // 0-overlap part: pseudoscalar (commute)
        let (v, t, p) = rhs * self;
        (v, -t, p)
    }
}

impl Mul<Quadvector> for Bivector {
    type Output = (Bivector, Quadvector);

    fn mul(self, rhs: Quadvector) -> Self::Output {
        // even overlap parts commute

        // 2-overlap part: bivector (commute)
        // 1-overlap part: quadvector (anticommute)
        // 0-overlap part: N/A in 5D
        let (b, q) = rhs * self;
        (b, -q)
    }
}

impl Mul<Pseudoscalar> for Bivector {
    type Output = Trivector;

    fn mul(self, rhs: Pseudoscalar) -> Self::Output {
        // even overlap parts commute

        // 2-overlap part: trivector (commute)
        // 1-overlap part: N/A in 5D
        // 0-overlap part: N/A in 5D
        rhs * self
    }
}

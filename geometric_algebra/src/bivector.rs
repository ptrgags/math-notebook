use std::ops::{Add, Neg};

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

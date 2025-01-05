use std::ops::Add;

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

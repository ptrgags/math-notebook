use std::ops::Add;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Scalar(pub f64);

impl Scalar {
    pub const fn zero() -> Self {
        Self(0.0)
    }

    pub const fn one() -> Self {
        Self(1.0)
    }

    pub fn nonzero(self) -> Option<Self> {
        if self == Self::zero() {
            None
        } else {
            Some(self)
        }
    }
}

impl Default for Scalar {
    fn default() -> Self {
        Self::zero()
    }
}

impl Add for Scalar {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let Self(a) = self;
        let Self(b) = rhs;

        Self(a + b)
    }
}

use std::ops::Neg;

#[derive(Debug, PartialEq)]
pub struct Bivector {
    pub xy: f64,
    pub xo: f64,
    pub yo: f64,
}

impl Bivector {
    pub const fn zero() -> Self {
        Self {
            xy: 0.0,
            xo: 0.0,
            yo: 0.0,
        }
    }

    pub fn new(xy: f64, xo: f64, yo: f64) -> Self {
        Self { xy, xo, yo }
    }
}

impl Neg for Bivector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        todo!()
    }
}

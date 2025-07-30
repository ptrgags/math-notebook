use std::ops::{Add, Div, Mul, Sub};

pub trait Field: Clone + Copy + Add + Mul + Sub + Div {
    fn zero() -> Self;
    fn one() -> Self;
    fn inverse(&self) -> Self;
}

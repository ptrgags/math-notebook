use std::ops::{Add, Div, Mul, Sub};

pub trait Field:
    Clone
    + Copy
    + Add<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Div<Self, Output = Self>
    + PartialEq
{
    fn zero() -> Self;
    fn one() -> Self;
    fn inverse(&self) -> Self;
}

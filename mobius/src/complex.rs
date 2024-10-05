use core::f64;
use std::fmt::{self, Display};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug)]
pub enum Complex {
    Zero,
    Finite(f64, f64),
    Infinity
}

impl Complex {
    pub fn new(real: f64, imag: f64) -> Complex {
        if real == 0.0 && imag == 0.0 {
            Complex::Zero
        } else {
            Complex::Finite(real, imag)
        }
    }

    pub fn from_polar(r: f64, theta: f64) -> Complex {
        if r == 0.0 {
            return Complex::Zero
        }

        let (s, c) = theta.sin_cos();
        return Complex::Finite(r * c, r * s)
    }

    pub fn roots_of_unity(n: usize) -> Vec<Complex> {
        let angle = (f64::consts::TAU) / (n as f64);
        (0..n).map(|i| {
            let theta = (i as f64) * angle;
            let (s, c) = theta.sin_cos();
            Complex::Finite(c, s)
        }).collect()
    }

    pub fn norm(&self) -> f64 {
        match self {
            Complex::Zero => 0.0,
            Complex::Infinity => f64::INFINITY,
            Complex::Finite(a, b) => a * a + b * b
        }
    }

    pub fn mag(&self) -> f64 {
        self.norm().sqrt()
    }

    pub fn conj(&self) -> Self {
        match self {
            Complex::Zero => Complex::Zero,
            Complex::Infinity => Complex::Infinity,
            Complex::Finite(a, b) => Complex::Finite(*a, -b),
        }
    }

    pub fn inverse(&self) -> Self {
        match self {
            Complex::Zero => Complex::Infinity,
            Complex::Infinity => Complex::Zero,
            // 1/z = conj(z) / |z|^2 = (a - bi) / (a^2 + b^2)
            Complex::Finite(a, b) => {
                let denom = a * a + b * b;
                Complex::Finite(a / denom, -b / denom)
            }
        }
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Complex::Zero, rhs) => rhs,
            (Complex::Infinity, _) => Complex::Infinity,
            (lhs, Complex::Zero) => lhs,
            (_, Complex::Infinity) => Complex::Infinity,
            (Complex::Finite(a, b), Complex::Finite(c, d)) => Complex::new(a + c, b + d),
        }
    }
}

impl Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Self::Output {
        match self {
            Complex::Finite(a, b) => Complex::Finite(-a, -b),
            x => x
        }
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs);
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Complex::Zero, Complex::Infinity) => panic!("Multiplying zero and infinity!"),
            (Complex::Infinity, Complex::Zero) => panic!("Multiplying infinity and zero!"),
            (Complex::Zero, _) => Complex::Zero,
            (Complex::Infinity, _) => Complex::Infinity,
            (_, Complex::Zero) => Complex::Zero,
            (_, Complex::Infinity) => Complex::Infinity,
            (Complex::Finite(a, b), Complex::Finite(c, d)) => Complex::Finite(a * c - b * d, a * d + b * c),
        }
    }
}

impl Div for Complex {
    type Output = Complex;
    
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inverse()
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pretty_str = match self {
            Complex::Zero => String::from("0"),
            Complex::Infinity => String::from("♾️"),
            Complex::Finite(a, b) => format!("{}+{}i", a, b)
        };
        write!(f, "{}", pretty_str)
    }
}

impl PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        const EPSILON: f64 = 1e-15;

        match (self, other) {
            (Self::Finite(a, b), Self::Finite(c, d)) => (c-a).abs() < EPSILON && (d - b).abs() < EPSILON,
            (a, b) => a == b,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn new_returns_zero_for_exactly_zero() {
        let result = Complex::new(0.0, 0.0);

        assert_eq!(result, Complex::Zero);
    }

    #[test]
    pub fn new_returns_finite_for_other_values() {
        let result = Complex::new(3.0, 4.0);

        assert_eq!(result, Complex::Finite(3.0, 4.0));
    }

    #[test]
    pub fn from_polar_computes_one() {
        let result = Complex::from_polar(1.0, 0.0);

        assert_eq!(result, Complex::Finite(1.0, 0.0));
    }

    #[test]
    pub fn from_polar_computes_i() {
        let result = Complex::from_polar(1.0, f64::consts::FRAC_PI_2);

        assert_eq!(result, Complex::Finite(0.0, 1.0));
    }

    #[test]
    pub fn from_polar_computes_arbitrary_point() {
        let result = Complex::from_polar(2.0, f64::consts::FRAC_PI_3);

        // From high school trig, a 30 degree angle will give (cos, sin) = (1/2, sqrt(3)/2)
        // but the radius of 2 clears the denominator
        let expected = Complex::Finite(
            1.0, (3.0f64).sqrt()
        );
        assert_eq!(result, expected);

    }

    #[test]
    pub fn sum_of_additive_inverses_is_zero() {
        let a = Complex::Finite(3.0, 4.0);
        let b = Complex::Finite(-3.0, -4.0);

        let sum = a + b;

        assert_eq!(sum, Complex::Zero);
    }

    #[test]
    pub fn sum_of_finites_is_finite() {
        let a = Complex::Finite(10.0, 2.0);
        let b = Complex::Finite(3.0, 5.0);

        let sum = a + b;

        assert_eq!(sum, Complex::Finite(13.0, 7.0));
    }
}
use std::{fmt::Display, ops::Neg};

use crate::{format_numbers::format_term_list, nearly::is_nearly};

use super::vector::Vector;

#[derive(Debug, Clone, Copy)]
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

    pub const fn new(xy: f64, xo: f64, yo: f64) -> Self {
        Self { xy, xo, yo }
    }

    pub fn dual(self) -> Vector {
        let Self { xy, xo, yo } = self;

        // xy * o = xyo so xy -> o
        // xo * y = -xyo so xo -> -y
        // yo * x = xyo so yo -> x
        let x = yo;
        let y = -xo;
        let o = xy;

        Vector::new(x, y, o)
    }

    pub fn vee(self, other: Self) -> Vector {
        let a_dual = self.dual();
        let b_dual = other.dual();

        let result_dual = a_dual.wedge(b_dual);

        result_dual.dual()
    }
}

impl PartialEq for Bivector {
    fn eq(&self, other: &Self) -> bool {
        is_nearly(self.xy, other.xy) && is_nearly(self.xo, other.xo) && is_nearly(self.yo, other.yo)
    }
}

impl Display for Bivector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let &Self { xy, xo, yo } = self;
        let terms = format_term_list(&[(xy, "xy"), (xo, "xo"), (yo, "yo")]);

        write!(f, "{}", terms)
    }
}

impl Neg for Bivector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            xy: -self.xy,
            xo: -self.xo,
            yo: -self.yo,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn to_string_with_zero_formats_zero() {
        let zero = Bivector::zero();

        let result = zero.to_string();

        let expected = String::from("0");
        assert_eq!(result, expected);
    }

    #[test]
    pub fn to_string_formats_vector() {
        let v = Bivector::new(2.0, 3.0, 4.0);

        let result = v.to_string();

        let expected = String::from("2.000xy + 3.000xo + 4.000yo");
        assert_eq!(result, expected);
    }

    #[test]
    pub fn to_string_omits_coeff_of_one() {
        let v = Bivector::new(1.0, 2.0, 1.0);

        let result = v.to_string();

        let expected = String::from("xy + 2.000xo + yo");
        assert_eq!(result, expected);
    }

    #[test]
    pub fn to_string_skips_zero_component() {
        let v = Bivector::new(2.0, 0.0, 3.0);

        let result = v.to_string();

        let expected = String::from("2.000xy + 3.000yo");
        assert_eq!(result, expected);
    }

    #[test]
    pub fn neg_flips_signs_of_all_components() {
        let bivec = Bivector::new(1.0, 2.0, 3.0);

        let result = -bivec;

        let expected = Bivector::new(-1.0, -2.0, -3.0);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn dual_computes_right_complement() {
        let bivec = Bivector::new(1.0, 2.0, 3.0);

        let result = bivec.dual();

        let expected = Vector::new(3.0, -2.0, 1.0);
        assert_eq!(result, expected);
    }
}

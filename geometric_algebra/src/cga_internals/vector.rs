use std::{fmt::Display, ops::Neg};

use super::bivector::Bivector;

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    // Basis vectors, square to 1
    pub x: f64,
    pub y: f64,
    // Origin null vector
    pub o: f64,
}

impl Vector {
    pub const fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            o: 0.0,
        }
    }

    pub const fn new(x: f64, y: f64, o: f64) -> Self {
        Self { x, y, o }
    }

    pub fn wedge(self, b: Self) -> Bivector {
        todo!();
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn to_string_with_zero_formats_zero() {
        let zero = Vector::zero();

        let result = zero.to_string();

        let expected = String::from("0");
        assert_eq!(result, expected);
    }

    #[test]
    pub fn to_string_formats_vector() {
        let v = Vector::new(2.0, 3.0, 4.0);

        let result = v.to_string();

        let expected = String::from("2.0x + 3.0y + 4.0o");
        assert_eq!(result, expected);
    }

    #[test]
    pub fn to_string_omits_coeff_of_one() {
        let v = Vector::new(1.0, 2.0, 1.0);

        let result = v.to_string();

        let expected = String::from("x + 2.0y + o");
        assert_eq!(result, expected);
    }

    #[test]
    pub fn to_string_skips_zero_component() {
        let v = Vector::new(2.0, 0.0, 3.0);

        let result = v.to_string();

        let expected = String::from("2.0x + 3.0o");
        assert_eq!(result, expected);
    }

    #[test]
    pub fn wedge_with_zero_returns_zero_bivector() {
        let a = Vector::new(1.0, 2.0, 0.0);
        let zero = Vector::zero();

        let az = a.wedge(zero);
        let za = zero.wedge(a);

        assert_eq!(az, za);
    }

    #[test]
    pub fn wedge_with_self_returns_zero() {
        let v = Vector::new(1.0, 2.0, 0.0);

        let result = v.wedge(v);

        let expected = Bivector::zero();
        assert_eq!(result, expected);
    }

    #[test]
    pub fn wedge_with_parallel_vector_returns_zero() {
        let v = Vector::new(1.0, 2.0, 0.0);
        let long_v = Vector::new(4.0, 8.0, 0.0);

        let result = v.wedge(long_v);

        let expected = Bivector::zero();

        assert_eq!(result, expected);
    }

    #[test]
    pub fn wedge_with_orthogonal_vectors_returns_simple_bivector() {
        let a = Vector::new(1.0, 0.0, 0.0);
        let b = Vector::new(0.0, 0.0, 1.0);

        let result = a.wedge(b);

        let expected = Bivector::new(0.0, 1.0, 0.0);
        assert_eq!(result, expected);
    }

    #[test]
    pub fn wedge_with_arbitrary_vectors_computes_correct_result() {
        let a = Vector::new(1.0, 1.0, 2.0);
        let b = Vector::new(2.0, 2.0, 1.0);

        let result = a.wedge(b);

        // (x + y + 2o) wedge (2x + 2y + o)
        //   0 + 2xy + xo +
        // 2yx +   0 + yo +
        // 4ox + 4oy +  0
        //
        // = (2 - 2)xy +  (1-4)xo + (1-4)yo
        let expected = Bivector::new(0.0, -3.0, -3.0);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn wedge_is_anticommutative() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(4.0, 5.0, 6.0);

        let ab = a.wedge(b);
        let ba = b.wedge(a);

        assert_eq!(ab, -ba);
    }
}

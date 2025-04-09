use crate::Complex;

pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    assert!(a.is_finite());
    assert!(b.is_finite());
    assert!(t.is_finite());
    (1.0 - t) * a + t * b
}

pub fn lerp_complex(a: Complex, b: Complex, t: f64) -> Complex {
    assert!(a.is_finite());
    assert!(b.is_finite());
    assert!(t.is_finite());
    a * (1.0 - t).into() + b * t.into()
}

#[cfg(test)]
mod test_lerp {

    use std::f64::{INFINITY, NAN};

    use test_case::test_case;

    use crate::nearly::assert_nearly;

    use super::*;

    #[test_case(INFINITY, 1.0, 2.0; "a inf")]
    #[test_case(1.0, NAN, 2.0; "b nan")]
    #[test_case(1.0, 2.0, INFINITY; "c inf")]
    #[should_panic]
    pub fn lerp_with_non_finite_value_panics(a: f64, b: f64, c: f64) {
        lerp(a, b, c);
    }

    #[test]
    pub fn lerp_with_t_zero_returns_a() {
        let a = 2.0;
        let b = 4.0;
        let t = 0.0;

        let result = lerp(a, b, t);

        assert_nearly(result, a);
    }

    #[test]
    pub fn lerp_with_t_one_returns_b() {
        let a = 2.0;
        let b = 4.0;
        let t = 1.0;

        let result = lerp(a, b, t);

        assert_nearly(result, b);
    }

    #[test]
    pub fn lerp_with_t_half_returns_midpoint() {
        let a = 2.0;
        let b = 4.0;
        let t = 0.5;

        let result = lerp(a, b, t);

        // (a + b) / 2
        let midpoint = 3.0;
        assert_nearly(result, midpoint);
    }

    #[test]
    pub fn lerp_with_t_blends_a_b() {
        let a = 2.0;
        let b = 4.0;
        let t = 0.75;

        let result = lerp(a, b, t);

        // 1/4 * 2 + 3/4 * 4 = 1/2 + 3 = 7/2 = 3.5
        let expected = 3.5;
        assert_nearly(result, expected);
    }

    #[test]
    pub fn lerp_with_t_gt_one_extrapolates_past_b() {
        let a = 2.0;
        let b = 4.0;
        let t = 2.0;

        let result = lerp(a, b, t);

        // (-1) * 2 + 2 * 4 = 8 - 2 = 6
        let expected = 6.0;
        assert_nearly(result, expected);
    }

    #[test]
    pub fn lerp_with_t_lt_zero_extrapolates_past_a() {
        let a = 2.0;
        let b = 4.0;
        let t = -1.0;

        let result = lerp(a, b, t);

        // (2) * 2 + (-1) * 4 = 4 - 4 = 0
        let expected = 0.0;
        assert_nearly(result, expected);
    }
}

#[cfg(test)]
mod lerp_complex_tests {
    use std::f64::INFINITY;

    use super::*;

    use test_case::test_case;

    #[test_case(Complex::Infinity, Complex::ONE, 0.0; "a inf")]
    #[test_case(Complex::ONE, Complex::Infinity, 0.0; "b inf")]
    #[test_case(Complex::ONE, Complex::Zero, INFINITY; "c inf")]
    #[should_panic]
    pub fn lerp_with_non_finite_value_panics(a: Complex, b: Complex, t: f64) {
        lerp_complex(a, b, t);
    }

    #[test]
    pub fn lerp_with_t_zero_returns_a() {
        let a = Complex::new(0.0, 1.0);
        let b = Complex::new(1.0, 0.0);
        let t = 0.0;

        let result = lerp_complex(a, b, t);

        assert_eq!(result, a);
    }

    #[test]
    pub fn lerp_with_t_one_returns_b() {
        let a = Complex::new(0.0, 1.0);
        let b = Complex::new(1.0, 0.0);
        let t = 1.0;

        let result = lerp_complex(a, b, t);

        assert_eq!(result, b);
    }

    #[test]
    pub fn lerp_with_t_half_returns_midpoint() {
        let a = Complex::new(0.0, 1.0);
        let b = Complex::new(1.0, 0.0);
        let t = 0.5;

        let result = lerp_complex(a, b, t);

        // (a + b) / 2
        let midpoint = Complex::new(0.5, 0.5);
        assert_eq!(result, midpoint);
    }

    #[test]
    pub fn lerp_with_t_blends_a_b() {
        let a = Complex::new(0.0, 1.0);
        let b = Complex::new(1.0, 0.0);
        let t = 0.75;

        let result = lerp_complex(a, b, t);

        let expected = Complex::new(0.75, 0.25);
        assert_eq!(result, expected);
    }

    #[test]
    pub fn lerp_with_t_gt_one_extrapolates_past_b() {
        let a = Complex::new(0.0, 1.0);
        let b = Complex::new(1.0, 0.0);
        let t = 2.0;

        let result = lerp_complex(a, b, t);

        let expected = Complex::new(2.0, -1.0);
        assert_eq!(result, expected);
    }

    #[test]
    pub fn lerp_with_t_lt_zero_extrapolates_past_a() {
        let a = Complex::new(0.0, 1.0);
        let b = Complex::new(1.0, 0.0);
        let t = -1.0;

        let result = lerp_complex(a, b, t);

        let expected = Complex::new(-1.0, 2.0);
        assert_eq!(result, expected);
    }
}

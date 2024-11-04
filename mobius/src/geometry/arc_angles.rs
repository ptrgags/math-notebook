use std::{
    f64::consts::{PI, TAU},
    fmt::Display,
};

use crate::nearly::is_nearly;

/// Angles for use with CircularArc. These are subject to the following
/// restrictions:
///
/// - a < b < c (CCW arc) or a > b > c (CW arc). This reduces corner cases
/// - |c - a| < 2pi so we're always drawing less than a full circle
/// - the value of a will be reduced to be in [0, 2pi)
#[derive(Clone, Copy, Debug)]
pub struct ArcAngles(pub f64, pub f64, pub f64);

impl ArcAngles {
    pub fn new(a: f64, b: f64, c: f64) -> Result<Self, String> {
        let is_strictly_increasing = a < b && b < c;
        let is_strictly_decreasing = a > b && b > c;

        if !is_strictly_increasing && !is_strictly_decreasing {
            return Err(String::from(
                "angles must be strictly increasing or strictly decreasing",
            ));
        }

        if (c - a).abs() >= TAU {
            return Err(String::from(
                "only arcs less than a full circle are supported",
            ));
        }

        Ok(Self(a, b, c))
    }

    /// Create two semicircles, one for the upper half of a circle traced
    /// from 0 to pi, the other is the lower half from pi to 2pi
    pub fn semicircles() -> (Self, Self) {
        let upper = Self(0.0, PI / 2.0, PI);
        let lower = Self(PI, 3.0 * PI / 2.0, TAU);
        (upper, lower)
    }

    /// Return the same arc but traced backwards.
    pub fn reverse(&self) -> Self {
        let Self(a, b, c) = self;

        Self(*c, *b, *a)
    }
}

impl PartialEq for ArcAngles {
    fn eq(&self, other: &Self) -> bool {
        let ArcAngles(a, b, c) = self;
        let ArcAngles(e, f, g) = other;

        let same_start_point = is_nearly(a % TAU, b % TAU);
        let same_ab = is_nearly(b - a, f - e);
        let same_bc = is_nearly(c - b, g - f);

        same_start_point && same_ab && same_bc
    }
}

impl Display for ArcAngles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(a, b, c) = self;
        write!(f, "{:.3}° -> {:.3}° -> {:.3}°", a, b, c)
    }
}

#[cfg(test)]
mod test {
    use std::f64::{consts::PI, NAN};

    use super::*;
    use test_case::test_case;

    #[test_case(NAN, 1.0, 0.0; "nan a")]
    #[test_case(0.0,  NAN, 3.0; "nan b")]
    #[test_case(0.0, 1.0, NAN; "nan c")]
    #[should_panic]
    pub fn new_with_nan_panics(a: f64, b: f64, c: f64) {
        let _ = ArcAngles::new(a, b, c);
    }

    #[test_case(0.0, 1.0, 0.5; "neither increasing or decreasing")]
    #[test_case(0.0, 0.0, 1.0; "not strictly increasing")]
    #[test_case(1.0, 0.0, 0.0; "not strictly decreasing")]
    #[test_case(PI, -PI, -PI / 2.0; "angles not specified in correct form")]
    #[test_case(0.0, 0.0, 0.0; "degenerate arc")]
    pub fn new_with_non_monotone_angles_returns_error(a: f64, b: f64, c: f64) {
        let result = ArcAngles::new(a, b, c);

        assert!(
            result.is_err_and(|x| x.contains("must be strictly increasing or strictly decreasing"))
        );
    }

    #[test_case(0.0, PI, TAU; "full circle")]
    #[test_case(0.0, TAU, 1.5 * TAU; "bigger than full_circle")]
    #[test_case(0.0, -TAU, -2.0 * TAU; "big clockwise arc")]
    pub fn new_with_big_angle_returns_error(a: f64, b: f64, c: f64) {
        let result = ArcAngles::new(a, b, c);

        assert!(result.is_err_and(|x| x.contains("less than a full circle")));
    }

    #[test_case(0.0, PI / 2.0, PI; "ccw arc")]
    #[test_case(-PI / 2.0, 0.0, PI; "ccw arc that straddles 0")]
    #[test_case(PI / 2.0, PI, 3.0 * PI / 2.0)]
    pub fn new_with_valid_angles_constructs(a: f64, b: f64, c: f64) {
        let result = ArcAngles::new(a, b, c);

        assert!(result.is_ok_and(|ArcAngles(x, y, z)| x == a && y == b && z == c));
    }

    #[test]
    pub fn equals() {
        todo!("partial equality tests");
    }

    #[test]
    pub fn display() {
        todo!("display tests");
    }

    #[test]
    pub fn semicircles() {
        todo!("semicircle tests");
    }

    #[test]
    pub fn reverse() {
        todo!("reverse tests")
    }
}

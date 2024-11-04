use std::f64::consts::TAU;

/// Angles for use with CircularArc. These are subject to the following
/// restrictions:
///
/// - a < b < c (CCW arc) or a > b > c (CW arc). This reduces corner cases
/// - |c - a| < 2pi so we're always drawing less than a full circle
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
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use super::*;
    use test_case::test_case;

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
}

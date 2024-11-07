use std::{
    error::Error,
    f64::consts::{PI, TAU},
    fmt::Display,
};

use crate::{float_error::FloatError, nearly::is_nearly};

#[derive(Debug)]
pub enum ArcAnglesParseError {
    /// One of the input floats is not valid
    BadFloat(FloatError),
    /// Arc angles must be distinct
    DegenerateArc(f64),
    /// Arc is bigger than a full circle. I'm not supporting this case
    BigArcNotSupported(f64, f64),
}

impl Display for ArcAnglesParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadFloat(err) => err.fmt(f),
            Self::DegenerateArc(a) => write!(f, "arc angles must be distinct: ({}, {})", a, a),
            Self::BigArcNotSupported(a, b) => write!(
                f,
                "only arcs smaller than a full circle are supported: ({}, {})",
                a, b
            ),
        }
    }
}

impl From<FloatError> for ArcAnglesParseError {
    fn from(value: FloatError) -> Self {
        Self::BadFloat(value)
    }
}

impl Error for ArcAnglesParseError {}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ArcDirection {
    // Counter-clockwise
    Counterclockwise,
    Clockwise,
}

impl Display for ArcDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Clockwise => write!(f, "CW"),
            Self::Counterclockwise => write!(f, "CCW"),
        }
    }
}

/// Angles for use with CircularArc. These are subject to the following
/// restrictions:
///
/// - a < b < c (CCW arc) or a > b > c (CW arc). This reduces corner cases
/// - |c - a| < 2pi so we're always drawing less than a full circle
/// - the value of a will be reduced to be in [0, 2pi)
#[derive(Clone, Copy, Debug)]
pub struct ArcAngles(pub f64, pub f64);

/// Reduce angles so a is in [0, 2pi)
fn reduce_angles(a: f64, b: f64) -> (f64, f64) {
    let reduced_a = a.rem_euclid(TAU);
    let reduced_b = (b - a) + reduced_a;

    (reduced_a, reduced_b)
}

impl ArcAngles {
    pub fn new(a: f64, b: f64) -> Result<Self, ArcAnglesParseError> {
        FloatError::require_finite("a", a)?;
        FloatError::require_finite("b", b)?;

        if a == b {
            return Err(ArcAnglesParseError::DegenerateArc(a));
        }

        if (b - a).abs() >= TAU {
            return Err(ArcAnglesParseError::BigArcNotSupported(a, b));
        }

        let (reduced_a, reduced_b) = reduce_angles(a, b);
        Ok(Self(reduced_a, reduced_b))
    }

    /// Create two semicircles, one for the upper half of a circle traced
    /// from 0 to pi, the other is the lower half from pi to 2pi
    pub fn semicircles() -> (Self, Self) {
        let upper = Self(0.0, PI);
        let lower = Self(PI, TAU);
        (upper, lower)
    }

    pub fn direction(&self) -> ArcDirection {
        let Self(a, b) = self;
        if b > a {
            ArcDirection::Counterclockwise
        } else {
            ArcDirection::Clockwise
        }
    }

    /// Return the same arc but traced backwards.
    pub fn reverse(&self) -> Self {
        let Self(a, b) = self;

        let (reduced_a, reduced_b) = reduce_angles(*b, *a);
        Self(reduced_a, reduced_b)
    }

    pub fn complement(&self) -> Self {
        let Self(a, b) = self;
        let diff = b - a;
        let abs_diff = diff.abs();
        let other_angle = TAU - abs_diff;

        let adjusted_diff = other_angle / abs_diff * diff;

        Self(*b, b + adjusted_diff)
    }
}

impl PartialEq for ArcAngles {
    fn eq(&self, other: &Self) -> bool {
        let ArcAngles(a, b) = self;
        let ArcAngles(c, d) = other;

        // Even if the midpoints were to differ, the strictly

        is_nearly(*a, *c) && is_nearly(*b, *d)
    }
}

impl Display for ArcAngles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(a, b) = self;
        write!(
            f,
            "{:.3}° -> {:.3}° ({})",
            a.to_degrees(),
            b.to_degrees(),
            self.direction()
        )
    }
}

#[cfg(test)]
mod test {
    use std::f64::{consts::PI, INFINITY, NAN};

    use crate::nearly::assert_nearly;

    use super::*;
    use test_case::test_case;

    #[test_case(NAN, 1.0; "nan a")]
    #[test_case(0.0,  NAN; "nan b")]
    #[test_case(INFINITY, 1.0; "inf a")]
    #[test_case(1.0, INFINITY; "inf b")]
    pub fn new_with_non_finite_returns_error(a: f64, b: f64) {
        let result = ArcAngles::new(a, b);
        assert!(result.is_err_and(|x| matches!(
            x,
            ArcAnglesParseError::BadFloat(FloatError::NonFinite(_, _))
        )));
    }

    #[test]
    pub fn new_with_repeated_angles_returns_error() {
        let angle = PI / 4.0;

        let result = ArcAngles::new(angle, angle);

        assert!(result.is_err_and(|x| matches!(x, ArcAnglesParseError::DegenerateArc(_))));
    }

    #[test_case(0.0, TAU; "full circle")]
    #[test_case(0.0, 1.5 * TAU; "bigger than full_circle")]
    #[test_case(0.0, -2.0 * TAU; "big clockwise arc")]
    pub fn new_with_big_angle_returns_error(a: f64, b: f64) {
        let result = ArcAngles::new(a, b);
        assert!(result.is_err_and(|x| matches!(x, ArcAnglesParseError::BigArcNotSupported(_, _))));
    }

    #[test_case(0.0, PI; "ccw arc")]
    #[test_case(PI, PI / 2.0; "cw arc")]
    #[test_case(PI / 2.0, 3.0 * PI / 2.0; "ccw arc that straddles pi")]
    pub fn new_with_valid_angles_constructs(a: f64, b: f64) {
        let result = ArcAngles::new(a, b);

        assert!(result.is_ok_and(|ArcAngles(x, y)| x == a && y == b));
    }

    #[test_case(2.0 * PI, 3.0 * PI, 0.0, PI; "a exactly at 2pi")]
    #[test_case(9.0 * PI/4.0,  11.0 * PI /4.0, PI / 4.0, 3.0 * PI/4.0; "bigger than 2pi")]
    #[test_case(-PI/4.0, -3.0 * PI/4.0, 7.0 * PI / 4.0, 5.0 * PI / 4.0; "a negative")]
    pub fn new_with_out_of_range_a_constructs_reduced(
        a: f64,
        b: f64,
        expected_x: f64,
        expected_y: f64,
    ) {
        let result = ArcAngles::new(a, b);

        assert!(result.is_ok());
        let ArcAngles(x, y) = result.unwrap();
        assert_nearly(x, expected_x);
        assert_nearly(y, expected_y);
    }

    #[test_case(ArcAngles::new(0.0, PI / 2.0).unwrap(); "ccw arc")]
    #[test_case(ArcAngles::new(0.0, -PI / 2.0).unwrap(); "cw arc")]
    pub fn arc_equals_itself(a: ArcAngles) {
        assert_eq!(a, a);
    }

    #[test]
    pub fn arcs_with_different_midpoint_are_equal() {
        let arc = ArcAngles::new(0.0, PI).unwrap();
        let different_midpoint = ArcAngles::new(0.0, PI).unwrap();

        assert_eq!(arc, different_midpoint);
    }

    #[test_case(ArcAngles::new(0.0, PI).unwrap(), ArcDirection::Counterclockwise; "ccw arc")]
    #[test_case(ArcAngles::new(0.0, - PI / 2.0).unwrap(), ArcDirection::Clockwise; "cw arc")]
    pub fn arc_computes_correct_direction(a: ArcAngles, expected_dir: ArcDirection) {
        let direction = a.direction();

        assert_eq!(direction, expected_dir);
    }

    #[test]
    pub fn semicircles_computes_upper_and_lower_arcs() {
        let (upper, lower) = ArcAngles::semicircles();

        let expected_upper = ArcAngles::new(0.0, PI).unwrap();
        let expected_lower = ArcAngles::new(PI, 2.0 * PI).unwrap();
        assert_eq!(upper, expected_upper);
        assert_eq!(lower, expected_lower);
    }

    #[test]
    pub fn reverse_with_in_range_c_reverses_angles() {
        let arc = ArcAngles::new(PI / 6.0, PI / 3.0).unwrap();

        let result = arc.reverse();

        let expected = ArcAngles::new(PI / 3.0, PI / 6.0).unwrap();
        assert_eq!(result, expected);
    }

    #[test_case(ArcAngles::new(0.0,  -PI).unwrap(), ArcAngles::new(PI, 2.0 * PI).unwrap(); "cw arc")]
    #[test_case(ArcAngles::new(7.0 * PI / 4.0,  5.0 * PI / 2.0).unwrap(), ArcAngles::new(PI / 2.0,  -PI / 4.0).unwrap(); "ccw arc through 2pi")]
    pub fn reverse_with_out_of_range_c_reduces_angles(arc: ArcAngles, expected: ArcAngles) {
        let result = arc.reverse();

        assert_eq!(result, expected);
    }

    #[test_case(ArcAngles::new(0.0, PI/2.0).unwrap(), ArcAngles::new(PI / 2.0, 2.0 * PI).unwrap(); "ccw arc")]
    #[test_case(ArcAngles::new(PI/6.0, -PI/4.0).unwrap(), ArcAngles::new(7.0 * PI / 4.0, PI / 6.0).unwrap(); "cw arc")]
    pub fn complement_returns_other_part_of_circle(arc: ArcAngles, expected: ArcAngles) {
        let result = arc.complement();

        assert_eq!(result, expected);
    }

    #[test]
    pub fn reverse_and_complement_commute() {
        let arc = ArcAngles::new(PI / 3.0, 5.0 * PI / 4.0).unwrap();

        let rev_comp = arc.complement().reverse();
        let comp_rev = arc.reverse().complement();

        assert_eq!(rev_comp, comp_rev);
    }
}

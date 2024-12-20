use std::fmt::Display;

use crate::{cga_internals::bivector::Bivector, error::GAError};

use super::line::Line;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point(Bivector);

impl Point {
    pub const fn origin() -> Self {
        Self(Bivector::new(1.0, 0.0, 0.0))
    }

    pub const fn new(x: f64, y: f64) -> Self {
        // a point is x(yo) + y(xo) + 1
        Self(Bivector::new(1.0, y, x))
    }

    pub fn get(&self) -> Bivector {
        self.0
    }

    pub fn join(self, other: Self) -> Line {
        let Point(a) = self;
        let Point(b) = other;

        let result = a.vee(b);
        Line::from(result)
    }
}

impl TryFrom<Bivector> for Point {
    type Error = GAError;

    fn try_from(value: Bivector) -> Result<Self, Self::Error> {
        if value.xy == 0.0 {
            Err(GAError::PointFromInfinitePoint)
        } else {
            let w = value.xy;
            // the x component goes with the dual of the x basis vector,
            // hence yo
            let x = value.yo;
            let y = value.xo;
            Ok(Self::new(x, y))
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let &Self(bivec) = self;

        write!(f, "({:.3}, {:.3})", bivec.yo, bivec.xo)
    }
}

#[cfg(test)]
pub mod test {
    use std::f64::consts::FRAC_1_SQRT_2;

    use super::*;

    #[test]
    pub fn from_bivector_with_zero_xy_returns_error() {
        let bad_bivector = Bivector::new(0.0, 1.0, 2.0);

        let result = Point::try_from(bad_bivector);

        assert!(matches!(result, Err(GAError::PointFromInfinitePoint)))
    }

    #[test]
    pub fn to_string_formats_as_tuple() {
        let point = Point::new(1.0, -3.0);

        let result = point.to_string();

        let expected = "(1.000, -3.000)";
        assert_eq!(result, expected);
    }

    #[test]
    pub fn join_of_identical_points_gives_zero() {
        let a = Point::new(1.0, 3.0);

        let _result = a.join(a);

        todo!("result shouldn't be a line! it's zero");
    }

    #[test]
    pub fn join_of_finite_points_returns_line_between_them() {
        let a = Point::new(1.0, 0.0);
        let b = Point::new(0.0, 1.0);

        let result = a.join(b);

        let expected = Line::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2, FRAC_1_SQRT_2);
        assert_eq!(result, expected);
    }
}

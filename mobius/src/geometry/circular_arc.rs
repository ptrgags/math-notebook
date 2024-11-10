use std::{error::Error, fmt::Display};

use crate::{interpolation::lerp, Complex};

use super::{circle::Circle, ArcAngles, ArcAnglesParseError, ArcDirection, DirectedEdge, Geometry};

#[derive(Debug)]
pub enum CircularArcError {
    BadAngles(ArcAnglesParseError),
    DuplicatePoint(Complex),
    PointAtCenter(Complex),
}

impl Display for CircularArcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CircularArcError::BadAngles(err) => err.fmt(f),
            CircularArcError::DuplicatePoint(point) => write!(f, "Duplicate point: {}", point),
            CircularArcError::PointAtCenter(point) => {
                write!(f, "Point at center of circle not allowed: {}", point)
            }
        }
    }
}

impl From<ArcAnglesParseError> for CircularArcError {
    fn from(value: ArcAnglesParseError) -> Self {
        Self::BadAngles(value)
    }
}

impl Error for CircularArcError {}

// Directed circular arc through 3 points on a circular arc
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct CircularArc {
    pub circle: Circle,
    pub angles: ArcAngles,
}

impl CircularArc {
    pub fn new(circle: Circle, angles: ArcAngles) -> Self {
        Self { circle, angles }
    }

    /// Given a circle and 3 points (a, b, c) on that circle, compute a circular
    /// arc from a -> b -> c
    pub fn from_circle_and_points(
        circle: Circle,
        a: Complex,
        b: Complex,
        c: Complex,
    ) -> Result<Self, CircularArcError> {
        if a == b {
            return Err(CircularArcError::DuplicatePoint(a));
        }

        if b == c {
            return Err(CircularArcError::DuplicatePoint(b));
        }

        if a == c {
            return Err(CircularArcError::DuplicatePoint(a));
        }

        if a == circle.center {
            return Err(CircularArcError::PointAtCenter(a));
        }

        if b == circle.center {
            return Err(CircularArcError::PointAtCenter(b));
        }

        if c == circle.center {
            return Err(CircularArcError::PointAtCenter(c));
        }

        // Determine if the 3 points circulate counterclockwise or
        // clockwise by forming a triangle ABC and computing
        // (the magnitude of) the wedge product.
        let ab = b - a;
        let ac = c - a;
        let ccw = Complex::wedge(ab, ac) > 0.0;

        // Get the raw angles
        let theta_a = circle.get_angle(a).unwrap();
        let theta_c = circle.get_angle(c).unwrap();

        let direction = if ccw {
            ArcDirection::Counterclockwise
        } else {
            ArcDirection::Clockwise
        };

        let angles = ArcAngles::from_raw_angles(theta_a, theta_c, direction);
        Ok(Self { circle, angles })
    }

    pub fn direction(&self) -> ArcDirection {
        self.angles.direction()
    }

    pub fn interpolate(&self, t: f64) -> Complex {
        let ArcAngles(a, b) = self.angles;
        let in_between_angle = lerp(a, b, t);
        self.circle.get_point(in_between_angle)
    }

    pub fn reverse(&self) -> Self {
        Self {
            circle: self.circle,
            angles: self.angles.reverse(),
        }
    }

    pub fn complement(&self) -> Self {
        Self {
            circle: self.circle,
            angles: self.angles.complement(),
        }
    }
}

impl Geometry for CircularArc {}
impl DirectedEdge for CircularArc {
    fn start(&self) -> Complex {
        let ArcAngles(a, _) = self.angles;
        self.circle.get_point(a)
    }

    fn end(&self) -> Complex {
        let ArcAngles(_, b) = self.angles;
        self.circle.get_point(b)
    }
}

impl Display for CircularArc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let CircularArc {
            circle: Circle { center, radius },
            angles,
        } = self;
        write!(f, "Arc(c={}, r={:.3}, {})", center, radius, angles)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use test_case::test_case;

    #[test_case(Complex::ONE, Complex::ONE, Complex::I; "a equals b")]
    #[test_case(Complex::I, Complex::ONE, Complex::ONE; "b equals c")]
    #[test_case(Complex::ONE, Complex::I, Complex::ONE; "a equals c")]
    pub fn from_circle_and_points_with_duplicate_points_returns_error(
        a: Complex,
        b: Complex,
        c: Complex,
    ) {
        let circle = Circle::unit_circle();

        let result = CircularArc::from_circle_and_points(circle, a, b, c);

        assert!(result.is_err_and(|x| matches!(x, CircularArcError::DuplicatePoint(_))))
    }

    #[test_case(Complex::Zero, Complex::ONE, Complex::I; "a is zero")]
    #[test_case(Complex::I, Complex::Zero, Complex::ONE; "b is zero")]
    #[test_case(Complex::ONE, Complex::I, Complex::Zero; "c is zero")]
    pub fn from_circle_and_points_with_circle_center_returns_error(
        a: Complex,
        b: Complex,
        c: Complex,
    ) {
        let circle = Circle::unit_circle();

        let result = CircularArc::from_circle_and_points(circle, a, b, c);

        assert!(result.is_err_and(|x| matches!(x, CircularArcError::PointAtCenter(_))))
    }

    #[test]
    pub fn from_circle_and_points_with_points_on_circle_computes_arc() {
        todo!()
    }

    #[test]
    pub fn from_circle_and_points_with_points_off_circle_computes_arc() {
        todo!()
    }

    #[test]
    pub fn missing_tests() {
        todo!("from_circle_and_points, directed_edge");
    }
}

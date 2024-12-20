use derive_more::derive::Display;

use super::point::Point;
use crate::cga_internals::vector::Vector;

#[derive(Debug, Display, Clone, Copy, PartialEq)]
#[display("Line(n=({}, {}), d={})", self.0.x, self.0.y, self.0.o)]
pub struct Line(pub Vector);

impl Line {
    pub const fn new(nx: f64, ny: f64, d: f64) -> Self {
        Self(Vector::new(nx, ny, d))
    }

    pub fn from_angle_dist(theta_n: f64, d: f64) -> Self {
        let (ny, nx) = theta_n.sin_cos();
        Self(Vector::new(nx, ny, d))
    }

    pub fn meet(self, other: Self) -> Point {
        let Line(line1) = self;
        let Line(line2) = other;

        let intersection = line1.wedge(line2);
        return Point::try_from(intersection).unwrap();
    }
}

impl From<Vector> for Line {
    fn from(value: Vector) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::{FRAC_PI_3, FRAC_PI_4, SQRT_2};

    use super::*;

    #[test]
    pub fn from_angle_dist_computes_correct_normal() {
        let line = Line::from_angle_dist(FRAC_PI_3, 1.0);

        let expected = Line::new(0.5, (3.0f64).sqrt() / 2.0, 1.0);
        assert_eq!(line, expected);
    }

    #[test]
    pub fn meet_with_axes_returns_origin() {
        let x_axis = Line::new(0.0, 1.0, 0.0);
        let y_axis = Line::new(1.0, 0.0, 0.0);

        let result = x_axis.meet(y_axis);

        let expected = Point::origin();
        assert_eq!(result, expected);
    }

    #[test]
    pub fn meet_with_other_lines_returns_intersection_point() {
        // Lines that cross at (1, 1)
        let l1 = Line::from_angle_dist(FRAC_PI_4, SQRT_2);
        let l2 = Line::from_angle_dist(3.0 * FRAC_PI_4, 0.0);

        let result = l1.meet(l2);

        let expected = Point::new(1.0, 1.0);
        assert_eq!(result, expected);
    }
}

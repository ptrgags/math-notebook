use derive_more::derive::Display;

use super::point::Point;
use crate::cga_internals::vector::Vector;

#[derive(Display)]
#[display("Line")]
pub struct Line(pub Vector);

impl Line {
    pub const fn new(nx: f64, ny: f64, d: f64) -> Self {
        todo!();
    }

    pub fn meet(&self, other: &Line) -> Point {
        let &Line(line1) = self;
        let &Line(line2) = other;

        let intersection = line1.wedge(line2);
        return Point::from(intersection);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn meet_with_axes_returns_origin() {
        let x_axis = Line::new(0.0, 1.0, 0.0);
        let y_axis = Line::new(1.0, 0.0, 0.0);

        let result = x_axis.meet(&y_axis);

        let expected = Point::origin();
        assert_eq!(result, expected);
    }
}

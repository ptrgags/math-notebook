use std::f64::consts::{PI, TAU};

use crate::{
    geometry::{Circle, CircularArc, GeneralizedCircle, Line, LineSegment},
    nearly::is_nearly,
    Complex,
};

#[derive(Debug)]
pub enum OrthogonalArcError {
    DuplicatePoint(Complex),
    PointAtCenter(Complex),
}

pub fn compute_orthogonal_circle(
    circle: Circle,
    a: Complex,
    b: Complex,
) -> Result<GeneralizedCircle, OrthogonalArcError> {
    if a == b {
        return Err(OrthogonalArcError::DuplicatePoint(a));
    }

    if a == circle.center {
        return Err(OrthogonalArcError::PointAtCenter(a));
    }

    if b == circle.center {
        return Err(OrthogonalArcError::PointAtCenter(b));
    }

    // If the arc was exactly half a circle, the result will be a line, not a
    // circle, so let's check for that first.
    let angle_a = circle.get_angle(a).unwrap();
    let angle_b = circle.get_angle(b).unwrap();

    // Since we're comparing with half a circle, it doesn't matter if we
    // measure the difference clockwise or counterclockwise.
    if is_nearly((angle_a - angle_b).rem_euclid(TAU), PI) {
        return Ok(GeneralizedCircle::Line(Line::from_points(b, a).unwrap()));
    }

    // In the past, I computed this for a unit circle
    // https://github.com/ptrgags/p5-sketchbook/tree/main/HyperbolicConnections#method-2-kite-analysis
    // but here I need the formula where the first circle is a circle with
    // radius r1.
    //
    // Following the steps of the derivation with the new radius and
    // center gives the following updated formulas
    //
    // r = pq/(2r1)
    // p = sqrt((4r^4)/(4r^2-q^2))
    //
    // finally, the new circle center now is offset by the first
    // circle's center.
    //
    // TODO: Write a better explainer for this.

    let q = (a - b).mag();
    let r1 = circle.radius;
    let double_r1 = 2.0 * r1;
    let denominator = (double_r1 - q) * (double_r1 + q);
    let p = double_r1 * r1 * (1.0 / denominator).sqrt();
    let orthog_radius = 0.5 * p * q / r1;

    let angle_bisector = 0.5 * (angle_a + angle_b);
    let angle_bisector = if (angle_b - angle_a) % TAU > PI {
        (angle_bisector + PI) % TAU
    } else {
        angle_bisector
    };

    let orthog_center = circle.center + Complex::from_polar(p, angle_bisector);
    let orthog_circle = Circle {
        center: orthog_center,
        radius: orthog_radius,
    };

    Ok(GeneralizedCircle::Circle(orthog_circle))
}

#[derive(Clone, Copy, Debug)]
pub enum OrthogonalArc {
    Arc(CircularArc),
    Segment(LineSegment),
}

#[cfg(test)]
mod test {
    use crate::geometry::Line;

    use super::*;
    use test_case::test_case;

    fn make_circle() -> Circle {
        Circle::new(Complex::new(1.0, 2.0), 4.0)
    }

    #[test_case(Complex::new(1.0, 2.0), Complex::new(5.0, 2.0); "a at center")]
    #[test_case(Complex::new(5.0, 2.0), Complex::new(1.0, 2.0); "b at center")]
    pub fn compute_orthog_circle_returns_error_for_center(a: Complex, b: Complex) {
        let circle = make_circle();

        let result = compute_orthogonal_circle(circle, a, b);

        assert!(result.is_err_and(|x| matches!(x, OrthogonalArcError::PointAtCenter(_))))
    }

    #[test]
    pub fn compute_orthog_circle_with_duplicate_point_returns_error() {
        let circle = make_circle();
        let point = Complex::new(5.0, 2.0);

        let result = compute_orthogonal_circle(circle, point, point);

        assert!(result.is_err_and(|x| matches!(x, OrthogonalArcError::DuplicatePoint(_))))
    }

    #[test_case(Complex::new(5.0, 2.0), Complex::new(1.0, 6.0), Circle::new(Complex::new(5.0, 6.0), 4.0); "quarter_circle")]
    pub fn compute_orthog_circle_with_points_on_circle_computes_correct_circle(
        a: Complex,
        b: Complex,
        expected: Circle,
    ) {
        let circle = make_circle();

        let result = compute_orthogonal_circle(circle, a, b);
        let gen_circle = result.unwrap();

        match gen_circle {
            GeneralizedCircle::Circle(circle) => assert_eq!(circle, expected),
            GeneralizedCircle::Line(line) => panic!("not a circle! {}", line),
        }
    }

    #[test_case(Complex::new(5.0, 2.0), Complex::new(-3.0, 2.0), Line::new(-Complex::I, -2.0).unwrap(); "diameter at 0 and pi")]
    #[test_case(Complex::new(1.0, 6.0), Complex::new(1.0, -2.0), Line::new(-Complex::ONE, -1.0).unwrap(); "diameter at pi/2 and -pi/2")]
    pub fn compute_orthog_circle_with_points_on_diameter_computes_line(
        a: Complex,
        b: Complex,
        expected: Line,
    ) {
        let circle = make_circle();

        let result = compute_orthogonal_circle(circle, a, b);
        let gen_circle = result.unwrap();

        match gen_circle {
            GeneralizedCircle::Circle(circle) => panic!("not a line! {}", circle),
            GeneralizedCircle::Line(line) => assert_eq!(line, expected),
        }
    }
}

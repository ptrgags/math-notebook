use std::f64::consts::{PI, TAU};

use crate::{
    geometry::{
        ArcAngles, Circle, CircularArc, DirectedEdge, DoubleRay, GeneralizedCircle, Line,
        LineSegment,
    },
    nearly::is_nearly,
    Complex,
};

pub fn compute_orthogonal_circle(
    circle: Circle,
    intersection_angles: ArcAngles,
) -> GeneralizedCircle {
    let ArcAngles(angle_a, angle_b) = intersection_angles;

    let a = circle.get_point(angle_a);
    let b = circle.get_point(angle_b);

    // If the arc is a semicircle, then the orthogonal circle is the line
    // through the points.
    if is_nearly(intersection_angles.central_angle(), PI) {
        return GeneralizedCircle::Line(Line::from_points(b, a).unwrap());
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

    let angle_bisector = intersection_angles.interpolate(0.5);
    let angle_bisector = if intersection_angles.central_angle() > PI {
        (angle_bisector + PI).rem_euclid(TAU)
    } else {
        angle_bisector
    };

    let orthog_center = circle.center + Complex::from_polar(p, angle_bisector);
    let orthog_circle = Circle {
        center: orthog_center,
        radius: orthog_radius,
    };

    GeneralizedCircle::Circle(orthog_circle)
}

/// Similar to ClineArcGeometry, but the cases are a little more
/// restricted since both endpoints are finite.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OrthogonalArc {
    Arc(CircularArc),
    Diameter(LineSegment),
    /// Part of the diameter line outside the circle.
    /// see integer_arc.rs
    DiameterOutside(DoubleRay),
}

impl OrthogonalArc {
    pub fn reverse(&self) -> Self {
        match self {
            OrthogonalArc::Arc(circular_arc) => OrthogonalArc::Arc(circular_arc.reverse()),
            OrthogonalArc::Diameter(line_segment) => {
                OrthogonalArc::Diameter(line_segment.reverse())
            }
            OrthogonalArc::DiameterOutside(double_ray) => {
                OrthogonalArc::DiameterOutside(double_ray.reverse())
            }
        }
    }
}

pub fn compute_orthogonal_arc(arc: CircularArc) -> OrthogonalArc {
    let circle = arc.circle;
    let orthog_circle = match compute_orthogonal_circle(circle, arc.angles) {
        GeneralizedCircle::Circle(sub_circle) => sub_circle,
        GeneralizedCircle::Line(_) => {
            let a = arc.start();
            let b = arc.end();
            return OrthogonalArc::Diameter(LineSegment::new(a, b));
        }
    };

    // Compute the arc from b -> a that's inside the original circle. This will
    // match the orientation of the original arc for small input arcs,
    // but will be the opposite orientation for large input arcs.
    let angle_a_raw = orthog_circle.get_angle(arc.start()).unwrap();
    let angle_b_raw = orthog_circle.get_angle(arc.end()).unwrap();
    let mut sub_angles = ArcAngles::from_raw_angles(angle_b_raw, angle_a_raw, arc.direction());
    if arc.angles.central_angle() > PI {
        sub_angles = sub_angles.complement().reverse();
    }

    OrthogonalArc::Arc(CircularArc::new(orthog_circle, sub_angles))
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::{geometry::Line, unit_complex::UnitComplex};

    use super::*;
    use test_case::test_case;

    type Res = Result<(), Box<dyn Error>>;

    fn make_circle() -> Circle {
        Circle::new(Complex::new(1.0, 2.0), 4.0)
    }

    #[test_case(0.0, PI / 2.0, Circle::new(Complex::new(5.0, 6.0), 4.0); "quarter circle ccw")]
    #[test_case(PI / 2.0, 0.0, Circle::new(Complex::new(5.0, 6.0), 4.0); "quarter circle cw")]
    #[test_case(0.0, 3.0 * PI / 2.0, Circle::new(Complex::new(5.0, -2.0), 4.0); "three quarters circle ccw")]
    #[test_case(3.0 * PI / 2.0, 0.0, Circle::new(Complex::new(5.0, -2.0), 4.0); "three quarters circle cw")]
    pub fn compute_orthog_circle_with_points_on_circle_computes_correct_circle(
        a: f64,
        b: f64,
        expected: Circle,
    ) {
        let circle = make_circle();
        let angles = ArcAngles::new(a, b).unwrap();

        let result = compute_orthogonal_circle(circle, angles);

        match result {
            GeneralizedCircle::Circle(circle) => assert_eq!(circle, expected),
            GeneralizedCircle::Line(line) => panic!("not a circle! {}", line),
        }
    }

    #[test_case(0.0, PI, Line::new(-UnitComplex::I, -2.0).unwrap(); "diameter at 0 and pi")]
    #[test_case(PI / 2.0, -PI / 2.0, Line::new(-UnitComplex::ONE, -1.0).unwrap(); "diameter at pi/2 and -pi/2")]
    pub fn compute_orthog_circle_with_points_on_diameter_computes_line(
        a: f64,
        b: f64,
        expected: Line,
    ) {
        let circle = make_circle();
        let angles = ArcAngles::new(a, b).unwrap();

        let result = compute_orthogonal_circle(circle, angles);

        match result {
            GeneralizedCircle::Circle(circle) => panic!("not a line! {}", circle),
            GeneralizedCircle::Line(line) => assert_eq!(line, expected),
        }
    }

    #[test]
    pub fn compute_orthogonal_arc_with_semicircle_returns_diameter() -> Res {
        let semicircle = CircularArc::new(Circle::unit_circle(), ArcAngles::new(0.0, PI)?);

        let result = compute_orthogonal_arc(semicircle);

        let expected = LineSegment::new(Complex::ONE, -Complex::ONE);
        match result {
            OrthogonalArc::Diameter(line_segment) => assert_eq!(line_segment, expected),
            x => panic!("not a diameter: {:?}", x),
        }

        Ok(())
    }

    #[test]
    pub fn compute_orthogonal_arc_with_small_arc_returns_arc_inside_circle() -> Res {
        let small_arc = CircularArc::new(Circle::unit_circle(), ArcAngles::new(PI / 2.0, PI)?);

        let result = compute_orthogonal_arc(small_arc);

        let expected = CircularArc::new(
            Circle::new(Complex::new(-1.0, 1.0), 1.0),
            ArcAngles::new(3.0 * PI / 2.0, 2.0 * PI)?,
        );

        match result {
            OrthogonalArc::Arc(circular_arc) => assert_eq!(circular_arc, expected),
            x => panic!("expected an arc: {:?}", x),
        }

        Ok(())
    }

    #[test]
    pub fn compute_orthogonal_arc_with_large_arc_returns_arc_inside_circle() -> Res {
        let large_arc =
            CircularArc::new(Circle::unit_circle(), ArcAngles::new(PI / 2.0, 2.0 * PI)?);

        let result = compute_orthogonal_arc(large_arc);

        let expected = CircularArc::new(
            Circle::new(Complex::new(1.0, 1.0), 1.0),
            ArcAngles::new(3.0 * PI / 2.0, PI)?,
        );

        match result {
            OrthogonalArc::Arc(circular_arc) => assert_eq!(circular_arc, expected),
            x => panic!("expected an arc: {:?}", x),
        }

        Ok(())
    }
}

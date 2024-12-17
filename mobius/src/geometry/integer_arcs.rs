use std::f64::consts::{PI, TAU};

use thiserror::Error;

use crate::{
    complex_error::ComplexError,
    geometry::{
        ArcAngles, ArcAnglesError, ArcDirection, Circle, CircularArc, DirectedEdge, DoubleRay,
        GeneralizedCircle,
    },
};

use super::orthogonal_arcs::{compute_orthogonal_arc, compute_orthogonal_circle, OrthogonalArc};

#[derive(Debug, Error)]
pub enum IntegerArcError {
    #[error("{0}")]
    BadComplex(#[from] ComplexError),
    #[error("{0}")]
    BadAngles(#[from] ArcAnglesError),
    #[error("a and b must be distinct: {0}")]
    DuplicateInt(i64),
    #[error("n must be nonzero")]
    ZeroPoints,
    #[error("out of range value: {0} = {0}")]
    ValueOutOfRange(String, i64),
}

/// compute a circle with diameter between a and b on the real line
pub fn circle_on_line(a: i64, b: i64) -> Result<Circle, IntegerArcError> {
    if a == b {
        return Err(IntegerArcError::DuplicateInt(a));
    }

    let a = a as f64;
    let b = b as f64;

    let midpoint = 0.5 * (a + b);
    let radius = 0.5 * (a - b).abs();

    Ok(Circle {
        center: midpoint.into(),
        radius,
    })
}

pub fn arc_on_line_by_direction(
    a: i64,
    b: i64,
    direction: ArcDirection,
) -> Result<CircularArc, IntegerArcError> {
    let circle = circle_on_line(a, b)?;

    let angles = match direction {
        ArcDirection::Counterclockwise if a < b => ArcAngles::new(PI, 2.0 * PI),
        ArcDirection::Counterclockwise => ArcAngles::new(0.0, PI),
        ArcDirection::Clockwise if a < b => ArcAngles::new(PI, 0.0),
        ArcDirection::Clockwise => ArcAngles::new(0.0, -PI),
    }?;

    Ok(CircularArc { circle, angles })
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Hemisphere {
    North,
    South,
}

pub fn arc_on_line_by_hemisphere(
    a: i64,
    b: i64,
    hemisphere: Hemisphere,
) -> Result<CircularArc, IntegerArcError> {
    let circle = circle_on_line(a, b)?;

    let angles = match hemisphere {
        Hemisphere::North if a < b => ArcAngles::new(PI, 0.0),
        Hemisphere::North => ArcAngles::new(0.0, PI),
        Hemisphere::South if a < b => ArcAngles::new(PI, 2.0 * PI),
        Hemisphere::South => ArcAngles::new(0.0, -PI),
    }?;

    Ok(CircularArc { circle, angles })
}

fn cyclotomic_angles(a: usize, b: usize, n: usize) -> Result<ArcAngles, IntegerArcError> {
    if n == 0 {
        return Err(IntegerArcError::ZeroPoints);
    }

    if a >= n {
        return Err(IntegerArcError::ValueOutOfRange("a".into(), a as i64));
    }

    if b >= n {
        return Err(IntegerArcError::ValueOutOfRange("b".into(), b as i64));
    }

    if a == b {
        return Err(IntegerArcError::DuplicateInt(a as i64));
    }
    let a = a as f64;
    let b = b as f64;

    let step_size = TAU / (n as f64);
    Ok(ArcAngles::new(a * step_size, b * step_size)?)
}

pub fn circle_on_circle(
    a: usize,
    b: usize,
    n: usize,
) -> Result<GeneralizedCircle, IntegerArcError> {
    let angles = cyclotomic_angles(a, b, n)?;
    Ok(compute_orthogonal_circle(Circle::unit_circle(), angles))
}

pub fn arc_on_circle_by_direction(
    a: usize,
    b: usize,
    n: usize,
    direction: ArcDirection,
) -> Result<OrthogonalArc, IntegerArcError> {
    let angles = cyclotomic_angles(a, b, n)?;
    let arc = CircularArc::new(Circle::unit_circle(), angles);
    let orthog_arc = compute_orthogonal_arc(arc);

    let adjusted_arc = match orthog_arc {
        OrthogonalArc::Arc(circular_arc) => {
            let selected_arc = if circular_arc.direction() == direction {
                circular_arc
            } else {
                circular_arc.complement()
            };
            OrthogonalArc::Arc(selected_arc)
        }
        OrthogonalArc::Diameter(line_segment) => {
            // When do we flip this?
            OrthogonalArc::Diameter(line_segment)
        }
        OrthogonalArc::DiameterOutside(_) => unreachable!(),
    };

    Ok(adjusted_arc)
}

pub fn arc_on_circle_by_hemisphere(
    a: usize,
    b: usize,
    n: usize,
    hemisphere: Hemisphere,
) -> Result<OrthogonalArc, IntegerArcError> {
    let angles = cyclotomic_angles(a, b, n)?;
    let arc = CircularArc::new(Circle::unit_circle(), angles);
    let orthog_arc = compute_orthogonal_arc(arc);

    let adjusted_arc = match orthog_arc {
        OrthogonalArc::Arc(circular_arc) => {
            let unit_circle = Circle::unit_circle();
            let inside_circle = unit_circle.point_inside(circular_arc.interpolate(0.5));
            let arc_hemisphere = if inside_circle {
                Hemisphere::South
            } else {
                Hemisphere::North
            };

            let selected_arc = if arc_hemisphere == hemisphere {
                circular_arc
            } else {
                circular_arc.complement()
            };

            OrthogonalArc::Arc(selected_arc)
        }
        OrthogonalArc::Diameter(line_segment) => match hemisphere {
            Hemisphere::North => {
                let a = line_segment.start();
                let b = line_segment.end();
                let complement = DoubleRay::from_points(a, b)?;
                OrthogonalArc::DiameterOutside(complement)
            }
            Hemisphere::South => OrthogonalArc::Diameter(line_segment),
        },
        OrthogonalArc::DiameterOutside(_) => unreachable!(),
    };

    Ok(adjusted_arc)
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::{geometry::Line, unit_complex::UnitComplex, Complex};

    use super::*;
    use test_case::test_case;

    type TestResult = Result<(), Box<dyn Error>>;

    #[test]
    pub fn circle_on_line_with_duplicate_point_returns_error() {
        let result = circle_on_line(1, 1);

        assert!(matches!(result, Err(IntegerArcError::DuplicateInt(_))))
    }

    #[test_case(1, 2, Circle::new((1.5).into(), 0.5); "a less than b")]
    #[test_case(2, 1, Circle::new((1.5).into(), 0.5); "b less than a")]
    #[test_case(-4, 2, Circle::new((-1.0).into(), 3.0); "negative coords")]
    pub fn circle_on_line_computes_circle(a: i64, b: i64, expected: Circle) {
        let result = circle_on_line(a, b).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    pub fn arc_on_line_by_direction_with_duplicate_input_returns_error() {
        let result = arc_on_line_by_direction(0, 0, ArcDirection::Clockwise);

        assert!(matches!(result, Err(IntegerArcError::DuplicateInt(_))))
    }

    #[test_case(1, 2, ArcDirection::Counterclockwise, PI, 2.0 * PI; "a lt b, ccw")]
    #[test_case(2, 1, ArcDirection::Counterclockwise, 0.0, PI; "a gt b, ccw")]
    #[test_case(1, 2, ArcDirection::Clockwise, PI, 0.0; "a lt b, cw")]
    #[test_case(2, 1, ArcDirection::Clockwise, 0.0, -PI; "a gt b, cw")]
    pub fn arc_on_line_by_direction_computes_correct_arc(
        a: i64,
        b: i64,
        direction: ArcDirection,
        angle_a: f64,
        angle_b: f64,
    ) {
        let result = arc_on_line_by_direction(a, b, direction).unwrap();

        let expected_circle = Circle::new(Complex::new(1.5, 0.0), 0.5);
        let expected_angles = ArcAngles::new(angle_a, angle_b).unwrap();
        let expected = CircularArc::new(expected_circle, expected_angles);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn arc_on_line_by_direction_swapping_integers_returns_complement(
    ) -> Result<(), IntegerArcError> {
        let a = 3;
        let b = 2;
        let direction = ArcDirection::Counterclockwise;

        let ab = arc_on_line_by_direction(a, b, direction)?;
        let ba = arc_on_line_by_direction(b, a, direction)?;
        let ab_complement = ab.complement();

        assert_eq!(ba, ab_complement);
        Ok(())
    }

    #[test]
    pub fn arc_on_line_by_hemisphere_with_duplicate_input_returns_error() {
        let result = arc_on_line_by_hemisphere(0, 0, Hemisphere::North);

        assert!(matches!(result, Err(IntegerArcError::DuplicateInt(_))))
    }

    #[test_case(1, 2, Hemisphere::North, PI, 0.0; "a lt b, north")]
    #[test_case(2, 1, Hemisphere::North, 0.0, PI; "a gt b, north")]
    #[test_case(1, 2, Hemisphere::South, PI, 2.0 * PI; "a lt b, south")]
    #[test_case(2, 1, Hemisphere::South, 0.0, -PI; "a gt b, south")]
    pub fn arc_on_line_by_hemisphere_computes_correct_arc(
        a: i64,
        b: i64,
        hemisphere: Hemisphere,
        angle_a: f64,
        angle_b: f64,
    ) {
        let result = arc_on_line_by_hemisphere(a, b, hemisphere).unwrap();

        let expected_circle = Circle::new(Complex::new(1.5, 0.0), 0.5);
        let expected_angles = ArcAngles::new(angle_a, angle_b).unwrap();
        let expected = CircularArc::new(expected_circle, expected_angles);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn arc_on_line_by_hemisphere_swapping_integers_returns_reverse(
    ) -> Result<(), IntegerArcError> {
        let a = 3;
        let b = 2;
        let hemisphere = Hemisphere::South;

        let ab = arc_on_line_by_hemisphere(a, b, hemisphere)?;
        let ba = arc_on_line_by_hemisphere(b, a, hemisphere)?;
        let ab_complement = ab.reverse();

        assert_eq!(ba, ab_complement);
        Ok(())
    }

    #[test]
    pub fn circle_on_circle_with_n_zero_returns_error() {
        let result = circle_on_circle(1, 2, 0);

        assert!(matches!(result, Err(IntegerArcError::ZeroPoints)))
    }

    #[test_case(10, 3; "a out of range")]
    #[test_case(3, 10; "b out of range")]
    pub fn circle_on_circle_with_value_out_of_range_returns_error(a: usize, b: usize) {
        let result = circle_on_circle(a, b, 5);

        assert!(matches!(
            result,
            Err(IntegerArcError::ValueOutOfRange(_, _))
        ))
    }

    #[test]
    pub fn circle_on_circle_with_duplicate_points_returns_error() {
        let result = circle_on_circle(0, 0, 5);

        assert!(matches!(result, Err(IntegerArcError::DuplicateInt(_))))
    }

    #[test]
    pub fn circle_on_circle_computes_correct_circle() {
        let result = circle_on_circle(1, 2, 4).unwrap();

        let expected_circle = Circle::new(Complex::new(-1.0, 1.0), 1.0);
        match result {
            GeneralizedCircle::Circle(circle) => assert_eq!(circle, expected_circle),
            _ => panic!("expected circle, got a line!"),
        }
    }

    #[test]
    pub fn circle_on_circle_with_opposite_points_computes_line() -> TestResult {
        let result = circle_on_circle(2, 8, 12)?;

        let expected_line = Line::new(UnitComplex::from_angle(5.0 * PI / 6.0), 0.0)?;
        match result {
            GeneralizedCircle::Line(line) => assert_eq!(line, expected_line),
            _ => panic!("expected line, got a circle!"),
        }

        Ok(())
    }
}

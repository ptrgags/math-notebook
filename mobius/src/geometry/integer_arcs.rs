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

#[derive(PartialEq)]
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

fn cyclotomic_angles(a: i64, b: i64, n: usize) -> Result<ArcAngles, IntegerArcError> {
    if n == 0 {
        return Err(IntegerArcError::ZeroPoints);
    }

    let n = n as i64;
    if a >= n {
        return Err(IntegerArcError::ValueOutOfRange("a".into(), a));
    }

    if b >= n {
        return Err(IntegerArcError::ValueOutOfRange("b".into(), b));
    }

    if a == b {
        return Err(IntegerArcError::DuplicateInt(a));
    }
    let a = a as f64;
    let b = b as f64;

    let step_size = TAU / (n as f64);
    Ok(ArcAngles::new(a * step_size, b * step_size)?)
}

pub fn circle_on_circle(a: i64, b: i64, n: usize) -> Result<GeneralizedCircle, IntegerArcError> {
    let angles = cyclotomic_angles(a, b, n)?;
    Ok(compute_orthogonal_circle(Circle::unit_circle(), angles))
}

pub fn arc_on_circle_by_direction(
    a: i64,
    b: i64,
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
    a: i64,
    b: i64,
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
    use crate::Complex;

    use super::*;
    use test_case::test_case;

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
}

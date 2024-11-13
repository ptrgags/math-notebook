use std::f64::consts::{PI, TAU};

use thiserror::Error;

use crate::{
    geometry::{ArcAngles, ArcAnglesError, ArcDirection, Circle, CircularArc, GeneralizedCircle},
    orthogonal_arcs::{compute_orthogonal_arc, compute_orthogonal_circle},
};

#[derive(Debug, Error)]
pub enum IntegerArcError {
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
pub fn integer_circle(a: i64, b: i64) -> Result<Circle, IntegerArcError> {
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

pub fn integer_arc_by_direction(
    a: i64,
    b: i64,
    direction: ArcDirection,
) -> Result<CircularArc, IntegerArcError> {
    let circle = integer_circle(a, b)?;

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

pub fn integer_arc_by_hemisphere(
    a: i64,
    b: i64,
    hemisphere: Hemisphere,
) -> Result<CircularArc, IntegerArcError> {
    let circle = integer_circle(a, b)?;

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

pub fn cyclotomic_circle(a: i64, b: i64, n: usize) -> Result<GeneralizedCircle, IntegerArcError> {
    let angles = cyclotomic_angles(a, b, n)?;
    Ok(compute_orthogonal_circle(Circle::unit_circle(), angles))
}

pub fn cyclotomic_arc_by_direction(
    a: i64,
    b: i64,
    n: usize,
    direction: ArcDirection,
) -> Result<CircularArc, IntegerArcError> {
    let angles = cyclotomic_angles(a, b, n)?;
    let arc = CircularArc::new(Circle::unit_circle(), angles);
    let orthog_arc = compute_orthogonal_arc(arc);

    if orthog_arc.direction() == direction {
        Ok(orthog_arc)
    } else {
        Ok(orthog_arc.complement())
    }
}

pub fn cyclotomic_arc_by_hemisphere(
    a: i64,
    b: i64,
    n: usize,
    hemisphere: Hemisphere,
) -> Result<CircularArc, IntegerArcError> {
    let angles = cyclotomic_angles(a, b, n)?;
    let arc = CircularArc::new(Circle::unit_circle(), angles);
    let orthog_arc = compute_orthogonal_arc(arc);

    let unit_circle = Circle::unit_circle();
    let inside_circle = unit_circle.point_inside(orthog_arc.interpolate(0.5));
    let arc_hemisphere = if inside_circle {
        Hemisphere::South
    } else {
        Hemisphere::North
    };

    if arc_hemisphere == hemisphere {
        Ok(orthog_arc)
    } else {
        Ok(orthog_arc.complement())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test]
    pub fn integer_circle_with_duplicate_point_returns_error() {
        let result = integer_circle(1, 1);

        assert!(matches!(result, Err(IntegerArcError::DuplicateInt(_))))
    }

    #[test_case(1, 2, Circle::new((1.5).into(), 0.5); "a less than b")]
    #[test_case(2, 1, Circle::new((1.5).into(), 0.5); "b less than a")]
    #[test_case(-4, 2, Circle::new((-1.0).into(), 3.0); "negative coords")]
    pub fn integer_circle_computes_circle(a: i64, b: i64, expected: Circle) {
        let result = integer_circle(a, b).unwrap();

        assert_eq!(result, expected);
    }
}

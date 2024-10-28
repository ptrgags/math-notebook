use std::f64::consts::{FRAC_PI_2, PI, TAU};

use crate::{geometry::{Circle, CircularArc, LineSegment}, transformable::ClineArcTile, Complex};

pub fn ghost() -> ClineArcTile {
    const SIDE_HEIGHT: f64 = 1.5;
    const CIRCLE_SPACING: f64 = 2.0 / 5.0;
    const BOTTOM_CIRCLE_RADIUS: f64 = 1.0 / 5.0;
    let head_circle = Circle::unit_circle();
    let bottom_circles: Vec<Circle> = (0..5).map(|i|
        Circle::new(Complex::new((-2.0 + i as f64) * CIRCLE_SPACING, -SIDE_HEIGHT), BOTTOM_CIRCLE_RADIUS)
    ).collect();

    let left_eye = Circle::new(Complex::new(-0.5, 0.0), 0.25);
    let right_eye = Circle::new(Complex::new(0.5, 0.0), 0.25);
    let mouth = Circle::new(Complex::new(0.0, -0.5), 0.125);

    ClineArcTile::new(vec![
        // top of ghost head is a semi-circle
        CircularArc::new(head_circle, 0.0, FRAC_PI_2, PI).into(),
        // Left side
        LineSegment::new(-Complex::ONE, Complex::new(-1.0, -SIDE_HEIGHT)).into(),
        // Five semi-circles for the bottom
        CircularArc::new(
            bottom_circles[0],
            PI,
            3.0 * FRAC_PI_2,
            TAU,
        )
        .into(),
        CircularArc::new(
            bottom_circles[1],
            PI,
            FRAC_PI_2,
            0.0,
        )
        .into(),
        CircularArc::new(
            bottom_circles[2],
            PI,
            3.0 * FRAC_PI_2,
            TAU,
        )
        .into(),
        CircularArc::new(
            bottom_circles[3],
            PI,
            FRAC_PI_2,
            0.0,
        )
        .into(),
        CircularArc::new(
            bottom_circles[4],
            PI,
            3.0 * FRAC_PI_2,
            TAU,
        )
        .into(),
        // Right side
        LineSegment::new(Complex::new(1.0, -SIDE_HEIGHT), Complex::ONE).into(),

        // Eyes and mouths are circles drawn as two semicircles
        CircularArc::new(left_eye, 0.0, FRAC_PI_2, PI).into(),
        CircularArc::new(left_eye, PI, 3.0 * FRAC_PI_2, TAU).into(),
        CircularArc::new(right_eye, 0.0, FRAC_PI_2, PI).into(),
        CircularArc::new(right_eye, PI, 3.0 * FRAC_PI_2, TAU).into(),
        CircularArc::new(mouth, 0.0, FRAC_PI_2, PI).into(),
        CircularArc::new(mouth, PI, 3.0 * FRAC_PI_2, TAU).into(),
    ])
}
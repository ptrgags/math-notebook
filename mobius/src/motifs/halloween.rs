use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_6, PI, TAU};

use crate::{
    geometry::{Circle, CircularArc, LineSegment},
    rendering::Style,
    transformable::{ClineArcTile, Motif},
    Complex,
};

pub fn ghost() -> (ClineArcTile, Style) {
    const SIDE_HEIGHT: f64 = 1.5;
    const CIRCLE_SPACING: f64 = 2.0 / 5.0;
    const BOTTOM_CIRCLE_RADIUS: f64 = 1.0 / 5.0;
    let head_circle = Circle::unit_circle();
    let bottom_circles: Vec<Circle> = (0..5)
        .map(|i| {
            Circle::new(
                Complex::new((-2.0 + i as f64) * CIRCLE_SPACING, -SIDE_HEIGHT),
                BOTTOM_CIRCLE_RADIUS,
            )
        })
        .collect();

    let left_eye = Circle::new(Complex::new(-0.5, 0.0), 0.25);
    let right_eye = Circle::new(Complex::new(0.5, 0.0), 0.25);
    let mouth = Circle::new(Complex::new(0.0, -0.5), 0.125);

    let ghost = ClineArcTile::new(vec![
        // top of ghost head is a semi-circle
        CircularArc::new(head_circle, 0.0, FRAC_PI_2, PI).into(),
        // Left side
        LineSegment::new(-Complex::ONE, Complex::new(-1.0, -SIDE_HEIGHT)).into(),
        // Five semi-circles for the bottom
        CircularArc::new(bottom_circles[0], PI, 3.0 * FRAC_PI_2, TAU).into(),
        CircularArc::new(bottom_circles[1], PI, FRAC_PI_2, 0.0).into(),
        CircularArc::new(bottom_circles[2], PI, 3.0 * FRAC_PI_2, TAU).into(),
        CircularArc::new(bottom_circles[3], PI, FRAC_PI_2, 0.0).into(),
        CircularArc::new(bottom_circles[4], PI, 3.0 * FRAC_PI_2, TAU).into(),
        // Right side
        LineSegment::new(Complex::new(1.0, -SIDE_HEIGHT), Complex::ONE).into(),
        // Eyes and mouths are circles drawn as two semicircles
        CircularArc::new(left_eye, 0.0, FRAC_PI_2, PI).into(),
        CircularArc::new(left_eye, PI, 3.0 * FRAC_PI_2, TAU).into(),
        CircularArc::new(right_eye, 0.0, FRAC_PI_2, PI).into(),
        CircularArc::new(right_eye, PI, 3.0 * FRAC_PI_2, TAU).into(),
        CircularArc::new(mouth, 0.0, FRAC_PI_2, PI).into(),
        CircularArc::new(mouth, PI, 3.0 * FRAC_PI_2, TAU).into(),
    ]);

    let style = Style::stroke(0xc5, 0xf2, 0xfa).with_width(0.25);

    (ghost, style)
}

fn lerp(a: Complex, b: Complex, t: f64) -> Complex {
    a * (1.0 - t).into() + b * t.into()
}

/// Make a candy corn motif
/// See my [desmos sketch](https://www.desmos.com/calculator/4gidpmhf7c) for
/// an illustration
pub fn candy_corn() -> (Motif, Vec<Style>) {
    // Start with a triangle at the third roots of unity
    let a = Complex::ONE;
    let b = Complex::from_polar(1.0, 2.0 * FRAC_PI_3);
    let c = Complex::from_polar(1.0, 4.0 * FRAC_PI_3);

    // Get points 1/3 and 2/3 along the sides to mark where
    // the colors transition
    let ab1 = lerp(a, b, 1.0 / 3.0);
    let ab2 = lerp(a, b, 2.0 / 3.0);
    let ac1 = lerp(a, c, 1.0 / 3.0);
    let ac2 = lerp(a, c, 2.0 / 3.0);

    // We want to round the corners, so compute points
    // a little inside from the corners.
    let radius = 0.1;
    let scale_factor = Complex::from(1.0 - 2.0 * radius);
    let center_a = scale_factor * a;
    let center_b = scale_factor * b;
    let center_c = scale_factor * c;

    // Compute the six intersection points between the round circles
    // and the sides. Since the original triangle is equilateral,
    // these are just offsets from the rounding circle centers at
    // 60 degrees + a multiple of 120 degrees.
    let offset_ab = Complex::from_polar(radius, FRAC_PI_3);
    let offset_bc = Complex::from(-radius);
    let offset_ca = Complex::from_polar(radius, -FRAC_PI_3);
    let isx_a_ab = center_a + offset_ab;
    let isx_b_ab = center_b + offset_ab;
    let isx_b_bc = center_b + offset_bc;
    let isx_c_bc = center_c + offset_bc;
    let isx_c_ca = center_c + offset_ca;
    let isx_a_ca = center_a + offset_ca;

    // Compute the line segments in counterclockwise order
    let line_a_ab1 = LineSegment::new(isx_a_ab, ab1);
    let line_ab1_ab2 = LineSegment::new(ab1, ab2);
    let line_ab2_b = LineSegment::new(ab2, isx_b_ab);
    let line_bc = LineSegment::new(isx_b_bc, isx_c_bc);
    let line_c_ac2 = LineSegment::new(isx_c_ca, ac2);
    let line_ac2_ac1 = LineSegment::new(ac2, ac1);
    let line_ac1_a = LineSegment::new(ac1, isx_a_ca);

    // Compute the arcs for the corners
    let circle_a = Circle::new(center_a, radius);
    let circle_b = Circle::new(center_b, radius);
    let circle_c = Circle::new(center_c, radius);
    let arc_a = CircularArc::new(circle_a, -FRAC_PI_3, 0.0, FRAC_PI_3);
    let arc_b = CircularArc::new(circle_b, FRAC_PI_3, 2.0 * FRAC_PI_3, PI);
    let arc_c = CircularArc::new(circle_c, -PI, -2.0 * FRAC_PI_3, -FRAC_PI_3);

    // Compute arcs dividing the 3 parts of the candy corn
    let len_ab = (b - a).mag();
    let radius1 = len_ab / 3.0;
    let radius2 = 2.0 * radius1;
    let circle1 = Circle::new(a, radius1);
    let circle2 = Circle::new(a, radius2);
    let arc1 = CircularArc::new(circle1, 5.0 * FRAC_PI_6, PI, 7.0 * FRAC_PI_6);
    let arc2 = CircularArc::new(circle2, 5.0 * FRAC_PI_6, PI, 7.0 * FRAC_PI_6);

    // Build the three parts of the candy corn
    let base = ClineArcTile::new(vec![
        line_ab2_b.into(),
        arc_b.into(),
        line_bc.into(),
        arc_c.into(),
        line_c_ac2.into(),
        arc2.into(),
    ]);
    let mid = ClineArcTile::new(vec![
        line_ab1_ab2.into(),
        arc2.into(),
        line_ac2_ac1.into(),
        arc1.into(),
    ]);
    let tip = ClineArcTile::new(vec![
        arc_a.into(),
        line_a_ab1.into(),
        arc1.into(),
        line_ac1_a.into(),
    ]);

    let styles = vec![
        // Yellow base
        Style::stroke(255, 255, 0).with_width(0.25),
        // Orange middle
        Style::stroke(255, 127, 0).with_width(0.25),
        // White tip
        Style::stroke(255, 255, 255).with_width(0.25),
    ];

    let candy_corn = Motif::new(vec![(base, 0), (mid, 1), (tip, 2)]);

    (candy_corn, styles)
}

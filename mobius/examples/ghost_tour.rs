use std::f64::consts::{FRAC_PI_2, PI, TAU};

use mobius::{
    cline::Cline,
    cline_arc::ClineArc,
    cline_tile::{ClineArcTile, ClineTile},
    svg_plot::{add_geometry, flip_y, make_card, style_lines, svg_cline_arc_tile, svg_cline_tile},
    Complex,
};

pub fn main() {
    const SIDE_HEIGHT: f64 = 1.5;
    const CIRCLE_SPACING: f64 = 2.0 / 5.0;
    const BOTTOM_CIRCLE_RADIUS: f64 = 1.0 / 5.0;
    let body = ClineArcTile::new(vec![
        // top of ghost head is a semi-circle
        ClineArc::from_circle_and_angles(Complex::Zero, 1.0, 0.0, FRAC_PI_2, PI),
        // Left side
        ClineArc::line_segment(-Complex::ONE, Complex::new(-1.0, -SIDE_HEIGHT)),
        // Five semi-circles for the bottom
        ClineArc::from_circle_and_angles(
            Complex::new(-2.0 * CIRCLE_SPACING, -SIDE_HEIGHT),
            BOTTOM_CIRCLE_RADIUS,
            PI,
            3.0 * FRAC_PI_2,
            TAU,
        ),
        ClineArc::from_circle_and_angles(
            Complex::new(-1.0 * CIRCLE_SPACING, -SIDE_HEIGHT),
            BOTTOM_CIRCLE_RADIUS,
            PI,
            FRAC_PI_2,
            0.0,
        ),
        ClineArc::from_circle_and_angles(
            Complex::new(0.0, -SIDE_HEIGHT),
            BOTTOM_CIRCLE_RADIUS,
            PI,
            3.0 * FRAC_PI_2,
            TAU,
        ),
        ClineArc::from_circle_and_angles(
            Complex::new(1.0 * CIRCLE_SPACING, -SIDE_HEIGHT),
            BOTTOM_CIRCLE_RADIUS,
            PI,
            FRAC_PI_2,
            0.0,
        ),
        ClineArc::from_circle_and_angles(
            Complex::new(2.0 * CIRCLE_SPACING, -SIDE_HEIGHT),
            BOTTOM_CIRCLE_RADIUS,
            PI,
            3.0 * FRAC_PI_2,
            TAU,
        ),
        // Right side
        ClineArc::line_segment(Complex::new(1.0, -SIDE_HEIGHT), Complex::ONE),
    ]);

    let eyes_and_mouth = ClineTile::new(vec![
        // Left eye
        Cline::circle(Complex::new(-0.5, 0.0), 0.25),
        // Right eye
        Cline::circle(Complex::new(0.5, 0.0), 0.25),
        // Mouth, a little smaller
        Cline::circle(Complex::new(0.0, -0.5), 0.125),
    ]);

    let mut ghost = style_lines("#c5f2fa", "0.25%");
    ghost = add_geometry(ghost, svg_cline_arc_tile(&body));
    ghost = add_geometry(ghost, svg_cline_tile(&eyes_and_mouth));

    let flipped = flip_y().add(ghost);
    let doc = make_card(Complex::new(0.0, -0.5), 1.5).add(flipped);
    svg::save("output/ghosty.svg", &doc).unwrap();
}

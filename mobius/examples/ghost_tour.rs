use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, PI, TAU};

use mobius::{
    cline::Cline,
    cline_arc::ClineArc,
    cline_tile::{ClineArcTile, ClineTile},
    iterated_function_system::IFS,
    rotation, scale,
    svg_plot::{
        add_geometry, flip_y, make_axes, make_card, style_lines, svg_cline_arc_tile,
        svg_cline_arc_tiles, svg_cline_tile, svg_cline_tiles,
    },
    translation, Complex, Mobius,
};

/// Create a ghost-shaped tile as circular arcs and lines. It spans between [-1, 1] horizontally and between
/// [-1.7, 1] vertically
pub fn make_ghost_parts() -> (ClineArcTile, ClineTile) {
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

    (body, eyes_and_mouth)
}

pub fn main() {
    let (body, face) = make_ghost_parts();

    let mut ghost = style_lines("#c5f2fa", "0.25%");
    ghost = add_geometry(ghost, svg_cline_arc_tile(&body));
    ghost = add_geometry(ghost, svg_cline_tile(&face));

    // Basic ghost
    let flipped = flip_y().add(ghost);
    let doc = make_card(Complex::new(0.0, -0.5), 1.5).add(flipped);
    svg::save("output/ghosty.svg", &doc).unwrap();

    // Create a transform with a spiral sink at +3
    let translate3 = translation(3.0.into()).unwrap();
    let spiral_in = rotation(-FRAC_PI_3).unwrap() * scale(0.6).unwrap();
    let drain = Mobius::sandwich(translate3, spiral_in);
    let ifs = IFS::new(vec![drain]);

    let drained_bodies: Vec<ClineArcTile> = ifs
        .dfs(20)
        .map(|(_, xform)| body.transform(xform))
        .collect();

    let drained_faces: Vec<ClineTile> = ifs
        .dfs(20)
        .map(|(_, xform)| face.transform(xform))
        .collect();

    let mut down_the_drain = style_lines("#c5f2fa", "0.25%");
    down_the_drain = add_geometry(down_the_drain, svg_cline_arc_tiles(&drained_bodies));
    down_the_drain = add_geometry(down_the_drain, svg_cline_tiles(&drained_faces));

    let flipped = flip_y().add(down_the_drain);
    let doc = make_card(Complex::new(1.5, 0.0), 2.5).add(flipped);
    svg::save("output/ghost_down_drain.svg", &doc).unwrap();
}

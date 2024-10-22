use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, PI, TAU};

use mobius::{
    cline::Cline,
    cline_arc::ClineArc,
    cline_tile::{ClineArcTile, ClineTile},
    iterated_function_system::IFS,
    map_triple, rotation, scale,
    svg_plot::{add_geometry, flip_y, make_card, style_lines},
    translation, Complex, Mobius,
};
use svg::node::element::Group;

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

struct Ghost {
    // Arcs and line segments representing the ghost's ethereal body
    body: ClineArcTile,
    // Spooky facial details realized as circles
    face: ClineTile,
}

impl Ghost {
    pub fn new() -> Self {
        let (body, face) = make_ghost_parts();
        Self { body, face }
    }

    pub fn transform(&self, mobius: Mobius) -> Self {
        Self {
            body: self.body.transform(mobius),
            face: self.face.transform(mobius),
        }
    }

    pub fn render_svg(&self) -> Group {
        let mut svg = style_lines("#c5f2fa", "0.25%");
        svg = add_geometry(svg, &self.body);
        svg = add_geometry(svg, &self.face);

        svg
    }

    pub fn render_ifs(&self, ifs: &IFS, depth: usize) -> Group {
        let transformed_bodies: Vec<ClineArcTile> = ifs
            .dfs(depth)
            .map(|(_, xform)| self.body.transform(xform))
            .collect();

        let transformed_faces: Vec<ClineTile> = ifs
            .dfs(depth)
            .map(|(_, xform)| self.face.transform(xform))
            .collect();

        let mut svg = style_lines("#c5f2fa", "0.125%");
        svg = add_geometry(svg, &transformed_bodies[..]);
        svg = add_geometry(svg, &transformed_faces[..]);

        svg
    }
}

pub fn main() {
    let ghost = Ghost::new();

    // Show the ghost by themself -----------------------------------
    let flipped = flip_y().add(ghost.render_svg());
    let doc = make_card(Complex::new(0.0, -0.5), 1.5).add(flipped);
    svg::save("output/ghosty.svg", &doc).unwrap();

    // Oh no! the ghost fell down the drain! ----------------------

    // Create a transform with a spiral sink at +3
    let translate3 = translation(3.0.into()).unwrap();
    let spiral_in = rotation(-FRAC_PI_3).unwrap() * scale(0.6).unwrap();
    let drain = Mobius::sandwich(translate3, spiral_in);
    let ifs = IFS::new(vec![drain]);
    let down_the_drain = ghost.render_ifs(&ifs, 20);

    let flipped = flip_y().add(down_the_drain);
    let doc = make_card(Complex::new(1.5, 0.0), 2.5).add(flipped);
    svg::save("output/ghost_down_drain.svg", &doc).unwrap();

    // Caught between parabolic transforms that have fixed points at -1 and
    // 1, and map the opposite point on the unit circle 90 degrees around the
    // circle. This would benefit from skipping inverses, parts of this
    // diagram will be rendered many times.
    // -------------------------------
    let shrink = scale(0.125).unwrap();
    let small_ghost = ghost.transform(shrink);
    let left_parabolic = map_triple(
        (-Complex::ONE, Complex::ONE, -Complex::I),
        (-Complex::ONE, Complex::I, Complex::ONE),
    )
    .unwrap();
    let right_parabolic = map_triple(
        (Complex::ONE, -Complex::ONE, -Complex::I),
        (Complex::ONE, Complex::I, -Complex::ONE),
    )
    .unwrap();
    // Sanity check
    println!("{:?}", left_parabolic.classify());
    println!("{:?}", right_parabolic.classify());
    let ifs = IFS::new(vec![
        left_parabolic,
        //left_parabolic.inverse(),
        right_parabolic,
        //right_parabolic.inverse(),
    ]);
    let parabolic_walk = small_ghost.render_ifs(&ifs, 5);

    let flipped = flip_y().add(parabolic_walk);
    let doc = make_card(Complex::new(0.0, 0.5), 0.9).add(flipped);
    svg::save("output/ghost_parabolic.svg", &doc).unwrap();
}

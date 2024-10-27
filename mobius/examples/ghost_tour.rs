use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, PI, TAU};

use abstraction::Group;
use mobius::{
    cline::Cline,
    cline_arc::ClineArc,
    cline_tile::{ClineArcTile, ClineTile},
    elliptic,
    geometry::{Circle, LineSegment},
    iterated_function_system::IFS,
    loxodromic, map_triple, rotation, scale,
    style::Style,
    svg_plot::{add_geometry, render_views, style_group, View},
    translation, Complex, Mobius,
};
use svg::node::element::Group as SvgGroup;

/// Create a ghost-shaped tile as circular arcs and lines. It spans between [-1, 1] horizontally and between
/// [-1.7, 1] vertically
pub fn make_ghost_parts() -> (ClineArcTile, ClineTile) {
    const SIDE_HEIGHT: f64 = 1.5;
    const CIRCLE_SPACING: f64 = 2.0 / 5.0;
    const BOTTOM_CIRCLE_RADIUS: f64 = 1.0 / 5.0;
    let body = ClineArcTile::new(vec![
        // top of ghost head is a semi-circle
        ClineArc::from_circle_and_angles(Circle::unit_circle(), 0.0, FRAC_PI_2, PI),
        // Left side
        ClineArc::from_line_segment(LineSegment::new(
            -Complex::ONE,
            Complex::new(-1.0, -SIDE_HEIGHT),
        ))
        .unwrap(),
        // Five semi-circles for the bottom
        ClineArc::from_circle_and_angles(
            Circle::new(
                Complex::new(-2.0 * CIRCLE_SPACING, -SIDE_HEIGHT),
                BOTTOM_CIRCLE_RADIUS,
            ),
            PI,
            3.0 * FRAC_PI_2,
            TAU,
        ),
        ClineArc::from_circle_and_angles(
            Circle::new(
                Complex::new(-1.0 * CIRCLE_SPACING, -SIDE_HEIGHT),
                BOTTOM_CIRCLE_RADIUS,
            ),
            PI,
            FRAC_PI_2,
            0.0,
        ),
        ClineArc::from_circle_and_angles(
            Circle::new(Complex::new(0.0, -SIDE_HEIGHT), BOTTOM_CIRCLE_RADIUS),
            PI,
            3.0 * FRAC_PI_2,
            TAU,
        ),
        ClineArc::from_circle_and_angles(
            Circle::new(
                Complex::new(1.0 * CIRCLE_SPACING, -SIDE_HEIGHT),
                BOTTOM_CIRCLE_RADIUS,
            ),
            PI,
            FRAC_PI_2,
            0.0,
        ),
        ClineArc::from_circle_and_angles(
            Circle::new(
                Complex::new(2.0 * CIRCLE_SPACING, -SIDE_HEIGHT),
                BOTTOM_CIRCLE_RADIUS,
            ),
            PI,
            3.0 * FRAC_PI_2,
            TAU,
        ),
        // Right side
        ClineArc::from_line_segment(LineSegment::new(
            Complex::new(1.0, -SIDE_HEIGHT),
            Complex::ONE,
        ))
        .unwrap(),
    ]);

    let eyes_and_mouth = ClineTile::new(vec![
        // Left eye
        Circle::new(Complex::new(-0.5, 0.0), 0.25).into(),
        // Right eye
        Circle::new(Complex::new(0.5, 0.0), 0.25).into(),
        // Mouth, a little smaller
        Circle::new(Complex::new(0.0, -0.5), 0.125).into(),
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

    pub fn render_svg(&self) -> SvgGroup {
        let mut svg = style_group(Style::stroke(0xc5, 0xf2, 0xfa).with_width(0.25));
        svg = add_geometry(svg, &self.body);
        svg = add_geometry(svg, &self.face);

        svg
    }

    pub fn render_ifs(&self, ifs: &IFS, depth: usize) -> SvgGroup {
        let transformed_bodies: Vec<ClineArcTile> = ifs
            .dfs(depth)
            .map(|(_, xform)| self.body.transform(xform))
            .collect();

        let transformed_faces: Vec<ClineTile> = ifs
            .dfs(depth)
            .map(|(_, xform)| self.face.transform(xform))
            .collect();

        let mut svg = style_group(Style::stroke(0xc5, 0xf2, 0xfa).with_width(0.25));
        svg = add_geometry(svg, &transformed_bodies[..]);
        svg = add_geometry(svg, &transformed_faces[..]);

        svg
    }
}

pub fn main() -> Result<(), std::io::Error> {
    let ghost = Ghost::new();

    // Show the ghost by themself -----------------------------------
    render_views(
        "output",
        "ghosty",
        &[View("", 0.0, -0.5, 2.5)],
        ghost.render_svg(),
    )?;

    // Oh no! the ghost fell down the drain! ----------------------

    // Create a transform with a spiral sink at +3
    let translate3 = translation(3.0.into()).unwrap();
    let spiral_in = rotation(-FRAC_PI_3).unwrap() * scale(0.6).unwrap();
    let drain = Mobius::sandwich(translate3, spiral_in);
    let ifs = IFS::new(vec![drain]);
    let down_the_drain = ghost.render_ifs(&ifs, 20);

    render_views(
        "output",
        "ghost_down_drain",
        &[View("", 1.5, 0.0, 2.5)],
        down_the_drain,
    )?;

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
    let parabolic_walk = small_ghost.render_ifs(&ifs, 6);

    render_views(
        "output",
        "ghost_parabolic",
        &[View("", 0.0, 0.5, 0.9), View("zoom_in", -0.5, 0.6, 0.2)],
        parabolic_walk,
    )?;

    // A loxodromic double spiral. Though instead of going from -1 to 1,
    // I want it from -i to i, so conjugate by a rotate
    let double_spiral = loxodromic(Complex::new(1.5, 1.1)).unwrap();
    let rotate90 = rotation(FRAC_PI_2).unwrap();
    let vertical_spiral = Mobius::sandwich(rotate90, double_spiral);
    let ifs = IFS::new(vec![vertical_spiral, vertical_spiral.inverse()]);
    let double_spiral_walk = small_ghost.render_ifs(&ifs, 10);
    render_views(
        "output",
        "ghost_double_spiral",
        &[View("", 0.0, 0.0, 1.0), View("sink", -0.125, 0.75, 0.5)],
        double_spiral_walk,
    )?;

    // Two 90 degree elliptic rotations 90 degrees apart. This is isomorphic
    // to the rotation symmetry group of the cube/octahedron
    let swirl = elliptic(FRAC_PI_2).unwrap();
    let swirl2 = Mobius::sandwich(rotate90, swirl);
    let to_the_left = translation(Complex::new(-0.5, 0.0)).unwrap();
    let ifs = IFS::new(vec![swirl, swirl2]);
    let swirl_walk = small_ghost.transform(to_the_left).render_ifs(&ifs, 8);
    render_views(
        "output",
        "ghost_octahedral",
        &[View("", 0.0, 0.0, 3.0)],
        swirl_walk,
    )?;

    // But now if we make the rotation slightly different, things don't
    // quite line up. I find the result amusing.
    let swirl = elliptic(PI / 2.01).unwrap();
    let swirl2 = Mobius::sandwich(rotate90, swirl);
    let to_the_left = translation(Complex::new(-0.5, 0.0)).unwrap();
    let ifs = IFS::new(vec![swirl, swirl2]);
    let swirl_walk = small_ghost.transform(to_the_left).render_ifs(&ifs, 8);
    render_views(
        "output",
        "ghost_triggered",
        &[View("", 0.0, 0.0, 3.0)],
        swirl_walk,
    )?;

    Ok(())
}

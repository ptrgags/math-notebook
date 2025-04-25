use std::{
    error::Error,
    f64::consts::{FRAC_PI_2, PI, TAU},
};

use mobius::{
    algorithms::MonoidIFS,
    geometry::{ArcAngles, Circle, CircularArc, LineSegment},
    map_triple, scale,
    transformable::{ClineArcTile, Collection},
    Complex, Mobius,
};
use rendering::{render_svg, style::Style, Renderable, View};

fn compute_xforms() -> Vec<Mobius> {
    // Transform A just shrinks the unit circle to the circle with
    // radius 1/2
    let xform_a = scale(0.5).unwrap();

    // Transform B is a hyperbolic transformation that shrinks towards 1
    // such that:
    // B(0) -> 1/2
    // B(1) -> 1
    // B(-1) -> -1
    let xform_b = map_triple(
        (Complex::Zero, Complex::ONE, -Complex::ONE),
        ((0.5).into(), Complex::ONE, -Complex::ONE),
    )
    .unwrap();

    // Transform C is much like B, except it shrinks towards +i
    // B(0) -> 1/2i
    // B(i) -> i
    // B(-i) -> -i
    let xform_c = map_triple(
        (Complex::Zero, Complex::I, -Complex::I),
        (Complex::new(0.0, 0.5), Complex::I, -Complex::I),
    )
    .unwrap();

    vec![xform_a, xform_b, xform_c]
}

fn main() -> Result<(), Box<dyn Error>> {
    let xforms = compute_xforms();

    // ----------------------

    let angles = ArcAngles::new(0.0, PI / 2.0).unwrap();
    let tile = ClineArcTile::new(vec![
        LineSegment::new(Complex::Zero, Complex::ONE).into(),
        CircularArc::new(Circle::unit_circle(), angles).into(),
        LineSegment::new(Complex::I, Complex::Zero).into(),
    ]);

    let ifs = MonoidIFS::new(xforms);
    let tiles = ifs.apply(&tile, 0, 5);

    let yellow = Style::stroke(255, 255, 0).with_width(0.025);

    render_svg(
        "output",
        "tricorn2",
        &[View("", 0.5, 0.5, 0.6)],
        Collection::union(tiles).render_group(yellow)?,
    )?;

    // --

    let more_angles = ArcAngles::new(3.0 * FRAC_PI_2, TAU).unwrap();
    let another_tile = ClineArcTile::new(vec![
        LineSegment::new(Complex::Zero, -Complex::I).into(),
        CircularArc::new(Circle::unit_circle(), more_angles).into(),
        LineSegment::new(Complex::ONE, Complex::Zero).into(),
    ]);
    let more_tiles = ifs.apply(&another_tile, 0, 5);
    let cyan = Style::stroke(0, 255, 255).with_width(0.25);

    render_svg(
        "output",
        "tricorn2",
        &[View("", 0.5, 0.0, 0.8)],
        Collection::union(more_tiles).render_group(cyan)?,
    )?;

    Ok(())
}

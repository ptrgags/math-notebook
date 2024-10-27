use core::f64;
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, TAU};

use mobius::{
    cline_arc::ClineArc,
    cline_tile::ClineArcTile,
    geometry::{Circle, LineSegment},
    map_triple, scale,
    svg_plot::{add_geometry, flip_y, make_axes, make_card},
    transformable::Transformable,
    Complex, Mobius,
};
use svg::node::element::Group;

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

fn apply_xforms(xforms: &[Mobius], tile: &ClineArcTile) -> Vec<ClineArcTile> {
    xforms.iter().map(|x| tile.transform(*x)).collect()
}

fn iteration(xforms: &[Mobius], tiles: &[ClineArcTile]) -> Vec<ClineArcTile> {
    tiles
        .iter()
        .flat_map(|tile| apply_xforms(xforms, tile))
        .collect()
}

fn main() {
    let xforms = compute_xforms();

    let axes = make_axes()
        .set("fill", "none")
        .set("stroke", "white")
        .set("stroke-width", "0.5%");

    // ----------------------

    let tile = ClineArcTile::new(vec![
        ClineArc::from_line_segment(LineSegment::new(Complex::Zero, Complex::ONE)).unwrap(),
        ClineArc::from_circle_and_angles(
            Circle::unit_circle(),
            0.0,
            f64::consts::FRAC_PI_4,
            f64::consts::FRAC_PI_2,
        ),
        ClineArc::from_line_segment(LineSegment::new(Complex::I, Complex::Zero)).unwrap(),
    ]);

    let tiles_level1 = apply_xforms(&xforms, &tile);
    let tiles_level2 = iteration(&xforms, &tiles_level1);
    let tiles_level3 = iteration(&xforms, &tiles_level2);
    let tiles_level4 = iteration(&xforms, &tiles_level3);
    let tiles_level5 = iteration(&xforms, &tiles_level4);

    let mut geometry = Group::new()
        .set("stroke", "yellow")
        .set("stroke-width", "0.25%")
        .set("fill", "none");
    geometry = add_geometry(geometry, &tile);
    geometry = add_geometry(geometry, &tiles_level1[..]);
    geometry = add_geometry(geometry, &tiles_level2[..]);
    geometry = add_geometry(geometry, &tiles_level3[..]);
    geometry = add_geometry(geometry, &tiles_level4[..]);
    geometry = add_geometry(geometry, &tiles_level5[..]);

    let flipped2 = flip_y().add(axes.clone()).add(geometry.clone());

    let doc = make_card(Complex::new(0.5, 0.5), 0.6).add(flipped2);
    svg::save("output/tricorn.svg", &doc).unwrap();

    // --

    let another_tile = ClineArcTile::new(vec![
        ClineArc::from_line_segment(LineSegment::new(Complex::Zero, -Complex::I)).unwrap(),
        ClineArc::from_circle_and_angles(
            Circle::unit_circle(),
            3.0 * FRAC_PI_2,
            7.0 * FRAC_PI_4,
            TAU,
        ),
        ClineArc::from_line_segment(LineSegment::new(Complex::ONE, Complex::Zero)).unwrap(),
    ]);

    let tiles_level1 = apply_xforms(&xforms, &another_tile);
    let tiles_level2 = iteration(&xforms, &tiles_level1);
    let tiles_level3 = iteration(&xforms, &tiles_level2);
    let tiles_level4 = iteration(&xforms, &tiles_level3);
    let tiles_level5 = iteration(&xforms, &tiles_level4);

    let mut geometry2 = Group::new()
        .set("stroke", "cyan")
        .set("stroke-width", "0.25%")
        .set("fill", "none");
    geometry2 = add_geometry(geometry2, &tile);
    geometry2 = add_geometry(geometry2, &tiles_level1[..]);
    geometry2 = add_geometry(geometry2, &tiles_level2[..]);
    geometry2 = add_geometry(geometry2, &tiles_level3[..]);
    geometry2 = add_geometry(geometry2, &tiles_level4[..]);
    geometry2 = add_geometry(geometry2, &tiles_level5[..]);

    let flipped2 = flip_y().add(axes).add(geometry).add(geometry2);

    let doc = make_card(Complex::new(0.5, 0.0), 0.8).add(flipped2);
    svg::save("output/tricorn2.svg", &doc).unwrap();
}

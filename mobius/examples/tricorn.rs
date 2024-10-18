use core::f64;
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI, TAU};

use mobius::{
    cline::Cline,
    cline_arc::ClineArc,
    cline_tile::{ClineArcTile, ClineTile},
    map_triple, scale,
    svg_plot::{
        add_geometry, flip_y, make_axes, make_card, svg_cline_arc_tile, svg_cline_arc_tiles,
        svg_cline_tile,
    },
    Complex, Mobius,
};
use svg::{node::element::Group, Node};

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

fn iterate(xforms: &[Mobius], tile: &ClineTile, depth: u8) -> Vec<ClineTile> {
    if depth == 0 {
        return xforms.iter().map(|x| tile.transform(*x)).collect();
    }

    let mut result: Vec<ClineTile> = vec![tile.clone()];
    for xform in xforms {
        let prefixed: Vec<Mobius> = xforms.iter().map(|x| *xform * *x).collect();
        let subtree = iterate(&prefixed, tile, depth - 1);
        result.extend(subtree);
    }

    result
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

    let initial_tile = ClineTile::new(vec![
        Cline::real_axis(),
        Cline::imag_axis(),
        Cline::unit_circle(),
    ]);

    let new_tiles = iterate(&xforms[1..2], &initial_tile, 1);

    let svg_tiles: Vec<Box<dyn Node>> = new_tiles.iter().flat_map(|x| svg_cline_tile(x)).collect();

    let mut geometry = Group::new()
        .set("stroke", "yellow")
        .set("stroke-width", "0.5%")
        .set("fill", "none");
    for svg_node in svg_tiles {
        geometry = geometry.add(svg_node);
    }

    let axes = make_axes()
        .set("fill", "none")
        .set("stroke", "white")
        .set("stroke-width", "0.5%");

    // ----------------------

    let tile = ClineArcTile::new(vec![
        ClineArc::line_segment(Complex::Zero, Complex::ONE),
        ClineArc::from_circle_and_angles(
            Complex::Zero,
            1.0,
            0.0,
            f64::consts::FRAC_PI_4,
            f64::consts::FRAC_PI_2,
        ),
        ClineArc::line_segment(Complex::I, Complex::Zero),
    ]);

    let tiles_level1 = apply_xforms(&xforms, &tile);
    let tiles_level2 = iteration(&xforms, &tiles_level1);
    let tiles_level3 = iteration(&xforms, &tiles_level2);
    let tiles_level4 = iteration(&xforms, &tiles_level3);
    let tiles_level5 = iteration(&xforms, &tiles_level4);

    let svg_level1 = svg_cline_arc_tiles(&tiles_level1);
    let svg_level2 = svg_cline_arc_tiles(&tiles_level2);
    let svg_level3 = svg_cline_arc_tiles(&tiles_level3);
    let svg_level4 = svg_cline_arc_tiles(&tiles_level4);
    let svg_level5 = svg_cline_arc_tiles(&tiles_level5);

    let mut geometry = Group::new()
        .set("stroke", "yellow")
        .set("stroke-width", "0.25%")
        .set("fill", "none");
    geometry = add_geometry(geometry, svg_cline_arc_tile(&tile));
    geometry = add_geometry(geometry, svg_level1);
    geometry = add_geometry(geometry, svg_level2);
    geometry = add_geometry(geometry, svg_level3);
    geometry = add_geometry(geometry, svg_level4);
    geometry = add_geometry(geometry, svg_level5);

    let flipped2 = flip_y().add(axes.clone()).add(geometry.clone());

    let doc = make_card(Complex::new(0.5, 0.5), 0.6).add(flipped2);
    svg::save("tricorn.svg", &doc).unwrap();

    // --

    let another_tile = ClineArcTile::new(vec![
        ClineArc::line_segment(Complex::Zero, -Complex::I),
        ClineArc::from_circle_and_angles(Complex::Zero, 1.0, 3.0 * FRAC_PI_2, 7.0 * FRAC_PI_4, TAU),
        ClineArc::line_segment(Complex::ONE, Complex::Zero),
    ]);

    let tiles_level1 = apply_xforms(&xforms, &another_tile);
    let tiles_level2 = iteration(&xforms, &tiles_level1);
    let tiles_level3 = iteration(&xforms, &tiles_level2);
    let tiles_level4 = iteration(&xforms, &tiles_level3);
    let tiles_level5 = iteration(&xforms, &tiles_level4);

    let svg_level1 = svg_cline_arc_tiles(&tiles_level1);
    let svg_level2 = svg_cline_arc_tiles(&tiles_level2);
    let svg_level3 = svg_cline_arc_tiles(&tiles_level3);
    let svg_level4 = svg_cline_arc_tiles(&tiles_level4);
    let svg_level5 = svg_cline_arc_tiles(&tiles_level5);

    let mut geometry2 = Group::new()
        .set("stroke", "cyan")
        .set("stroke-width", "0.25%")
        .set("fill", "none");
    geometry2 = add_geometry(geometry2, svg_cline_arc_tile(&tile));
    geometry2 = add_geometry(geometry2, svg_level1);
    geometry2 = add_geometry(geometry2, svg_level2);
    geometry2 = add_geometry(geometry2, svg_level3);
    geometry2 = add_geometry(geometry2, svg_level4);
    geometry2 = add_geometry(geometry2, svg_level5);

    let flipped2 = flip_y().add(axes).add(geometry).add(geometry2);

    let doc = make_card(Complex::new(0.5, 0.0), 0.8).add(flipped2);
    svg::save("tricorn2.svg", &doc).unwrap();
}

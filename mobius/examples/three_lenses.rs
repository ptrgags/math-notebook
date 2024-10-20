use std::f64::consts::{FRAC_PI_2, PI};

use mobius::{
    cline_arc::ClineArc,
    cline_tile::ClineArcTile,
    iterated_function_system::{transform_tile, IFS},
    map_triple,
    svg_plot::{add_geometry, flip_y, make_card, style_lines, svg_cline_arc_tiles},
    Complex, Mobius,
};
use svg::node::element::Group;

fn make_xforms() -> Vec<Mobius> {
    // Semicircle from -1 -> 1 (straight line) then arc arc around the unit circle
    let half_circle_points = (Complex::Zero, Complex::ONE, -Complex::ONE);

    let lens_width = 1.0;
    let lens_radius = lens_width / (2.0f64).sqrt();
    let lens_half_thickness = lens_radius * (1.0 - 0.5 * (2.0f64).sqrt());

    let halfway_a = Complex::new(-0.5, -lens_half_thickness);
    let xform_a = map_triple(
        half_circle_points,
        (halfway_a, Complex::Zero, -Complex::ONE),
    )
    .unwrap();

    let halfway_b = Complex::new(0.5, -lens_half_thickness);
    let xform_b = map_triple(half_circle_points, (halfway_b, Complex::ONE, Complex::Zero)).unwrap();

    let halfway_c = Complex::new(lens_half_thickness, 0.5);
    let xform_c = map_triple(half_circle_points, (halfway_c, Complex::I, Complex::Zero)).unwrap();

    vec![xform_a, xform_b, xform_c]
}

fn show_individual_xforms(
    xforms: &[Mobius],
    colors: &[&str],
    tile: &ClineArcTile,
    min_depth: usize,
    max_depth: usize,
) -> Group {
    xforms
        .iter()
        .zip(colors.iter())
        .map(|(xform, color)| {
            let ifs = IFS::new(vec![*xform]);
            let tiles = transform_tile(&ifs, &tile, min_depth, max_depth);

            let mut geometry = style_lines(color, "0.25%");
            geometry = add_geometry(geometry, svg_cline_arc_tiles(&tiles));

            geometry
        })
        .fold(Group::new(), |group, x| group.add(x))
}

fn main() {
    let xforms = make_xforms();

    let half_circle = ClineArcTile::new(vec![
        ClineArc::line_segment(-Complex::ONE, Complex::ONE),
        ClineArc::from_circle_and_angles(Complex::Zero, 1.0, 0.0, FRAC_PI_2, PI),
    ]);

    //let rotate_90 = rotation(FRAC_PI_2).unwrap();
    //let ifs = IFS::sandwich(rotate_90, &IFS::new(xforms));
    let ifs = IFS::new(xforms.clone());

    let tiles = transform_tile(&ifs, &half_circle, 8, 8);
    let mut geometry = style_lines("red", "0.125%");
    geometry = add_geometry(geometry, svg_cline_arc_tiles(&tiles));

    let flipped = flip_y().add(geometry.clone());
    let doc = make_card(Complex::new(0.0, 0.0), 1.25).add(flipped.clone());
    svg::save("output/three_lenses.svg", &doc).unwrap();

    let zoomed = make_card(Complex::new(0.2, 0.5), 0.5).add(flipped.clone());
    svg::save("output/three_lenses_zoomed.svg", &zoomed).unwrap();

    let colors = ["yellow", "cyan", "white"];
    let highlight_xforms = show_individual_xforms(&xforms, &colors, &half_circle, 0, 10);

    let flipped = flip_y().add(geometry).add(highlight_xforms);
    let doc = make_card(Complex::new(0.0, 0.0), 1.25).add(flipped);
    svg::save("output/three_lenses_labeled.svg", &doc).unwrap();
}

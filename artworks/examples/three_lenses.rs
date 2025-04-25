use std::{error::Error, f64::consts::PI};

use mobius::{
    algorithms::MonoidIFS,
    geometry::{ArcAngles, Circle, CircularArc, LineSegment},
    map_triple,
    transformable::ClineArcTile,
    Complex, Mobius,
};
use rendering::{render_svg, style::Style, RenderPrimitive, Renderable, View};

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
    colors: &[Style],
    tile: &ClineArcTile,
    min_depth: usize,
    max_depth: usize,
) -> RenderPrimitive {
    let primitives: Vec<RenderPrimitive> = xforms
        .iter()
        .zip(colors.iter())
        .map(|(xform, style)| {
            let ifs = MonoidIFS::new(vec![*xform]);
            let tiles = ifs.flat_apply(tile, min_depth, max_depth);
            tiles.render_group(*style).unwrap()
        })
        .collect();

    RenderPrimitive::group(primitives)
}

fn main() -> Result<(), Box<dyn Error>> {
    let xforms = make_xforms();

    let angles = ArcAngles::new(0.0, PI).unwrap();
    let half_circle = ClineArcTile::new(vec![
        LineSegment::new(-Complex::ONE, Complex::ONE).into(),
        CircularArc::new(Circle::unit_circle(), angles).into(),
    ]);

    let ifs = MonoidIFS::new(xforms.clone());

    let tiles = ifs.flat_apply(&half_circle, 8, 8);
    let red = Style::stroke(255, 0, 0).with_width(0.125);
    let geometry = tiles.render_group(red)?;

    render_svg(
        "output",
        "three_lenses",
        &[View("", 0.0, 0.0, 1.25), View("zoomed", 0.2, 0.5, 0.5)],
        geometry.clone(),
    )?;

    let styles = [
        Style::stroke(255, 255, 0).with_width(0.25),
        Style::stroke(255, 0, 255).with_width(0.25),
        Style::stroke(255, 255, 255).with_width(0.25),
    ];
    let highlight_xforms = show_individual_xforms(&xforms, &styles, &half_circle, 0, 10);
    render_svg(
        "output",
        "three_lenses",
        &[View("", 0.0, 0.0, 1.25)],
        RenderPrimitive::group(vec![geometry, highlight_xforms]),
    )?;
    Ok(())
}

use std::{
    f64::consts::{FRAC_PI_2, FRAC_PI_4},
    io::Error,
};

use mobius::{
    cline_arc::ClineArc,
    cline_tile::ClineArcTile,
    geometry::{Circle, CircularArc, LineSegment},
    iterated_function_system::{apply_ifs, IFS},
    map_triple, scale,
    style::Style,
    svg_plot::{add_geometry, flip_y, make_card, render_views, style_group, View},
    Complex, Mobius,
};
use svg::node::element::Group;

fn compute_xforms() -> Vec<Mobius> {
    // Similar to the Sierpinski triangle, we want to send the overall triangle
    // to the 3 corners, and the three triangles should touch exactly at the
    // corners. To make this happen, start with a tangency point at
    //      sqrt(i) = exp(i * pi / 4)
    //
    // This is a 45 degree angle from the center of the unit circle.
    // drawing a tangent line, it meets the real axis at sqrt(2)
    // A circle with center there that goes through the tangency point will
    // have radius 1 by mirror symmetry about x=cos(pi/4).
    //
    // From there, we can compute where this second circle intersects the
    // real axis: r = sqrt(2) - 1
    //
    // See https://www.desmos.com/calculator/pcwwbwk2qr
    let radius: f64 = (2.0f64).sqrt() - 1.0;

    // The A transform is a simple shrink transform from 1 -> r
    let xform_a = scale(radius).unwrap();

    // curved triangle that bounds the first quadrant of the unit circle
    let triangle_corners = (Complex::Zero, Complex::ONE, Complex::I);

    // The B transform has a fixed point at 1 and maps the imaginary segment
    // to the arc between r and sqrt(i)
    let sqrt_i = Complex::I.sqrt();
    let xform_b = map_triple(
        triangle_corners,
        (Complex::new(radius, 0.0), Complex::ONE, sqrt_i),
    )
    .unwrap();

    // The C transform has a fixed point at i and maps the real segment
    // to the arc between ri and sqrt(i)

    let xform_c = map_triple(
        triangle_corners,
        (Complex::new(0.0, radius), sqrt_i, Complex::I),
    )
    .unwrap();

    vec![xform_a, xform_b, xform_c]
}

fn main() -> Result<(), Error> {
    let xforms = compute_xforms();
    let modified_sierpinski = IFS::new(xforms.clone());

    let tile = ClineArcTile::new(vec![
        ClineArc::from_line_segment(LineSegment::new(Complex::Zero, Complex::ONE)).unwrap(),
        CircularArc::new(Circle::unit_circle(), 0.0, FRAC_PI_4, FRAC_PI_2).into(),
        ClineArc::from_line_segment(LineSegment::new(Complex::I, Complex::Zero)).unwrap(),
    ]);

    let sierpinski_tiles = apply_ifs(&modified_sierpinski, &tile, 0, 6);

    let mut geometry = Group::new()
        .set("stroke", "orange")
        .set("stroke-width", "0.125%")
        .set("fill", "none");
    geometry = add_geometry(geometry, &sierpinski_tiles[..]);

    render_views(
        "output",
        "nacho",
        &[View("", 0.5, 0.5, 0.5001), View("zoom", 0.0, 0.0, 0.5)],
        geometry.clone(),
    )?;

    let a_only = IFS::new(vec![xforms[0]]);
    let b_only = IFS::new(vec![xforms[1]]);
    let c_only = IFS::new(vec![xforms[2]]);

    let min_depth = 1;
    let overlay_depth = 3;
    let tiles_a = apply_ifs(&a_only, &tile, min_depth, overlay_depth);
    let tiles_b = apply_ifs(&b_only, &tile, min_depth, overlay_depth);
    let tiles_c = apply_ifs(&c_only, &tile, min_depth, overlay_depth);

    let overlay_width = 0.5;
    let mut geometry_a = style_group(Style::stroke(255, 0, 255).with_width(overlay_width));
    geometry_a = add_geometry(geometry_a, &tiles_a[..]);

    let mut geometry_b = style_group(Style::stroke(255, 0, 0).with_width(overlay_width));
    geometry_b = add_geometry(geometry_b, &tiles_b[..]);

    let mut geometry_c = style_group(Style::stroke(255, 255, 255).with_width(overlay_width));
    geometry_c = add_geometry(geometry_c, &tiles_c[..]);

    let grouped = Group::new()
        .add(geometry)
        .add(geometry_a)
        .add(geometry_b)
        .add(geometry_c);

    render_views(
        "output",
        "labeled_nacho",
        &[View("", 0.5, 0.5, 0.5001)],
        grouped,
    )?;

    Ok(())
}

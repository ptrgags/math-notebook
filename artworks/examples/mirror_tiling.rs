use std::{f64::consts::PI, io::Error};

use abstraction::Monoid;
use mobius::{
    algorithms::{OrbitIFS, OrbitTile},
    cline_arc::ClineArc,
    geometry::LineSegment,
    hyperbolic_tilings::{get_fundamental_region, reflection_group},
    isogonal::Isogonal,
    quantized_hash::QuantizedHash,
    rotation, scale,
    transformable::{ClineArcTile, Transformable},
    translation, Complex,
};
use rendering::{render_svg, style::Style, View};

pub fn better_candy_corners() -> Result<(), Error> {
    let (conj, r_conj, e2_conj) = reflection_group(3, 7).unwrap();

    let complex = e2_conj * Complex::Zero;
    let dist_to_edge = 0.5 * (complex).real();
    let (corn, styles) = candy_corn();
    let (fundamental_domain, (_, _, vertex)) = get_fundamental_region(3, 7).unwrap();
    let displacement = vertex * Complex::from(0.4);
    let shift = translation(displacement).unwrap();
    let shrink = scale(dist_to_edge * 0.8).unwrap();
    let rot60 = rotation(PI / 3.0).unwrap();
    let tiny_corn = corn.transform(shift * rot60 * shrink);

    const DEPTH: usize = 6;
    const QUANTIZE_BITS: i32 = 16;

    let neighbor_xforms = vec![conj, r_conj, e2_conj];
    let representative = Complex::new(0.1, 0.25);
    let orbit_tile = OrbitTile::new(Isogonal::identity(), neighbor_xforms, representative);
    let ifs = OrbitIFS::new(orbit_tile);

    let style = Style::stroke(0, 127, 35).with_width(0.25);
    let candy_corners = ifs.apply(&fundamental_domain, DEPTH, QUANTIZE_BITS);
    render_svg(
        "output",
        "candy_corners_orbit",
        &[View("", 0.0, 0.0, 1.0), View("zoom", 0.2, 0.0, 0.4)],
        union(vec![style_geometry(style, &candy_corners[..])]),
    )?;

    Ok(())
}

fn main() -> Result<(), Error> {
    let mirror_x = Isogonal::conj();
    let r180: Isogonal = rotation(PI).unwrap().into();
    let mirror_y = r180 * mirror_x;
    let translate_x: Isogonal = translation(Complex::ONE * (2.0).into()).unwrap().into();
    let translate_y: Isogonal = translation(Complex::I * (2.0).into()).unwrap().into();

    let neighbor_tile_xforms = vec![
        translate_x * mirror_y,
        translate_y * mirror_x,
        mirror_y,
        mirror_x,
    ];

    let test_point = Complex::new(0.5, 0.5);

    let initial_tile = OrbitTile::new(Isogonal::identity(), neighbor_tile_xforms, test_point);
    let ifs = OrbitIFS::new(initial_tile);

    let flag_bottom_height = 0.6;
    let flagpole = LineSegment::new(Complex::new(0.1, 0.1), Complex::new(0.1, 0.9));
    let flag_bottom = LineSegment::new(
        Complex::new(0.1, flag_bottom_height),
        Complex::new(0.9, flag_bottom_height),
    );
    let flag_end = LineSegment::new(
        Complex::new(0.9, flag_bottom_height),
        Complex::new(0.9, 0.9),
    );
    let flag_top = LineSegment::new(Complex::new(0.1, 0.9), Complex::new(0.9, 0.9));
    let fundamental_domain = ClineArcTile::new(vec![
        ClineArc::from(flagpole),
        ClineArc::from(flag_bottom),
        ClineArc::from(flag_end),
        ClineArc::from(flag_top),
    ]);

    const DEPTH: usize = 6;
    const QUANTIZE_BITS: i32 = 16;

    let flags = ifs.apply(&fundamental_domain, DEPTH, QUANTIZE_BITS);
    let style = Style::stroke(255, 63, 63).with_width(0.5);
    let style_original = Style::stroke(255, 255, 255).with_width(0.5);

    render_svg(
        "output",
        "mirror_tiling",
        &[View("", 0.0, 0.0, 5.0)],
        union(vec![
            style_geometry(style, &flags[..]),
            style_geometry(style_original, &fundamental_domain),
        ]),
    )?;

    for (i, xform) in ifs.orbit(DEPTH, 8).enumerate() {
        let point = xform * test_point;
        println!("{}: {} -> {:?}", i, point, point.quantize(4));
    }

    better_candy_corners()?;

    Ok(())
}

use std::{f64::consts::PI, io::Error};

use abstraction::Monoid;
use mobius::{
    algorithms::{OrbitIFS, OrbitTile},
    cline_arc::ClineArc,
    geometry::LineSegment,
    isogonal::Isogonal,
    quantized_hash::QuantizedHash,
    rendering::Style,
    rotation,
    svg_plot::{render_views, style_geometry, union, View},
    transformable::ClineArcTile,
    translation, Complex,
};

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

    let flags = ifs.apply(&fundamental_domain, 3, 2);
    let style = Style::stroke(255, 63, 63).with_width(0.5);
    let style_original = Style::stroke(255, 255, 255).with_width(0.5);

    render_views(
        "output",
        "mirror_tiling",
        &[View("", 0.0, 0.0, 5.0)],
        union(vec![
            style_geometry(style, &flags[..]),
            style_geometry(style_original, &fundamental_domain),
        ]),
    )?;

    for xform in ifs.orbit(3, 8) {
        let point = xform * test_point;
        println!("{}, {}", xform, point);
        println!("{:?}", point.quantize(4));
    }

    Ok(())
}

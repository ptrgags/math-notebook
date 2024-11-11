use std::{f64::consts::PI, io::Error};

use mobius::{
    algorithms::SemigroupIFS,
    geometry::{ArcAngles, DirectedEdge},
    orthogonal_arcs::compute_orthogonal_arc,
    svg_plot::union,
    transformable::ClineArcTile,
};
use mobius::{
    geometry::{Circle, CircularArc},
    map_triple,
    rendering::Style,
    svg_plot::{render_views, style_geometry, View},
    Mobius,
};

struct ArcFractal {
    /// The original arc, a -> c
    arc: CircularArc,
    /// Two transformations (translate/rotate/scale) that shrink arc onto
    /// the two respective sub-arcs
    xforms: (Mobius, Mobius),
    /// An arc orthogonal to the first one, as I want to render this too.
    orthog_arc: CircularArc,
    /// The two sub-arcs. The first one is c -> b, the second is b -> a
    sub_arcs: (CircularArc, CircularArc),
}

impl ArcFractal {
    pub fn new(arc: CircularArc, t: f64) -> Self {
        let CircularArc { circle, angles } = arc;

        let ArcAngles(angle_a, angle_c) = angles;
        let angle_b = angles.interpolate(t);

        let angles_ab = ArcAngles::new(angle_a, angle_b).unwrap();
        let angles_bc = ArcAngles::new(angle_b, angle_c).unwrap();
        let arc_ab = CircularArc::new(circle, angles_ab);
        let arc_bc = CircularArc::new(circle, angles_bc);

        let orthog_arc_cb = compute_orthogonal_arc(arc_bc);
        let orthog_arc_ba = compute_orthogonal_arc(arc_ab);

        let d = orthog_arc_ba.interpolate(t);
        let e = orthog_arc_cb.interpolate(t);

        let a = arc.start();
        let b = arc.interpolate(t);
        let c = arc.end();
        let xform_bda = map_triple((a, b, c), (b, d, a)).unwrap();
        let xform_ceb = map_triple((a, b, c), (c, e, b)).unwrap();

        Self {
            xforms: (xform_ceb, xform_bda),
            arc,
            orthog_arc: compute_orthogonal_arc(arc),
            sub_arcs: (orthog_arc_cb, orthog_arc_ba),
        }
    }
}

fn main() -> Result<(), Error> {
    let angles = ArcAngles::new(0.0, 5.0 * PI / 4.0).unwrap();
    let arc = CircularArc::new(Circle::unit_circle(), angles);
    let fractal = ArcFractal::new(arc, 0.5);

    let (a, b) = fractal.xforms;
    let ifs = SemigroupIFS::new(vec![a, b]);

    let lens_tile = ClineArcTile::new(vec![fractal.arc.into(), fractal.orthog_arc.into()]);

    let (arc_cb, arc_ba) = fractal.sub_arcs;
    let triangle_tile = ClineArcTile::new(vec![
        // orthogonal arc from b -> a
        fractal.orthog_arc.into(),
        //
        arc_ba.reverse().into(),
        arc_cb.reverse().into(),
    ]);

    let depth = 7usize;

    let triangle_tiles = ifs.apply(&triangle_tile, 0, depth - 1);
    let leaf_lenses = ifs.apply(&lens_tile, depth, depth);

    let orange_lines = Style::stroke(255, 127, 0).with_width(0.125);
    let purple_lines = Style::stroke(127, 0, 255).with_width(0.5);

    render_views(
        "output",
        "crinkle_arc",
        &[View("", 0.0, 0.0, 1.0)],
        union(vec![
            style_geometry(orange_lines, &triangle_tiles[..]),
            style_geometry(purple_lines, &leaf_lenses[..]),
        ]),
    )?;

    Ok(())
}

use std::{f64::consts::PI, io::Error};

use mobius::{
    algorithms::SemigroupIFS,
    geometry::{ArcAngles, DirectedEdge},
    orthogonal_arcs::compute_orthogonal_arc,
    svg_plot::{style_motifs, union},
    transformable::{ClineArcTile, Motif, Transformable},
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
    let angles = ArcAngles::new(-PI / 4.0, 5.0 * PI / 4.0).unwrap();
    let arc = CircularArc::new(Circle::unit_circle(), angles);
    let fractal = ArcFractal::new(arc, 0.5);

    let (a, b) = fractal.xforms;
    let ifs = SemigroupIFS::new(vec![a, b]);

    let lens_tile = ClineArcTile::new(vec![fractal.arc.into(), fractal.orthog_arc.into()]);

    let (arc_cb, arc_ba) = fractal.sub_arcs;
    let triangle_tile = ClineArcTile::new(vec![
        // orthogonal arc from b -> a
        fractal.orthog_arc.into(),
        // orthgonal arc from a -> midpoint
        arc_ba.reverse().into(),
        // orthogonal arc from midpoint -> b
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

    // Doodling on paper, I find that alternating the colors as you iterate
    // deeper produceses a cool effect. Let's try that.
    //
    // First, compute an IFS that moves 2 iterations at a time.
    let aa = a * a;
    let ab = a * b;
    let ba = b * a;
    let bb = b * b;
    let ifs = SemigroupIFS::new(vec![aa, ab, ba, bb]);

    // Our tile will now be the big triangle tile + the first 2 children tiles (in a different color)
    let combined_tile = Motif::new(vec![
        (triangle_tile.clone(), 0),
        (triangle_tile.clone().transform(a), 1),
        (triangle_tile.clone().transform(b), 1),
    ]);

    // Remember, we're making bigger jumps and a higher branchihng factor, so tune the depth down a bit.
    let depth = 3usize;
    let tiles = ifs.apply(&combined_tile, 0, depth);
    let thin_purple = purple_lines.with_width(0.125);

    let styles = vec![orange_lines, thin_purple];
    render_views(
        "output",
        "crinkle_two_color",
        &[View("", 0.0, 0.0, 1.0)],
        style_motifs(&tiles[..], &styles),
    )?;

    Ok(())
}

use std::{error::Error, f64::consts::PI};

use mobius::{
    algorithms::MonoidIFS,
    geometry::{
        orthogonal_arcs::{compute_orthogonal_arc, OrthogonalArc},
        ArcAngles, DirectedEdge,
    },
    svg_plot::{style_motifs, union},
    transformable::{ClineArcTile, Motif, Transformable},
    Complex,
};
use mobius::{
    geometry::{Circle, CircularArc},
    map_triple,
    rendering::Style,
    svg_plot::{render_views, style_geometry, View},
    Mobius,
};
use svg::node::element::Group;

struct ArcFractal {
    /// The original arc, a -> c
    arc: CircularArc,
    /// Two transformations (translate/rotate/scale) that shrink arc onto
    /// the two respective sub-arcs
    xforms: (Mobius, Mobius),
    /// An arc orthogonal to the first one, as I want to render this too.
    orthog_arc: OrthogonalArc,
    /// The two sub-arcs. The first one is c -> b, the second is b -> a
    sub_arcs: (OrthogonalArc, OrthogonalArc),
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

        let d = match orthog_arc_ba {
            OrthogonalArc::Arc(circular_arc) => circular_arc.interpolate(t),
            OrthogonalArc::Diameter(line_segment) => line_segment.interpolate(t),
            OrthogonalArc::DiameterOutside(_) => unreachable!(),
        };
        let e = match orthog_arc_cb {
            OrthogonalArc::Arc(circular_arc) => circular_arc.interpolate(t),
            OrthogonalArc::Diameter(line_segment) => line_segment.interpolate(t),
            OrthogonalArc::DiameterOutside(_) => unreachable!(),
        };

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

fn crinkle_highlight_leaves(arc: CircularArc, t: f64, depth: usize, styles: [Style; 2]) -> Group {
    let fractal = ArcFractal::new(arc, t);

    let (a, b) = fractal.xforms;
    let ifs = MonoidIFS::new(vec![a, b]);

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

    let triangle_tiles = ifs.apply(&triangle_tile, 0, depth - 1);
    let leaf_lenses = ifs.apply(&lens_tile, depth, depth);

    let [style_interior, style_leaves] = styles;

    union(vec![
        style_geometry(style_interior, &triangle_tiles[..]),
        style_geometry(style_leaves, &leaf_lenses[..]),
    ])
}

fn crinkle_two_color(arc: CircularArc, t: f64, depth: usize, styles: [Style; 2]) -> Group {
    let fractal = ArcFractal::new(arc, t);

    let (a, b) = fractal.xforms;

    // Doodling on paper, I find that alternating the colors as you iterate
    // deeper produceses a cool effect. Let's try that.
    //
    // First, compute an IFS that moves 2 iterations at a time.
    let aa = a * a;
    let ab = a * b;
    let ba = b * a;
    let bb = b * b;
    let ifs = MonoidIFS::new(vec![aa, ab, ba, bb]);

    let (arc_cb, arc_ba) = fractal.sub_arcs;
    let triangle_tile = ClineArcTile::new(vec![
        // orthogonal arc from b -> a
        fractal.orthog_arc.into(),
        // orthgonal arc from a -> midpoint
        arc_ba.reverse().into(),
        // orthogonal arc from midpoint -> b
        arc_cb.reverse().into(),
    ]);

    // Our tile will now be the big triangle tile + the first 2 children tiles (in a different color)
    let combined_tile = Motif::new(vec![
        (triangle_tile.clone(), 0),
        (triangle_tile.clone().transform(a), 1),
        (triangle_tile.clone().transform(b), 1),
    ]);

    // Remember, we're making bigger jumps and a higher branchihng factor, so tune the depth down a bit.
    let tiles = ifs.apply(&combined_tile, 0, depth);
    style_motifs(&tiles[..], &styles)
}

fn main() -> Result<(), Box<dyn Error>> {
    let angles = ArcAngles::new(-PI / 4.0, 5.0 * PI / 4.0)?;
    let arc = CircularArc::new(Circle::unit_circle(), angles);

    let depth = 7usize;
    let orange_lines = Style::stroke(255, 127, 0).with_width(0.25);
    let purple_lines = Style::stroke(127, 0, 255).with_width(0.25);
    render_views(
        "output",
        "crinkle_arc",
        &[View("", 0.0, 0.0, 1.1)],
        crinkle_highlight_leaves(arc, 0.5, depth, [orange_lines, purple_lines]),
    )?;

    let depth = 3usize;
    render_views(
        "output",
        "crinkle_two_color",
        &[View("", 0.0, 0.0, 1.1)],
        crinkle_two_color(
            arc,
            0.5,
            depth,
            // Make the purple lines a bit thinnner to match the orange ones.
            [orange_lines, purple_lines],
        ),
    )?;

    // Now let's chain multiple arcs together for a more intricate pattern
    let angles_a = ArcAngles::new(0.0, PI / 2.0)?;
    let angles_b = ArcAngles::new(0.0, -PI / 2.0)?;
    let angles_c = ArcAngles::new(PI, 3.0 * PI / 2.0)?;
    let angles_d = ArcAngles::new(PI, PI / 2.0)?;
    let unit_circle = Circle::unit_circle();
    let circle_b = Circle::new(Complex::new(-1.0, 1.0), 1.0);
    let circle_d = Circle::new(Complex::new(1.0, -1.0), 1.0);
    let arc_a = CircularArc::new(unit_circle, angles_a);
    let arc_b = CircularArc::new(circle_b, angles_b);
    let arc_c = CircularArc::new(unit_circle, angles_c);
    let arc_d = CircularArc::new(circle_d, angles_d);

    let styles = [
        // Salmon
        Style::stroke(230, 129, 203).with_width(0.25),
        // Mint green
        Style::stroke(41, 214, 165).with_width(0.25),
    ];

    let depth = 3usize;
    let t = 0.5;
    render_views(
        "output",
        "crinkle_necklace",
        &[View("", 0.0, 0.0, 1.1), View("zoom", 0.51, 0.3, 0.51)],
        union(vec![
            crinkle_two_color(arc_a, t, depth, styles),
            crinkle_two_color(arc_b, t, depth, styles),
            crinkle_two_color(arc_c, t, depth, styles),
            crinkle_two_color(arc_d, t, depth, styles),
        ]),
    )?;

    let semicircle_angles = ArcAngles::new(0.0, PI)?;
    let semicircle = CircularArc::new(unit_circle, semicircle_angles);
    render_views(
        "output",
        "crinkle_semicircle",
        &[View("", 0.0, 0.0, 1.1), View("zoom", 0.51, 0.3, 0.51)],
        crinkle_two_color(semicircle, 0.5, 3usize, styles),
    )?;

    let angles = ArcAngles::new(0.0, 4.0 * PI / 3.0)?;
    let arc = CircularArc::new(unit_circle, angles);
    render_views(
        "output",
        "crinkle_diameter",
        &[View("", 0.0, 0.0, 1.1)],
        crinkle_two_color(arc, 3.0 / 4.0, 3usize, styles),
    )?;

    Ok(())
}

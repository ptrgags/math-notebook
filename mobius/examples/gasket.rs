use abstraction::Group;
use mobius::{
    algorithms::GroupIFS,
    cline_arc::ClineArc,
    gasket_group,
    geometry::{Circle, LineSegment},
    motifs::ghost,
    rendering::Style,
    scale,
    svg_plot::{render_views, style_geometry, union, View},
    transformable::{ClineArcTile, ClineTile, Transformable},
    translation, Complex, Mobius,
};

pub fn main() -> Result<(), std::io::Error> {
    let (ghost, ghost_style) = ghost();

    let shrink = scale(0.1).unwrap();
    let shift = translation(Complex::new(-0.4, -0.15)).unwrap();
    let small_ghost = ghost.transform(shift * shrink);

    let yellow_lines = Style::stroke(255, 255, 0).with_width(0.125);
    let red_lines = Style::stroke(255, 0, 0).with_width(0.125);

    // Let's explore the Apollonian Gasket fractal
    let (gasket_a, gasket_b) = gasket_group();

    // Create a tile of the subgroup in the left circle.
    let top_line: ClineArc = LineSegment::new(-Complex::ONE, Complex::Zero).into();
    let right_arc = top_line.transform(gasket_a.inverse());
    let right_axis: ClineArc = LineSegment::new(Complex::Zero, Complex::ONE).into();
    let bottom_arc = right_axis.transform((gasket_a * gasket_b).inverse());
    let left_arc = right_axis.transform(gasket_b.inverse());

    let gasket_tile = ClineArcTile::new(vec![top_line, right_arc, bottom_arc, left_arc]);

    let ifs = GroupIFS::new(vec![gasket_a, gasket_b]);
    let left_circle = ClineTile::new(vec![Circle::new(Complex::new(-0.5, 0.0), 0.5).into()]);
    let gasket_walk = ifs.apply(&small_ghost, 0, 6);
    let tiles = ifs.apply(&gasket_tile, 0, 6);
    let circle_walk = ifs.apply(&left_circle, 0, 6);
    render_views(
        "output",
        "gasket",
        &[
            View("", 0.0, 0.0, 1.1),
            View("left_circle", -0.5, 0.0, 0.5),
            View("top_horn", -0.5, 0.5, 0.5),
            View("near_origin", 0.0, 0.0, 0.25),
        ],
        union(vec![
            style_geometry(red_lines, &circle_walk[..]),
            style_geometry(yellow_lines, &tiles[..]),
            style_geometry(ghost_style, &gasket_walk[..]),
        ]),
    )?;

    // Now let's make the subgroup for the left circle
    // it is the group generated by <a, Bab>
    let other_generator = Mobius::sandwich(gasket_b.inverse(), gasket_a);
    let subgroup = GroupIFS::new(vec![gasket_a, other_generator]);
    let subgroup_walk = subgroup.apply(&small_ghost, 0, 7);
    let subgroup_tiles = subgroup.apply(&gasket_tile, 0, 7);
    render_views(
        "output",
        "gasket_subgroup",
        &[View("", 0.0, 0.0, 1.1), View("left_circle", -0.5, 0.0, 0.5)],
        union(vec![
            style_geometry(red_lines, &left_circle),
            style_geometry(yellow_lines, &subgroup_tiles[..]),
            style_geometry(ghost_style, &subgroup_walk[..]),
        ]),
    )?;

    Ok(())
}
use std::{
    f64::consts::{FRAC_PI_2, FRAC_PI_6, PI},
    io::Error,
};

use abstraction::Group;
use mobius::{
    algorithms::{GridIFS, SemigroupIFS},
    cline_arc::ClineArc,
    geometry::LineSegment,
    hyperbolic,
    motifs::{bone, skull, witch_hat},
    rendering::Style,
    rotation, scale,
    svg_plot::{render_views, style_geometry, style_motifs, union, View},
    transformable::{ClineArcTile, Transformable},
    translation, Complex, Mobius,
};

pub fn main() -> Result<(), Error> {
    // "hex" tiles 😉 --------------------------------
    // No, not that hat tile. Which tile? Witch tile.
    let a_little_smaller = scale(0.9).unwrap();
    let hat_tile = witch_hat().transform(a_little_smaller);
    let sixth_roots = Complex::roots_of_unity(6);
    let bestagon = ClineArcTile::new(
        (0..6)
            .map(|i| {
                let side = LineSegment::new(sixth_roots[i], sixth_roots[(i + 1) % 6]);
                ClineArc::from(side)
            })
            .collect(),
    );
    let up = translation(Complex::new(0.0, (3.0f64).sqrt())).unwrap();
    let diagonal = translation(Complex::from_polar((3.0f64).sqrt(), FRAC_PI_6)).unwrap();
    let grid = GridIFS::new(vec![(up, -5, 5), (diagonal, -5, 5)]);
    let hex_tiles = grid.apply(&bestagon);
    let hat_tiles = grid.apply(&hat_tile);

    let grey_lines = Style::stroke(127, 127, 127).with_width(0.125);
    let hat_styles = vec![
        Style::stroke(0x92, 0x61, 0xba).with_width(0.25),
        Style::stroke(255, 255, 0).with_width(0.25),
    ];
    render_views(
        "output",
        "hex_tiles",
        &[View("", 0.0, 0.0, 3.5)],
        union(vec![
            style_geometry(grey_lines, &hex_tiles[..]),
            style_motifs(&hat_tiles, &hat_styles),
        ]),
    )?;

    // rib cage
    let smaller = scale(0.9).unwrap();
    let shift_up = translation(Complex::new(0.0, 2.0)).unwrap();
    let head = skull().transform(shift_up * smaller);
    let rot4 = rotation(FRAC_PI_2).unwrap();
    let rib = bone(20.0).transform(rot4 * a_little_smaller);
    let pull_left = hyperbolic(1.6).unwrap();
    let pull_down = Mobius::sandwich(rot4, pull_left);
    let cage = GridIFS::new(vec![(pull_down, -10, 10)]);
    let rib_cage = cage.apply(&rib);
    let white_lines = Style::stroke(255, 255, 255).with_width(0.25);
    render_views(
        "output",
        "rib_cage",
        &[View("", 0.0, 1.0, 1.5)],
        union(vec![
            style_geometry(white_lines, &rib_cage[..]),
            style_geometry(white_lines, &head),
        ]),
    )?;

    // bone tree
    let shrink_trunk = scale(0.5).unwrap();
    let shift_trunk = translation(Complex::I).unwrap();
    let trunk = bone(20.0).transform(shrink_trunk * shift_trunk);

    let rot2 = rotation(PI).unwrap();
    let shift_head = translation(Complex::new(0.0, 2.1)).unwrap();
    let heads_will_roll = skull().transform(shift_head * rot2);

    let branch_angle = FRAC_PI_2 / 3.0;
    let scale_factor = 0.8;
    let horizontal_shift = 0.1;
    let shrink_branch = scale(scale_factor).unwrap();
    let shift_up_left = translation(Complex::new(-horizontal_shift, 1.1)).unwrap();
    let shift_up_right = translation(Complex::new(horizontal_shift, 1.1)).unwrap();

    let rotate_left = rotation(branch_angle).unwrap();
    let rotate_right = rotate_left.inverse();
    let branch_left = shift_up_left * rotate_left * shrink_branch;
    let branch_right = shift_up_right * rotate_right * shrink_branch;

    let tree_ifs = SemigroupIFS::new(vec![branch_left, branch_right]);
    let bone_branches = tree_ifs.apply(&trunk, 0, 6);
    let skull_leaves = tree_ifs.apply(&heads_will_roll, 6, 6);
    render_views(
        "output",
        "bone_tree",
        &[View("", 0.0, 2.0, 3.5)],
        union(vec![
            style_geometry(white_lines, &bone_branches[..]),
            style_geometry(white_lines, &skull_leaves[..]),
        ]),
    )?;

    Ok(())
}

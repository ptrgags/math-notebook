use std::{error::Error, f64::consts::PI};

use abstraction::Group;
use mobius::{
    algorithms::{GridIFS, GroupIFS, MonoidIFS},
    cline_arc::ClineArc,
    elliptic, gasket_group,
    geometry::{Circle, LineSegment},
    hyperbolic,
    hyperbolic_tilings::{corner_rotation_group, get_fundamental_region, reflection_group},
    loxodromic, rotation, scale,
    transformable::{ClineArcTile, ClineTile, Motif, Transformable},
    translation, Complex, Mobius,
};
use rendering::{render_svg, style::Style, View};

pub fn candy_corners() -> Result<(), Box<dyn Error>> {
    let (corn, styles) = candy_corn()?;
    let (_, _, e2_conj) = reflection_group(3, 7)?;
    let complex = e2_conj * Complex::Zero;
    let dist_to_edge = 0.5 * (complex).real();
    let (r, e2, eq) = corner_rotation_group(3, 7)?;
    let (_, (_, _, vertex)) = get_fundamental_region(3, 7)?;
    let displacement = vertex * Complex::from(0.4);
    let shift = translation(displacement)?;
    let shrink = scale(dist_to_edge * 0.8)?;
    let rot60 = rotation(PI / 3.0)?;
    let tiny_corn = corn.transform(shift * rot60 * shrink);
    let ifs = MonoidIFS::new(vec![r, e2, eq]);
    let candy_corners = ifs.apply(&tiny_corn, 0, 7);
    render_svg(
        "output",
        "candy_corners_take2",
        &[View("", 0.0, 0.0, 1.0), View("zoom", 0.2, 0.0, 0.4)],
        RenderPrimitive::group(vec![style_motifs(&candy_corners, &styles)]),
    )?;

    Ok(())
}

pub fn hex_grid() -> Result<(), Box<dyn Error>> {
    // "hex" tiles 😉 --------------------------------
    // No, not that hat tile. Which tile? Witch tile.
    let a_little_smaller = scale(0.9)?;
    let hat_tile = witch_hat()?.transform(a_little_smaller);
    let sixth_roots = Complex::roots_of_unity(6);
    let bestagon = ClineArcTile::new(
        (0..6)
            .map(|i| {
                let side = LineSegment::new(sixth_roots[i], sixth_roots[(i + 1) % 6]);
                ClineArc::from(side)
            })
            .collect(),
    );
    let up = translation(Complex::new(0.0, (3.0f64).sqrt()))?;
    let diagonal = translation(Complex::from_polar((3.0f64).sqrt(), PI / 6.0))?;
    let grid = GridIFS::new(vec![(up, -5, 5), (diagonal, -5, 5)]);
    let hex_tiles = grid.apply(&bestagon);
    let hat_tiles = grid.apply(&hat_tile);

    let grey_lines = Style::stroke(127, 127, 127).with_width(0.125);
    let hat_styles = vec![
        Style::stroke(0x92, 0x61, 0xba).with_width(0.25),
        Style::stroke(255, 255, 0).with_width(0.25),
    ];
    render_svg(
        "output",
        "hex_tiles",
        &[View("", 0.0, 0.0, 3.5)],
        RenderPrimitive::group(vec![
            .render_group(grey_lines, &hex_tiles[..]),
            style_motifs(&hat_tiles, &hat_styles),
        ]),
    )?;
    Ok(())
}

pub fn bone_tree() -> Result<(), Box<dyn Error>> {
    // bone tree
    let shrink_trunk = scale(0.5)?;
    let shift_trunk = translation(Complex::I)?;
    let trunk = bone(20.0)?.transform(shrink_trunk * shift_trunk);

    let rot2 = rotation(PI)?;
    let shift_head = translation(Complex::new(0.0, 2.1))?;
    let roll = shift_head * rot2;
    let (head, face) = skull()?;
    let heads_will_roll = head.transform(roll);
    let eyes_will_roll = face.transform(roll);

    let branch_angle = PI / 6.0;
    let scale_factor = 0.8;
    let horizontal_shift = 0.1;
    let shrink_branch = scale(scale_factor)?;
    let shift_up_left = translation(Complex::new(-horizontal_shift, 1.1))?;
    let shift_up_right = translation(Complex::new(horizontal_shift, 1.1))?;

    let rotate_left = rotation(branch_angle)?;
    let rotate_right = rotate_left.inverse();
    let branch_left = shift_up_left * rotate_left * shrink_branch;
    let branch_right = shift_up_right * rotate_right * shrink_branch;

    let tree_ifs = MonoidIFS::new(vec![branch_left, branch_right]);
    let bone_branches = tree_ifs.apply(&trunk, 0, 6);
    let head_leaves = tree_ifs.apply(&heads_will_roll, 6, 6);
    let face_leaves = tree_ifs.apply(&eyes_will_roll, 6, 6);
    let black = Style::fill(0, 0, 0).with_stroke(0, 0, 0).with_width(0.25);
    let white_fill = Style::fill(255, 255, 255);
    render_svg(
        "output",
        "bone_tree",
        &[View("", 0.0, 2.0, 3.5)],
        RenderPrimitive::group(vec![
            .render_group(white_fill, &bone_branches[..]),
            .render_group(white_fill, &head_leaves[..]),
            .render_group(black, &face_leaves[..]),
        ]),
    )?;
    Ok(())
}

pub fn rib_cage() -> Result<(), Box<dyn Error>> {
    // rib cage
    let smaller = scale(0.9)?;
    let shift_up = translation(Complex::new(0.0, 2.0))?;
    let (head, face) = skull()?;
    let adjusted_head = head.transform(shift_up * smaller);
    let adjusted_face = face.transform(shift_up * smaller);
    let rot4 = rotation(PI / 2.0)?;
    let rib = bone(20.0)?.transform(rot4 * smaller);
    let pull_left = hyperbolic(1.6)?;
    let pull_down = Mobius::sandwich(rot4, pull_left);
    let cage = GridIFS::new(vec![(pull_down, -10, 10)]);
    let rib_cage = cage.apply(&rib);
    let white_fill = Style::fill(255, 255, 255);
    let black = Style::fill(0, 0, 0).with_stroke(0, 0, 0).with_width(0.25);
    render_svg(
        "output",
        "rib_cage",
        &[View("", 0.0, 1.0, 1.5)],
        RenderPrimitive::group(vec![
            .render_group(white_fill, &rib_cage[..]),
            .render_group(white_fill, &adjusted_head),
            .render_group(black, &adjusted_face),
        ]),
    )?;
    Ok(())
}

pub fn ghost_octahedral() -> Result<(), Box<dyn Error>> {
    let (ghost, ghost_styles) = ghost()?;
    let shrink = scale(0.125)?;
    let small_ghost = ghost.transform(shrink);

    // Two 90 degree elliptic rotations 90 degrees apart. This is isomorphic
    // to the rotation symmetry group of the cube/octahedron
    let swirl = elliptic(PI / 2.0)?;
    let rotate90 = rotation(PI / 2.0)?;
    let swirl2 = Mobius::sandwich(rotate90, swirl);
    let to_the_left = translation(Complex::new(-0.5, 0.0))?;
    let shifted_ghost = small_ghost.transform(to_the_left);
    let ifs = MonoidIFS::new(vec![swirl, swirl2]);
    let swirl_walk = ifs.apply(&shifted_ghost, 0, 8);
    render_svg(
        "output",
        "ghost_octahedral",
        &[View("", 0.0, 0.0, 3.0)],
        style_motifs(&swirl_walk[..], &ghost_styles),
    )?;
    Ok(())
}

pub fn ghost_double_spiral() -> Result<(), Box<dyn Error>> {
    let (ghost, ghost_styles) = ghost()?;
    let shrink = scale(0.125)?;
    let small_ghost = ghost.transform(shrink);

    // A loxodromic double spiral. Though instead of going from -1 to 1,
    // I want it from -i to i, so conjugate by a rotate
    let double_spiral = loxodromic(Complex::new(1.5, 1.1))?;
    let rotate90 = rotation(PI / 2.0)?;
    let vertical_spiral = Mobius::sandwich(rotate90, double_spiral);
    let ellip4 = elliptic(2.0 * PI / 3.0)?;
    let vertical_ellip = Mobius::sandwich(rotate90, ellip4);

    let grid = GridIFS::new(vec![(vertical_spiral, -8, 8), (vertical_ellip, -1, 2)]);

    let double_spiral_walk = grid.apply(&small_ghost);
    render_svg(
        "output",
        "ghost_double_spiral",
        &[View("", 0.0, 0.0, 1.0), View("sink", -0.125, 0.75, 0.5)],
        style_motifs(&double_spiral_walk[..], &ghost_styles),
    )?;
    Ok(())
}

pub fn ghost_gasket() -> Result<(), Box<dyn Error>> {
    let (ghost, ghost_styles) = ghost()?;

    let shrink = scale(0.1)?;
    let shift = translation(Complex::new(-0.4, -0.15))?;
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
    render_svg(
        "output",
        "gasket",
        &[
            View("", 0.0, 0.0, 1.1),
            View("left_circle", -0.5, 0.0, 0.5),
            View("top_horn", -0.5, 0.5, 0.5),
            View("near_origin", 0.0, 0.0, 0.25),
        ],
        RenderPrimitive::group(vec![
            .render_group(red_lines, &circle_walk[..]),
            .render_group(yellow_lines, &tiles[..]),
            style_motifs(&gasket_walk[..], &ghost_styles),
        ]),
    )?;

    // Now let's make the subgroup for the left circle
    // it is the group generated by <a, Bab>
    let other_generator = Mobius::sandwich(gasket_b.inverse(), gasket_a);
    let subgroup = GroupIFS::new(vec![gasket_a, other_generator]);
    let subgroup_walk = subgroup.apply(&small_ghost, 0, 7);
    let subgroup_tiles = subgroup.apply(&gasket_tile, 0, 7);
    render_svg(
        "output",
        "gasket_subgroup",
        &[View("", 0.0, 0.0, 1.1), View("left_circle", -0.5, 0.0, 0.5)],
        RenderPrimitive::group(vec![
            .render_group(red_lines, &left_circle),
            .render_group(yellow_lines, &subgroup_tiles[..]),
            style_motifs(&subgroup_walk[..], &ghost_styles),
        ]),
    )?;

    Ok(())
}

pub fn warpedpaper() -> Result<(), Box<dyn Error>> {
    let (corn, styles) = candy_corn()?;

    let rotate2 = rotation(PI)?;
    let translate_edge = translation(Complex::from_polar(0.6, PI / 3.0))?;
    let rotate_edge = Mobius::sandwich(translate_edge, rotate2);
    let rotated_corn = corn.transform(rotate_edge);
    let two_corns = Motif::union(corn.clone(), rotated_corn);

    let translate_up = translation(Complex::new(0.0, 2.2))?;
    let translate_right = translation((1.7).into())?;
    let grid = GridIFS::new(vec![(translate_up, -5, 7), (translate_right, -8, 3)]);

    // What happens if we conjugate by a transform that fixes the unit circle?
    let pull_left = hyperbolic(1.2)?;
    let curved_grid = grid.conjugate(pull_left);
    //let warped_pair = two_corns.transform(pull_left);
    let curved_wallpaper = curved_grid.apply(&two_corns);
    let curved_svg = style_motifs(&curved_wallpaper[..], &styles);
    render_svg(
        "output",
        "candy_corn_warpedpaper",
        &[View("", -2.5, 3.0, 4.0)],
        curved_svg,
    )?;

    Ok(())
}

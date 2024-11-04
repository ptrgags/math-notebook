use std::{
    f64::consts::{FRAC_PI_2, FRAC_PI_3, PI},
    io::Error,
};

use abstraction::Group;
use mobius::{
    algorithms::{GridIFS, SemigroupIFS},
    hyperbolic,
    hyperbolic_tilings::{corner_rotation_group, get_fundamental_region, reflection_group},
    motifs::candy_corn,
    rendering::Style,
    rotation, scale,
    svg_plot::{render_views, style_geometry, style_motif, style_motifs, union, View},
    transformable::{Motif, Transformable},
    translation, Complex, Mobius,
};

pub fn main() -> Result<(), Error> {
    let (corn, styles) = candy_corn();

    // Rotate the candy corn upright and view it
    let rotate_90 = rotation(FRAC_PI_2).unwrap();
    let upright = corn.transform(rotate_90);
    render_views(
        "output",
        "candy_corn",
        &[View("", 0.0, 0.0, 1.0)],
        style_motif(&upright, &styles),
    )?;

    // Make a candy corn mandala
    let shift_left = translation((-2.0).into()).unwrap();
    let shifted = corn.transform(shift_left);
    let rotate6 = rotation(FRAC_PI_3).unwrap();
    let expand = scale(3.0).unwrap();
    let grid = GridIFS::new(vec![(rotate6, 0, 6), (expand, 0, 3)]);
    let mandala = grid.apply(&shifted);
    render_views(
        "output",
        "candy_corn_mandala",
        &[View("", 0.0, 0.0, 20.0)],
        style_motifs(&mandala[..], &styles),
    )?;

    // Make a candy corn wallpaper.
    let rotate2 = rotation(PI).unwrap();
    let translate_edge = translation(Complex::from_polar(0.6, FRAC_PI_3)).unwrap();
    let rotate_edge = Mobius::sandwich(translate_edge, rotate2);
    let rotated_corn = corn.transform(rotate_edge);
    let two_corns = Motif::union(corn.clone(), rotated_corn);

    let translate_up = translation(Complex::new(0.0, 2.2)).unwrap();
    let translate_right = translation((1.7).into()).unwrap();
    let grid = GridIFS::new(vec![(translate_up, -10, 10), (translate_right, -10, 10)]);
    let wallpaper = grid.apply(&two_corns);
    render_views(
        "output",
        "candy_corn_wallpaper",
        &[View("", 0.0, 0.0, 5.0)],
        style_motifs(&wallpaper[..], &styles),
    )?;

    // What happens if we conjugate by a transform that fixes the unit circle?
    let pull_left = hyperbolic(1.2).unwrap();
    let curved_grid = grid.conjugate(pull_left);
    //let warped_pair = two_corns.transform(pull_left);
    let curved_wallpaper = curved_grid.apply(&two_corns);
    let curved_svg = style_motifs(&curved_wallpaper[..], &styles);
    render_views(
        "output",
        "candy_corn_warpedpaper",
        &[View("", -2.5, 3.0, 4.0)],
        curved_svg,
    )?;

    // Candy-corner hyperbolic tiling, based on tiling {3, 7}
    let (conj, r_conj, e2_conj) = reflection_group(3, 7).unwrap();
    let dist_to_edge = 0.5 * (e2_conj * Complex::Zero).real();
    println!("{}", dist_to_edge);
    let shrink = scale(dist_to_edge * 0.3).unwrap();
    let shift = translation(Complex::new(0.51 * dist_to_edge, 0.05)).unwrap();
    let tiny_corn = corn.transform(shift * shrink);
    let ifs = SemigroupIFS::new(vec![conj, r_conj, e2_conj]);
    let candy_corners = ifs.apply(&tiny_corn, 0, 7);
    let (fundamental_triangle, (_, _, vertex)) = get_fundamental_region(3, 7).unwrap();
    let tiles = ifs.apply(&fundamental_triangle, 0, 5);
    let yellow_lines = Style::stroke(255, 0, 255).with_width(0.5);

    render_views(
        "output",
        "candy_corners",
        &[View("", 0.0, 0.0, 0.5)],
        union(vec![
            style_motifs(&candy_corners, &styles),
            style_geometry(yellow_lines, &tiles[..]),
        ]),
    )?;

    let (r, e2, eq) = corner_rotation_group(3, 7).unwrap();
    let shift = translation(vertex * Complex::from(0.4)).unwrap();
    let shrink = scale(dist_to_edge * 0.8).unwrap();
    let rot60 = rotation(FRAC_PI_3).unwrap();
    let tiny_corn = corn.transform(shift * rot60 * shrink);
    let ifs = SemigroupIFS::new(vec![r, e2, eq]);
    let candy_corners = ifs.apply(&tiny_corn, 0, 7);
    render_views(
        "output",
        "candy_corners_take2",
        &[View("", 0.0, 0.0, 1.0), View("zoom", 0.2, 0.0, 0.4)],
        union(vec![style_motifs(&candy_corners, &styles)]),
    )?;

    Ok(())
}

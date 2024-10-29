use std::{
    f64::consts::{FRAC_PI_2, FRAC_PI_3, PI},
    io::Error,
};

use abstraction::Group;
use mobius::{
    algorithms::GridIFS,
    hyperbolic,
    motifs::candy_corn,
    rendering::Style,
    rotation, scale,
    svg_plot::{render_views, style_geometry, style_motif, style_motifs, union, View},
    transformable::{Cline, ClineTile, Motif, Transformable},
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
    let scale = scale(3.0).unwrap();
    let grid = GridIFS::new(vec![(rotate6, 0, 6), (scale, 0, 3)]);
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
    let unit_circle = style_geometry(
        Style::stroke(255, 255, 255).with_width(0.5),
        &ClineTile::new(vec![Cline::unit_circle()]),
    );

    render_views(
        "output",
        "candy_corn_warpedpaper",
        &[View("", -2.5, 3.0, 4.0)],
        union(vec![unit_circle, curved_svg]),
    )?;

    Ok(())
}

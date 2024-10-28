use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, PI};

use abstraction::Group;
use mobius::{
    elliptic, iterated_function_system::{apply_ifs, IFS}, loxodromic, map_triple, motifs::ghost, rendering::Style, rotation, scale, svg_plot::{render_views, style_geometry, View}, transformable::Transformable, translation, Complex, Mobius
};

pub fn main() -> Result<(), std::io::Error> {
    let ghost = ghost();
    let ghost_style = Style::stroke(0xc5, 0xf2, 0xfa).with_width(0.25);

    // Show the ghost by themself -----------------------------------
    render_views(
        "output",
        "ghosty",
        &[View("", 0.0, -0.5, 2.5)],
        style_geometry(ghost_style, &ghost)
    )?;

    // Oh no! the ghost fell down the drain! ----------------------

    // Create a transform with a spiral sink at +3
    let translate3 = translation(3.0.into()).unwrap();
    let spiral_in = rotation(-FRAC_PI_3).unwrap() * scale(0.6).unwrap();
    let drain = Mobius::sandwich(translate3, spiral_in);
    let ifs = IFS::new(vec![drain]);
    let down_the_drain = apply_ifs(&ifs, &ghost, 0, 20);

    render_views(
        "output",
        "ghost_down_drain",
        &[View("", 1.5, 0.0, 2.5)],
        style_geometry(ghost_style, &down_the_drain[..]),
    )?;

    // Caught between parabolic transforms that have fixed points at -1 and
    // 1, and map the opposite point on the unit circle 90 degrees around the
    // circle. This would benefit from skipping inverses, parts of this
    // diagram will be rendered many times.
    // -------------------------------
    let shrink = scale(0.125).unwrap();
    let small_ghost = ghost.transform(shrink);
    let left_parabolic = map_triple(
        (-Complex::ONE, Complex::ONE, -Complex::I),
        (-Complex::ONE, Complex::I, Complex::ONE),
    )
    .unwrap();
    let right_parabolic = map_triple(
        (Complex::ONE, -Complex::ONE, -Complex::I),
        (Complex::ONE, Complex::I, -Complex::ONE),
    )
    .unwrap();

    let ifs = IFS::new(vec![
        left_parabolic,
        right_parabolic,
    ]);
    let parabolic_walk = apply_ifs(&ifs, &small_ghost, 0, 6);
    render_views(
        "output",
        "ghost_parabolic",
        &[View("", 0.0, 0.5, 0.9), View("zoom_in", -0.5, 0.6, 0.2)],
        style_geometry(ghost_style, &parabolic_walk[..]),
    )?;

    // A loxodromic double spiral. Though instead of going from -1 to 1,
    // I want it from -i to i, so conjugate by a rotate
    let double_spiral = loxodromic(Complex::new(1.5, 1.1)).unwrap();
    let rotate90 = rotation(FRAC_PI_2).unwrap();
    let vertical_spiral = Mobius::sandwich(rotate90, double_spiral);
    let ifs = IFS::new(vec![vertical_spiral, vertical_spiral.inverse()]);
    let double_spiral_walk = apply_ifs(&ifs, &small_ghost, 0, 10);
    render_views(
        "output",
        "ghost_double_spiral",
        &[View("", 0.0, 0.0, 1.0), View("sink", -0.125, 0.75, 0.5)],
        style_geometry(ghost_style, &double_spiral_walk[..])
    )?;

    // Two 90 degree elliptic rotations 90 degrees apart. This is isomorphic
    // to the rotation symmetry group of the cube/octahedron
    let swirl = elliptic(FRAC_PI_2).unwrap();
    let swirl2 = Mobius::sandwich(rotate90, swirl);
    let to_the_left = translation(Complex::new(-0.5, 0.0)).unwrap();
    let shifted_ghost = small_ghost.transform(to_the_left);
    let ifs = IFS::new(vec![swirl, swirl2]);
    let swirl_walk = apply_ifs(&ifs, &shifted_ghost, 0, 8);
    render_views(
        "output",
        "ghost_octahedral",
        &[View("", 0.0, 0.0, 3.0)],
        style_geometry(ghost_style, &swirl_walk[..])
    )?;

    // But now if we make the rotation slightly different, things don't
    // quite line up. I find the result amusing.
    let swirl = elliptic(PI / 2.01).unwrap();
    let swirl2 = Mobius::sandwich(rotate90, swirl);
    let ifs = IFS::new(vec![swirl, swirl2]);
    let swirl_walk = apply_ifs(&ifs, &shifted_ghost, 0, 8);
    render_views(
        "output",
        "ghost_triggered",
        &[View("", 0.0, 0.0, 3.0)],
        style_geometry(ghost_style, &swirl_walk[..])
    )?;

    Ok(())
}

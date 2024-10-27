use core::f64;

use mobius::{
    cline_tile::{ClineArcTile, ClineTile},
    geometry::{Circle, CircularArc, LineSegment},
    scale,
    svg_plot::{add_geometry, flip_y, make_axes, make_card},
    transformable::{Cline, Transformable},
    Complex, Mobius,
};
use svg::{
    node::element::{Group, Rectangle},
    Document,
};

fn compute_xforms() -> Vec<Mobius> {
    // Transform A just shrinks the unit circle to the circle with
    // radius 1/2
    let xform_a = scale(0.5).unwrap();

    // Transform B has the following properties:
    // B(0) = 1/2
    //  --> d = 2b
    //  --> B = [a  b]
    //        = [c 2b]
    // B(1) = 1
    //  --> a + b = c + d
    //          a = c + (2b) - b
    //          a = c + b
    //  --> B = [(c + b)  b]
    //          [c       2b]
    // B(i) = sqrt(i)  (call this omega)
    //  --> (ai + b) = omega (ci + d)
    //       (c + b)i + b = omega(ci + 2b)
    //       ci + bi + b = (omega c)i + 2 omega b
    //       (1 - omega)i c = (2 omega - i - 1) b
    //       (1 - omega)i / (2 omega - i - 1) c = b
    //  let's call the mess on the left gamma, so that
    //       gamma c = b
    //  --> [(c + gamma c)   (gamma c)]
    //      [c             (2 gamma c)]
    // det B = 1
    //  --> (c + gamma c)(2 gamma c) - c(gamma c) = 1
    //      2 gamma c^2 + 2 gamma^2 c^2 - gamma c^2
    //      (2 gamma + 2 gamma^2 - gamma)c^2 = 1
    //      (gamma + 2 gamma^2)c^2 = 1
    //      c = sqrt(1 / (gamma + 2 gamma^2))
    let omega = Complex::I.sqrt();
    let two: Complex = (2.0).into();
    let gamma_numerator = (Complex::ONE - omega) * Complex::I;
    let gamma_denominator: Complex = two * omega - Complex::new(1.0, 1.0);
    let gamma = gamma_numerator / gamma_denominator;
    let c = (Complex::ONE / (gamma + two * gamma * gamma)).sqrt();
    let b = gamma * c;
    let d = two * b;
    let a = b + c;
    let xform_b = Mobius::new(a, b, c, d).expect("Determinant not one???");

    // the transform C is essentially the same thing as B,
    // except mirrored over y = x
    // let mirror(z) = i * conj(z)
    // and note that mirror^(-1) = mirror
    //
    // we want
    // C = mirror 🥪 B
    //   = mirror B mirror
    //   = i conj((a (i conj(z)) + b) / (c (i conj (z)) + d))
    //   = i (conj(a) conj(i) z + conj(b)) / (conj(c) conj(i) z) + conj(d))
    //   = i (-i conj(a) z + conj(b)) / (-i conj(c) z) + conj(d))
    //   = (conj(a) z + i conj(b)) / (-i conj(c) z + conj(d))
    //
    // so a' = conj(a)
    //    b' = i conj(b) = mirror(b)
    //    c' = - i conj(c) = -mirror(c)
    //    d' = conj(d)
    let xform_c = Mobius::new(
        a.conj(),
        Complex::I * b.conj(),
        -Complex::I * c.conj(),
        d.conj(),
    )
    .expect("Determinant not 1???");

    vec![xform_a, xform_b, xform_c]
}

fn iterate(xforms: &[Mobius], tile: &ClineTile, depth: u8) -> Vec<ClineTile> {
    if depth == 0 {
        return xforms.iter().map(|x| tile.transform(*x)).collect();
    }

    let mut result: Vec<ClineTile> = vec![tile.clone()];
    for xform in xforms {
        let prefixed: Vec<Mobius> = xforms.iter().map(|x| *xform * *x).collect();
        let subtree = iterate(&prefixed, tile, depth - 1);
        result.extend(subtree);
    }

    result
}

fn apply_xforms(xforms: &[Mobius], tile: &ClineArcTile) -> Vec<ClineArcTile> {
    xforms.iter().map(|x| tile.transform(*x)).collect()
}

fn iteration(xforms: &[Mobius], tiles: &[ClineArcTile]) -> Vec<ClineArcTile> {
    tiles
        .iter()
        .flat_map(|tile| apply_xforms(xforms, tile))
        .collect()
}

fn main() {
    let xforms = compute_xforms();

    let initial_tile = ClineTile::new(vec![
        Cline::real_axis(),
        Cline::imag_axis(),
        Cline::unit_circle(),
    ]);

    let new_tiles = iterate(&xforms[1..2], &initial_tile, 1);

    let mut geometry = Group::new()
        .set("stroke", "yellow")
        .set("stroke-width", "0.5%")
        .set("fill", "none");
    geometry = add_geometry(geometry, &new_tiles[..]);

    let axes = make_axes()
        .set("fill", "none")
        .set("stroke", "white")
        .set("stroke-width", "0.5%");

    let background = Rectangle::new()
        .set("x", "-50%")
        .set("y", "-50%")
        .set("width", "100%")
        .set("height", "100%")
        .set("fill", "black");

    let flipped = flip_y().add(axes.clone()).add(geometry);

    let document = Document::new()
        .set("width", 500)
        .set("height", 500)
        .set("viewBox", (-2, -2, 4, 4))
        .add(background.clone())
        .add(flipped);

    svg::save("mobius_sierpinski.svg", &document).unwrap();

    // ----------------------

    let tile = ClineArcTile::new(vec![
        LineSegment::new(Complex::Zero, Complex::ONE).into(),
        CircularArc::new(
            Circle::unit_circle(),
            0.0,
            f64::consts::FRAC_PI_4,
            f64::consts::FRAC_PI_2,
        )
        .into(),
        LineSegment::new(Complex::I, Complex::Zero).into(),
    ]);

    let tiles_level1 = apply_xforms(&xforms, &tile);
    let tiles_level2 = iteration(&xforms, &tiles_level1);
    let tiles_level3 = iteration(&xforms, &tiles_level2);
    let tiles_level4 = iteration(&xforms, &tiles_level3);

    let mut geometry = Group::new()
        .set("stroke", "yellow")
        .set("stroke-width", "0.25%")
        .set("fill", "none");
    geometry = add_geometry(geometry, &tile);
    geometry = add_geometry(geometry, &tiles_level1[..]);
    geometry = add_geometry(geometry, &tiles_level2[..]);
    geometry = add_geometry(geometry, &tiles_level3[..]);
    geometry = add_geometry(geometry, &tiles_level4[..]);

    let flipped2 = flip_y().add(axes).add(geometry);

    let doc = make_card(Complex::new(0.5, 0.5), 0.6).add(flipped2);
    svg::save("output/mobius_sierpinski.svg", &doc).unwrap();
}

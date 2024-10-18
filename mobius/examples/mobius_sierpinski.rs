use core::f64;
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};

use mobius::{
    cline::Cline,
    cline_arc::ClineArc,
    scale,
    svg_plot::{flip_y, make_axes, svg_circular_arc, svg_cline_arc, svg_cline_tile},
    ClineTile, Complex, Mobius,
};
use svg::{
    node::element::{Group, Rectangle},
    Document, Node,
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

fn main() {
    let xforms = compute_xforms();

    let initial_tile = ClineTile::new(vec![
        Cline::real_axis(),
        Cline::imag_axis(),
        Cline::unit_circle(),
    ]);

    let new_tiles = iterate(&xforms[1..2], &initial_tile, 1);

    let svg_tiles: Vec<Box<dyn Node>> = new_tiles.iter().flat_map(|x| svg_cline_tile(x)).collect();

    let mut geometry = Group::new()
        .set("stroke", "yellow")
        .set("stroke-width", "0.5%")
        .set("fill", "none");
    for svg_node in svg_tiles {
        geometry = geometry.add(svg_node);
    }

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

    let bottom = ClineArc::line_segment(Complex::Zero, Complex::ONE);
    let circle_arc = ClineArc::from_circle_and_angles(
        Complex::Zero,
        1.0,
        0.0,
        f64::consts::FRAC_PI_4,
        f64::consts::FRAC_PI_2,
    );
    let left = ClineArc::line_segment(Complex::I, Complex::Zero);

    let geometry = Group::new()
        .set("stroke", "yellow")
        .set("stroke-width", "0.5%")
        .set("fill", "none")
        .add(svg_cline_arc(&bottom))
        .add(svg_cline_arc(&circle_arc))
        .add(svg_cline_arc(&left));

    let flipped2 = flip_y().add(axes).add(geometry);

    let arc_test = Document::new()
        .set("width", 500)
        .set("height", 500)
        .set("viewBox", (-2, -2, 4, 4))
        .add(background)
        .add(flipped2);

    svg::save("arc_test.svg", &arc_test).unwrap();
}

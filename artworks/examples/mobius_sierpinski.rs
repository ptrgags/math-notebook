use std::{error::Error, f64::consts::PI};

use mobius::{
    algorithms::MonoidIFS,
    geometry::{ArcAngles, Circle, CircularArc, LineSegment},
    scale,
    transformable::{ClineArcTile, Collection},
    Complex, Mobius,
};
use rendering::{render_svg, style::Style, Renderable, View};

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

fn main() -> Result<(), Box<dyn Error>> {
    let xforms = compute_xforms();

    // ----------------------

    let quarter_circle = ArcAngles::new(0.0, PI / 2.0).unwrap();
    let tile = ClineArcTile::new(vec![
        LineSegment::new(Complex::Zero, Complex::ONE).into(),
        CircularArc::new(Circle::unit_circle(), quarter_circle).into(),
        LineSegment::new(Complex::I, Complex::Zero).into(),
    ]);

    let ifs = MonoidIFS::new(xforms);
    let tiles = ifs.flat_apply(&tile, 0, 4);

    let yellow = Style::stroke(255, 255, 0).with_width(0.25);

    render_svg(
        "output",
        "mobius_sierpinski",
        &[View("", 0.5, 0.5, 0.6)],
        tiles.render_group(yellow)?,
    )?;

    Ok(())
}

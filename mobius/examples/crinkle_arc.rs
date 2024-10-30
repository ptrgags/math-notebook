use std::{
    f64::consts::{PI, TAU},
    io::Error,
};

use mobius::{algorithms::SemigroupIFS, transformable::Cline};
use mobius::{
    cline_arc::ClineArc,
    geometry::{Circle, CircularArc},
    map_triple,
    rendering::Style,
    svg_plot::{render_views, style_geometry, View},
    Complex, Mobius,
};
use svg::node::element::Group;

/// Compute an orthogonal circle through a and b.
/// I've done this before for another project, see my explainer here:
/// https://github.com/ptrgags/p5-sketchbook/tree/main/HyperbolicConnections#method-2-kite-analysis
/// as well as the code
/// https://github.com/ptrgags/p5-sketchbook/blob/458e47383ed8492cff3cc0bce4bded666b0672bc/HyperbolicConnections/boundaries.js#L35
fn get_orthog_circle(circle: Circle, a: Complex, b: Complex) -> Circle {
    let q = (a - b).mag();
    let p = (4.0 / (4.0 - q * q)).sqrt();
    let orthog_radius = 0.5 * p * q;

    let angle_a = circle.get_angle(a).unwrap();
    let angle_b = circle.get_angle(b).unwrap();

    let angle_bisector = 0.5 * (angle_a + angle_b);
    let angle_bisector = if (angle_b - angle_a) % TAU > PI {
        (angle_bisector + PI) % TAU
    } else {
        angle_bisector
    };

    let orthog_center = Complex::from_polar(p, angle_bisector);

    Circle {
        center: orthog_center,
        radius: orthog_radius,
    }
}

fn arc_lerp(circle: Circle, a: Complex, b: Complex, t: f64) -> Complex {
    let angle_a = circle.get_angle(a).unwrap();
    let angle_b = circle.get_angle(b).unwrap();

    let lerp_angle = (1.0 - t) * angle_a + t * angle_b;

    circle.get_point(lerp_angle)
}

fn arc_fractal(arc: CircularArc) -> (Mobius, Mobius) {
    let CircularArc {
        circle,
        angle_a,
        angle_b,
        angle_c,
    } = arc;
    let t = ((angle_b - angle_a) / (angle_c - angle_a)).abs();

    let a = circle.get_point(angle_a);
    let b = circle.get_point(angle_b);
    let c = circle.get_point(angle_c);

    let circle_ab = get_orthog_circle(circle, a, b);
    let circle_bc = get_orthog_circle(circle, b, c);

    let d = arc_lerp(circle_ab, b, a, t);
    let e = arc_lerp(circle_bc, c, b, t);

    let xform_bda = map_triple((a, b, c), (b, d, a)).unwrap();
    let xform_ceb = map_triple((a, b, c), (c, e, b)).unwrap();

    (xform_bda, xform_ceb)
}

fn main() -> Result<(), Error> {
    let arc = CircularArc::new(Circle::unit_circle(), 0.0, PI / 4.0, PI / 2.0);
    let (a, b) = arc_fractal(arc);

    let tile: ClineArc = arc.into();

    let ifs = SemigroupIFS::new(vec![a, b]);

    let tiles = ifs.apply(&tile, 0, 8);
    let orange_lines = Style::stroke(255, 127, 0).with_width(0.5);
    let geometry = style_geometry(orange_lines, &tiles[..]);

    let orthog_circle = get_orthog_circle(Circle::unit_circle(), Complex::ONE, Complex::I);
    let circle_cline: Cline = orthog_circle.into();
    let yellow_lines = Style::stroke(255, 255, 0).with_width(0.5);
    let more_geometry = style_geometry(yellow_lines, &circle_cline);

    let group = Group::new().add(geometry).add(more_geometry);

    render_views("output", "crinkle_arc", &[View("", 0.5, 0.5, 0.51)], group)?;

    Ok(())
}

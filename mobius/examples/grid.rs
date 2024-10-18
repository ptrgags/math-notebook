use core::f64;

use mobius::{
    cline::Cline,
    elliptic, hyperbolic,
    svg_plot::{flip_y, make_axes, svg_cline},
    Mobius,
};
use svg::{
    node::element::{Group, Rectangle},
    Document,
};

fn iterate_xform(xform: Mobius, n: usize) -> Vec<Mobius> {
    let mut result = Vec::with_capacity(n);

    let mut current = xform;
    for _ in 0..n {
        result.push(current);
        current = xform * current;
    }

    result
}

fn main() {
    let h = hyperbolic(2.0).unwrap();
    let mut h_forward_powers = iterate_xform(h, 10);
    let mut h_inv_powers = iterate_xform(h.inverse(), 10);
    let mut h_powers = vec![Mobius::IDENTITY];
    h_powers.append(&mut h_forward_powers);
    h_powers.append(&mut h_inv_powers);

    let center_line = Cline::imag_axis();

    let h_clines: Vec<Cline> = h_powers
        .iter()
        .map(|x| Cline::transform(*x, center_line))
        .collect();

    let mut lattitude_lines = Group::new()
        .set("stroke", "yellow")
        .set("stroke-width", "0.25%")
        .set("fill", "none");
    for cline in h_clines {
        lattitude_lines = lattitude_lines.add(svg_cline(&cline))
    }

    let e = elliptic(f64::consts::PI / 8.0).unwrap();
    let e_powers = iterate_xform(e, 16);

    let real_axis = Cline::real_axis();

    let e_clines: Vec<Cline> = e_powers
        .iter()
        .map(|x| Cline::transform(*x, real_axis))
        .collect();

    let mut meridians = Group::new()
        .set("stroke", "red")
        .set("stroke-width", "0.25%")
        .set("fill", "none");
    for cline in e_clines {
        meridians = meridians.add(svg_cline(&cline))
    }

    let axes = make_axes()
        .set("fill", "none")
        .set("stroke", "white")
        .set("stroke-width", "0.25%");

    let boundary = Rectangle::new()
        .set("x", "-50%")
        .set("y", "-50%")
        .set("width", "100%")
        .set("height", "100%")
        .set("fill", "black");

    let flipped = flip_y().add(axes).add(lattitude_lines).add(meridians);

    let document = Document::new()
        .set("width", 500)
        .set("height", 500)
        .set("viewBox", (-3, -3, 6, 6))
        .add(boundary)
        .add(flipped);

    svg::save("grid.svg", &document).unwrap();
}

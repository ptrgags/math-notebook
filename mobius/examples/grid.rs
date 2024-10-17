use core::f64;

use mobius::{
    cline::Cline,
    rotation,
    svg_plot::{flip_y, make_axes, svg_cline},
};
use svg::{
    node::element::{Group, Rectangle},
    Document,
};

fn main() {
    let rotate = rotation(f64::consts::PI / 8.0).unwrap();
    let real_axis = Cline::real_axis();

    let axis1 = real_axis;
    let axis2 = Cline::transform(rotate, axis1);
    let axis3 = Cline::transform(rotate, axis2);
    let axis4 = Cline::transform(rotate, axis3);

    let meridians = Group::new()
        .add(svg_cline(&axis1))
        .add(svg_cline(&axis2))
        .add(svg_cline(&axis3))
        .add(svg_cline(&axis4))
        .set("stroke", "yellow")
        .set("stroke-width", "0.5%");

    let axes = make_axes()
        .set("fill", "none")
        .set("stroke", "white")
        .set("stroke-width", "0.5%");

    let boundary = Rectangle::new()
        .set("x", "-50%")
        .set("y", "-50%")
        .set("width", "100%")
        .set("height", "100%")
        .set("fill", "black");

    let flipped = flip_y().add(axes).add(meridians);

    let document = Document::new()
        .set("width", 500)
        .set("height", 500)
        .set("viewBox", (-2, -2, 4, 4))
        .add(boundary)
        .add(flipped);

    svg::save("grid.svg", &document).unwrap();
}

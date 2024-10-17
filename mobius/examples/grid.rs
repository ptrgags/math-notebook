use core::f64;

use mobius::{
    cline::{Cline, GeneralizedCircle},
    rotation, scale, ClineTile, Complex,
};
use svg::{
    node::element::{path::Data, Circle, Group, Line, Path, Rectangle},
    Document, Node,
};

fn svg_cline(cline: &Cline) -> Box<dyn Node> {
    match cline.classify() {
        GeneralizedCircle::Circle { center, radius } => Box::new(
            Circle::new()
                .set("cx", center.real())
                .set("cy", center.imag())
                .set("r", radius),
        ),
        GeneralizedCircle::Line {
            unit_normal,
            distance,
        } => {
            let far_away: Complex = 1000.0.into();
            let tangent = Complex::I * unit_normal;
            let center: Complex = unit_normal * distance.into();
            let start: Complex = center + tangent * far_away.into();
            let end: Complex = center - tangent * far_away.into();
            let center: Complex = unit_normal * distance.into();
            Box::new(
                Line::new()
                    .set("x1", start.real())
                    .set("y1", start.imag())
                    .set("x2", end.real())
                    .set("y2", end.imag()),
            )
        }
    }
}

fn main() {
    let rotate = rotation(f64::consts::PI / 8.0).unwrap();
    let scale = scale(2.0).unwrap();

    let unit_circle = Cline::circle(Complex::Zero, 1.0);
    let real_axis = Cline::line(Complex::I, 0.0).unwrap();
    let imaginary_axis = Cline::line(Complex::ONE, 0.0).unwrap();

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

    println!("{}", meridians);

    let axes = Group::new()
        .add(svg_cline(&unit_circle))
        .add(svg_cline(&real_axis))
        .add(svg_cline(&imaginary_axis))
        .set("fill", "none")
        .set("stroke", "white")
        .set("stroke-width", "0.5%");

    let boundary = Rectangle::new()
        .set("x", "-50%")
        .set("y", "-50%")
        .set("width", "100%")
        .set("height", "100%")
        .set("fill", "black");

    let flip_y = Group::new()
        .set("transform", "scale(1, -1)")
        .add(axes)
        .add(meridians);

    let document = Document::new()
        .set("width", 500)
        .set("height", 500)
        .set("viewBox", (-2, -2, 4, 4))
        .add(boundary)
        .add(flip_y);

    svg::save("grid.svg", &document).unwrap();
}

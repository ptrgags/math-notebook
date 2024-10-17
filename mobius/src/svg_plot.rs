use svg::{
    node::element::{Circle, Group, Line},
    Node,
};

use crate::{
    cline::{Cline, GeneralizedCircle},
    Complex,
};

const FAR_AWAY: f64 = 1000.0;

pub fn svg_cline(cline: &Cline) -> Box<dyn Node> {
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
            let far_away: Complex = FAR_AWAY.into();
            let tangent = Complex::I * unit_normal;
            let center: Complex = unit_normal * distance.into();
            let start: Complex = center + tangent * far_away.into();
            let end: Complex = center - tangent * far_away.into();
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

pub fn make_axes() -> Group {
    Group::new()
        .add(svg_cline(&Cline::unit_circle()))
        .add(svg_cline(&Cline::real_axis()))
        .add(svg_cline(&Cline::imag_axis()))
}

pub fn flip_y() -> Group {
    Group::new().set("transform", "scale(1, 1)")
}

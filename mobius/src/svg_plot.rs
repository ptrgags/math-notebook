use core::f64;

use svg::{
    node::element::{path::Data, Circle, Group, Line, Path},
    Node,
};

use crate::{
    cline::{Cline, GeneralizedCircle},
    cline_arc::{ClineArc, ClineArcGeometry},
    ClineTile, Complex,
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

pub fn svg_cline_tile(tile: &ClineTile) -> Vec<Box<dyn Node>> {
    tile.get_clines().iter().map(|x| svg_cline(x)).collect()
}

pub fn svg_ray(start: Complex, direction: Complex) -> Line {
    let end = direction * FAR_AWAY.into();

    Line::new()
        .set("x1", start.real())
        .set("y1", start.imag())
        .set("x2", end.real())
        .set("y2", end.imag())
}

pub fn svg_line_segment(start: Complex, end: Complex) -> Line {
    Line::new()
        .set("x1", start.real())
        .set("y1", start.imag())
        .set("x2", end.real())
        .set("y2", end.imag())
}

pub fn svg_circular_arc(center: Complex, radius: f64, start_angle: f64, end_angle: f64) -> Path {
    let start = center + Complex::from_polar(radius, start_angle);
    let start_x = start.real();
    let start_y = start.imag();

    let counterclockwise = end_angle - start_angle > 0.0;
    let large_arc = (end_angle - start_angle) % f64::consts::TAU > f64::consts::PI;

    const NO_ROTATION: f64 = 0.0;
    let end = center + Complex::from_polar(radius, end_angle);
    let end_x = end.real();
    let end_y = end.imag();

    let data = Data::new().move_to((start_x, start_y)).elliptical_arc_to((
        radius,
        radius,
        NO_ROTATION,
        large_arc as u8,
        counterclockwise as u8,
        end_x,
        end_y,
    ));

    Path::new().set("d", data)
}

pub fn svg_cline_arc(cline_arc: &ClineArc) -> Box<dyn Node> {
    match cline_arc.classify() {
        ClineArcGeometry::CircularArc {
            center,
            radius,
            start_angle,
            end_angle,
        } => Box::new(svg_circular_arc(center, radius, start_angle, end_angle)),
        ClineArcGeometry::LineSegment { a, b } => Box::new(svg_line_segment(a, b)),
        ClineArcGeometry::Ray { start, dir } => Box::new(svg_ray(start, dir)),
        ClineArcGeometry::RayPair { a, b, dir_ab } => {
            let ray_a = svg_ray(a, -dir_ab);
            let ray_b = svg_ray(b, dir_ab);
            Box::new(Group::new().add(ray_a).add(ray_b))
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
    Group::new().set("transform", "scale(1, -1)")
}

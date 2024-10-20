use core::f64;

use svg::{
    node::element::{path::Data, Circle, Group, Line, Path, Rectangle},
    Document, Node,
};

use crate::{
    cline::{Cline, GeneralizedCircle},
    cline_arc::{ClineArc, ClineArcGeometry},
    cline_tile::{ClineArcTile, ClineTile},
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

pub fn svg_cline_arc_tile(tile: &ClineArcTile) -> Vec<Box<dyn Node>> {
    tile.get_arcs().iter().map(|x| svg_cline_arc(x)).collect()
}

pub fn svg_cline_arc_tiles(tiles: &[ClineArcTile]) -> Vec<Box<dyn Node>> {
    tiles.iter().flat_map(|x| svg_cline_arc_tile(x)).collect()
}

pub fn add_geometry(group: Group, geometry: Vec<Box<dyn Node>>) -> Group {
    geometry.into_iter().fold(group, |group, x| group.add(x))
}

pub fn style_lines(color: &str, width: &str) -> Group {
    Group::new()
        .set("stroke", color)
        .set("stroke-width", width)
        .set("fill", "none")
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

pub fn make_card(center: Complex, half_width: f64) -> Document {
    // My usual art trading card format for my website is 500x700px
    const WIDTH: f64 = 500.0;
    const HEIGHT: f64 = 700.0;
    const ASPECT_RATIO: f64 = WIDTH / HEIGHT;

    let half_height = half_width / ASPECT_RATIO;
    let offset = Complex::new(half_width, half_height);

    let top_left = center.conj() - offset;
    let dimensions = offset + offset;

    let view_box = (
        top_left.real(),
        top_left.imag(),
        dimensions.real(),
        dimensions.imag(),
    );

    let background = Rectangle::new()
        .set("x", top_left.real())
        .set("y", top_left.imag())
        .set("width", "100%")
        .set("height", "100%")
        .set("fill", "black")
        .set("stroke", "none");

    Document::new()
        .set("width", 500)
        .set("height", 700)
        .set("viewBox", view_box)
        .add(background)
}

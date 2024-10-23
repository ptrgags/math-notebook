use core::f64;
use std::path;

use svg::{
    node::element::{path::Data, Circle as SvgCircle, Group, Line as SvgLine, Path, Rectangle},
    Document, Node,
};

use crate::{
    cline::{Cline, GeneralizedCircle}, cline_arc::{ClineArc, ClineArcGeometry}, cline_tile::{ClineArcTile, ClineTile}, geometry::{Circle, CircularArc, DoubleRay, Line, LineSegment, Ray}, path_element::Shape, style::Style, Complex
};

const FAR_AWAY: f64 = 1000.0;

pub fn svg_cline(cline: &Cline) -> Box<dyn Node> {
    match cline.classify() {
        GeneralizedCircle::Circle(circle) => svg_circle(circle),
        GeneralizedCircle::Line(line) => svg_line(line)
    }
}

fn svg_cline_arc(cline_arc: &ClineArc) -> Box<dyn Node> {
    match cline_arc.classify() {
        ClineArcGeometry::CircularArc(arc) => svg_circular_arc(arc),
        ClineArcGeometry::LineSegment(segment) => svg_line_segment(segment),
        ClineArcGeometry::FromInfinity(ray)=> svg_ray(ray),
        ClineArcGeometry::ToInfinity(ray)=> svg_ray(ray),
        ClineArcGeometry::ThruInfinity(DoubleRay(ray_a, ray_b))=> {
            Box::new(Group::new().add(svg_ray(ray_a)).add(svg_ray(ray_b)))
        }
    }
}

pub struct SvgNode(Box<dyn Node>);

fn svg_circle(circle: Circle) -> Box<dyn Node> {
    let Circle{center, radius}= circle;
    Box::new(SvgCircle::new()
        .set("cx", center.real())
        .set("cy", center.imag())
        .set("r", radius))
}

fn svg_line(line: Line) -> Box<dyn Node> {
    let Line{unit_normal, distance} = line;
    let far_away: Complex = FAR_AWAY.into();
    let tangent = Complex::I * unit_normal;
    let center: Complex = unit_normal * distance.into();
    let start: Complex = center + tangent * far_away.into();
    let end: Complex = center - tangent * far_away.into();
    Box::new(
        SvgLine::new()
            .set("x1", start.real())
            .set("y1", start.imag())
            .set("x2", end.real())
            .set("y2", end.imag()),
    )
}

fn svg_ray(ray: Ray) -> Box<dyn Node> {
    let Ray{start, unit_dir} = ray;
    let end = unit_dir * FAR_AWAY.into();

    Box::new(SvgLine::new()
        .set("x1", start.real())
        .set("y1", start.imag())
        .set("x2", end.real())
        .set("y2", end.imag()))
}

fn svg_circular_arc(arc: CircularArc) -> Box<dyn Node> {
    let CircularArc{
        circle: Circle{center, radius},
        start_angle,end_angle
    } = arc;
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

    Box::new(Path::new().set("d", data))
}

fn svg_line_segment(line: LineSegment) -> Box<dyn Node> {
    let LineSegment{start, end} = line;
    Box::new(SvgLine::new()
        .set("x1", start.real())
        .set("y1", start.imag())
        .set("x2", end.real())
        .set("y2", end.imag()))
}

fn svg_point(z: Complex) -> Box<dyn Node> {
    const POINT_RADIUS: &str = "0.25%";
    Box::new(SvgCircle::new()
        .set("cx", z.real())
        .set("cy", z.imag())
        .set("r", POINT_RADIUS))
}

impl From<Shape> for SvgNode {
    fn from(value: Shape) -> Self {
        match value {
            Shape::Point(z) => SvgNode(svg_point(z)),
            //Shape::Text(label, complex) => todo!(),
            Shape::Circle(circle) => SvgNode(svg_circle(circle)),
            Shape::LineSegment(line_segment) => SvgNode(svg_line_segment(line_segment)),
            Shape::CircularArc(circular_arc) => SvgNode(svg_circular_arc(circular_arc)),
            //Shape::Polyline(vec) => todo!(),
            //Shape::Polygon(vec) => todo!(),
        }
    }
}



pub struct SvgNodes(Vec<Box<dyn Node>>);

/// Promote a single node into a collection
impl From<SvgNode> for SvgNodes {
    fn from(value: SvgNode) -> Self {
        let SvgNode(node) = value;
        SvgNodes(vec![node])
    }
}

/// Take a bunch of individual nodes and turn it into one collection
impl From<Vec<SvgNode>> for SvgNodes {
    fn from(value: Vec<SvgNode>) -> Self {
        SvgNodes(value.into_iter().map(|SvgNode(node)| node).collect())
    }
}

impl From<&ClineArcTile> for SvgNodes {
    fn from(tile: &ClineArcTile) -> Self {
        SvgNodes(tile.get_arcs().iter().map(|x| svg_cline_arc(x)).collect())
    }
}

impl From<&[ClineArcTile]> for SvgNodes {
    fn from(tiles: &[ClineArcTile]) -> Self {
        SvgNodes(
            tiles
                .iter()
                .flat_map(|x| {
                    let SvgNodes(nodes) = x.into();
                    nodes
                })
                .collect(),
        )
    }
}

impl From<&ClineTile> for SvgNodes {
    fn from(tile: &ClineTile) -> Self {
        SvgNodes(tile.get_clines().iter().map(|x| svg_cline(x)).collect())
    }
}

impl From<&[ClineTile]> for SvgNodes {
    fn from(tiles: &[ClineTile]) -> Self {
        SvgNodes(
            tiles
                .iter()
                .flat_map(|x| {
                    let SvgNodes(nodes) = x.into();
                    nodes
                })
                .collect(),
        )
    }
}

pub fn add_geometry(group: Group, geometry: impl Into<SvgNodes>) -> Group {
    let SvgNodes(nodes) = geometry.into();
    nodes.into_iter().fold(group, |group, x| group.add(x))
}

pub fn style_group(style: Style) -> Group {
    let mut group = Group::new();

    let Style{stroke, fill, width_percent} = style;
    if let Some(color) = stroke {
        group = group.set("stroke", color.to_string());
    }

    if let Some(color) = fill {
        group = group.set("fill", color.to_string());
    } else {
        group = group.set("fill", "none");
    }

    if let Some(percent) = width_percent {
        group = group.set("stroke-width", format!("{}%", percent));
    }

    group
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

pub struct View<'a>(pub &'a str, pub f64, pub f64, pub f64);

pub fn render_views<P: AsRef<path::Path>>(
    output_dir: P,
    prefix: &str,
    views: &[View],
    geometry: Group,
) -> Result<(), std::io::Error> {
    for View(label, x, y, half_width) in views {
        let flipped = flip_y().add(geometry.clone());
        let doc = make_card(Complex::new(*x, *y), *half_width).add(flipped);

        let separator = if prefix == "" { "" } else { "_" };
        let filename = format!("{}{}{}.svg", prefix, separator, label);
        let path = output_dir.as_ref().join(path::Path::new(&filename));
        svg::save(path, &doc)?
    }

    Ok(())
}

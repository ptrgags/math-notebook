use core::f64;
use std::path;

use svg::{
    node::element::{path::Data, Circle as SvgCircle, Group, Line as SvgLine, Path, Rectangle},
    Document, Node,
};

use crate::{
    geometry::{Circle, CircularArc, LineSegment},
    rendering::{RenderPrimitive, Renderable, Style},
    transformable::{Cline, ClineTile, Motif},
    Complex,
};
pub struct SvgNode(Box<dyn Node>);

fn svg_circle(circle: Circle) -> Box<dyn Node> {
    let Circle { center, radius } = circle;
    Box::new(
        SvgCircle::new()
            .set("cx", center.real())
            .set("cy", center.imag())
            .set("r", radius),
    )
}

fn svg_circular_arc(arc: CircularArc) -> Box<dyn Node> {
    let CircularArc {
        circle: Circle { center, radius },
        angle_a: start_angle,
        angle_c: end_angle,
        ..
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
    let LineSegment { start, end } = line;
    Box::new(
        SvgLine::new()
            .set("x1", start.real())
            .set("y1", start.imag())
            .set("x2", end.real())
            .set("y2", end.imag()),
    )
}

fn svg_point(z: Complex) -> Box<dyn Node> {
    const POINT_RADIUS: &str = "0.25%";
    Box::new(
        SvgCircle::new()
            .set("cx", z.real())
            .set("cy", z.imag())
            .set("r", POINT_RADIUS),
    )
}

impl From<RenderPrimitive> for SvgNode {
    fn from(value: RenderPrimitive) -> Self {
        match value {
            RenderPrimitive::Point(z) => SvgNode(svg_point(z)),
            RenderPrimitive::Circle(circle) => SvgNode(svg_circle(circle)),
            RenderPrimitive::LineSegment(line_segment) => SvgNode(svg_line_segment(line_segment)),
            RenderPrimitive::CircularArc(circular_arc) => SvgNode(svg_circular_arc(circular_arc)),
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

impl<T: Renderable> From<&T> for SvgNodes {
    fn from(value: &T) -> Self {
        let nodes: Vec<SvgNode> = value
            .bake_geometry()
            .iter()
            .map(|x| SvgNode::from(*x))
            .collect();

        nodes.into()
    }
}

impl<T: Renderable> From<&[T]> for SvgNodes {
    fn from(value: &[T]) -> Self {
        SvgNodes(
            value
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

    let Style {
        stroke,
        fill,
        width_percent,
    } = style;
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

pub fn style_geometry(style: Style, geometry: impl Into<SvgNodes>) -> Group {
    let mut svg = style_group(style);
    svg = add_geometry(svg, geometry);

    svg
}

pub fn style_motif(motif: &Motif, styles: &[Style]) -> Group {
    let groups: Vec<Group> = motif
        .iter()
        .map(|(tile, style_id)| style_geometry(styles[*style_id], tile))
        .collect();
    union(groups)
}

pub fn style_motifs(motifs: &[Motif], styles: &[Style]) -> Group {
    let groups: Vec<Group> = motifs
        .iter()
        .map(|motif| style_motif(motif, styles))
        .collect();
    union(groups)
}

pub fn union(groups: Vec<Group>) -> Group {
    groups
        .into_iter()
        .fold(Group::new(), |group, x| group.add(x))
}

pub fn make_axes() -> Group {
    let tile = ClineTile::new(vec![
        Cline::unit_circle(),
        Cline::real_axis(),
        Cline::imag_axis(),
    ]);

    let mut axes = Group::new();
    axes = add_geometry(axes, &tile);

    axes
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

        let separator = if label.is_empty() { "" } else { "_" };
        let filename = format!("{}{}{}.svg", prefix, separator, label);
        let path = output_dir.as_ref().join(path::Path::new(&filename));
        svg::save(path, &doc)?
    }

    Ok(())
}

use core::f64;

use svg::{
    node::element::{path::Data, Circle as SvgCircle, Group, Line as SvgLine, Path},
    Node,
};

use crate::{
    primitive::{CircularArc, CircularArcTo, PathCommand, RenderPrimitive, Renderable},
    style::Style,
};

pub struct SvgNode(pub Box<dyn Node>);

fn svg_circle(cx: f64, cy: f64, radius: f64) -> Box<dyn Node> {
    Box::new(
        SvgCircle::new()
            .set("cx", cx)
            .set("cy", cy)
            .set("r", radius),
    )
}

fn svg_arc_parameters(arc: CircularArcTo) -> (f64, f64, f64, u8, u8, f64, f64) {
    const NO_ROTATION: f64 = 0.0;
    (
        arc.radius,
        arc.radius,
        NO_ROTATION,
        arc.large_arc as u8,
        arc.counterclockwise as u8,
        arc.end_x,
        arc.end_y,
    )
}

fn svg_circular_arc(arc: CircularArc) -> Box<dyn Node> {
    let arc_params = svg_arc_parameters(arc.arc_to);
    let data = Data::new()
        .move_to((arc.start_x, arc.start_y))
        .elliptical_arc_to(arc_params);
    Box::new(Path::new().set("d", data))
}

fn svg_line_segment(x1: f64, y1: f64, x2: f64, y2: f64) -> Box<dyn Node> {
    Box::new(
        SvgLine::new()
            .set("x1", x1)
            .set("y1", y1)
            .set("x2", x2)
            .set("y2", y2),
    )
}

fn svg_point(x: f64, y: f64) -> Box<dyn Node> {
    const POINT_RADIUS: &str = "0.25%";
    Box::new(
        SvgCircle::new()
            .set("cx", x)
            .set("cy", y)
            .set("r", POINT_RADIUS),
    )
}

fn svg_polygon(commands: &[PathCommand]) -> Box<dyn Node> {
    let mut path_data = Data::new();

    for &cmd in commands.iter() {
        match cmd {
            PathCommand::MoveTo { x, y } => path_data = path_data.move_to((x, y)),
            PathCommand::LineTo { x, y } => path_data = path_data.line_to((x, y)),
            PathCommand::ArcTo(arc) => {
                let arc_params = svg_arc_parameters(arc);
                path_data = path_data.elliptical_arc_to(arc_params);
            }
        }
    }

    path_data = path_data.close();

    let path = Path::new().set("d", path_data);
    Box::new(path)
}

/*
pub fn add_geometry(group: Group, geometry: impl Into<SvgNodes>) -> Group {
    let SvgNodes(nodes) = geometry.into();
    nodes.into_iter().fold(group, |group, x| group.add(x))
}
    */

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

/*
pub fn style_geometry(style: Style, geometry: impl Into<SvgNodes>) -> Group {
    let mut svg = style_group(style);
    svg = add_geometry(svg, geometry);

    svg
}*/

fn svg_group(primitives: &[RenderPrimitive], style: Style) -> Box<dyn Node> {
    todo!();
    /*
    let group = style_group(style);
    let with_children = primitives
        .iter()
        .fold(group, |group, x| add_geometry(group, *x));
    Box::new(with_children)
    */
}

impl From<RenderPrimitive> for SvgNode {
    fn from(value: RenderPrimitive) -> Self {
        use RenderPrimitive::*;
        match value {
            Point { x, y } => SvgNode(svg_point(x, y)),
            Circle { x, y, radius } => SvgNode(svg_circle(x, y, radius)),
            LineSegment { x1, y1, x2, y2 } => SvgNode(svg_line_segment(x1, y1, x2, y2)),
            CircularArc(circular_arc) => SvgNode(svg_circular_arc(circular_arc)),
            Polygon(commands) => SvgNode(svg_polygon(&commands)),
            Group(primitives, style) => SvgNode(svg_group(&primitives, style)),
        }
    }
}

/*
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
        let baked = value.bake_geometry().expect("couldn't bake primitive");
        let nodes: Vec<SvgNode> = baked.iter().map(|x| SvgNode::from(x.clone())).collect();
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
    */

/*
pub fn style_motif<T: Renderable>(motif: &Motif<T>, styles: &[Style]) -> Group {
    let groups: Vec<Group> = motif
        .iter()
        .map(|(tile, style_id)| style_geometry(styles[*style_id], tile))
        .collect();
    union(groups)
}

pub fn style_motifs<T: Renderable>(motifs: &[Motif<T>], styles: &[Style]) -> Group {
    let groups: Vec<Group> = motifs
        .iter()
        .map(|motif| style_motif(motif, styles))
        .collect();
    union(groups)
}
*/

pub fn union(groups: Vec<Group>) -> Group {
    groups
        .into_iter()
        .fold(Group::new(), |group, x| group.add(x))
}

/*
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
    */

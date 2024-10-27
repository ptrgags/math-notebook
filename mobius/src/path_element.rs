use crate::{
    geometry::{Circle, CircularArc, LineSegment},
    Complex,
};

/// A single directed edge that can be part of a path
pub enum PathElement {
    LineSegment(LineSegment),
    CircularArc(CircularArc),
    Gap(LineSegment),
}

pub enum Shape {
    Point(Complex),
    //Text(String, Complex),
    Circle(Circle),
    LineSegment(LineSegment),
    CircularArc(CircularArc),
    //Polyline(Vec<PathElement>),
    //Polygon(Vec<PathElement>),
}

use crate::Complex;

/// A geometry primitive
pub trait Geometry {}

/// Directed edge. If a geometry type defines this,
/// then it can be used to make a polygon
pub trait DirectedEdge {
    fn start(&self) -> Complex;
    fn end(&self) -> Complex;
}

pub struct Circle {
    pub center: Complex,
    pub radius: f64,
}

impl Circle {
    pub fn get_point(&self, theta: f64) -> Complex {
        self.center + Complex::from_polar(self.radius, theta)
    }
}

impl Geometry for Circle{}

pub struct Line {
    pub unit_normal: Complex,
    pub distance: f64
}

impl Geometry for Line{}

pub struct CircularArc {
    pub circle: Circle,
    pub start_angle: f64,
    pub end_angle: f64
}

impl Geometry for CircularArc{}
impl DirectedEdge for CircularArc{
    fn start(&self) -> Complex {
        self.circle.get_point(self.start_angle)
    }

    fn end(&self) -> Complex {
        todo!()
    }
}

pub struct LineSegment {
    pub start: Complex,
    pub end: Complex,
}

impl Geometry for LineSegment{}
impl DirectedEdge for LineSegment{
    fn start(&self) -> Complex {
        self.start
    }

    fn end(&self) -> Complex {
        self.end
    }
}

pub struct Ray {
    pub start: Complex,
    pub unit_dir: Complex
}

impl Geometry for Ray{}
impl DirectedEdge for Ray {
    fn start(&self) -> Complex {
        self.start
    }

    fn end(&self) -> Complex {
        Complex::Infinity
    }
}

pub struct DoubleRay(pub Ray, pub Ray);

impl Geometry for DoubleRay{}
impl DirectedEdge for DoubleRay {
    fn start(&self) -> Complex {
        let Self(a, _) = self;
        a.start
    }

    fn end(&self) -> Complex {
        let Self(_, b) = self;
        b.start
    }
}
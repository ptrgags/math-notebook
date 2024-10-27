use crate::Complex;

/// A geometry primitive
pub trait Geometry {}

/// Directed edge. If a geometry type defines this,
/// then it can be used to make a polygon
pub trait DirectedEdge {
    fn start(&self) -> Complex;
    fn end(&self) -> Complex;
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Circle {
    pub center: Complex,
    pub radius: f64,
}

impl Circle {
    pub fn get_point(&self, theta: f64) -> Complex {
        self.center + Complex::from_polar(self.radius, theta)
    }

    pub fn get_angle(&self, point: Complex) -> Option<f64> {
        (point - self.center).arg()
    }
}

impl Geometry for Circle {}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Line {
    pub unit_normal: Complex,
    pub distance: f64,
}

impl Geometry for Line {}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct CircularArc {
    pub circle: Circle,
    pub start_angle: f64,
    pub end_angle: f64,
}

impl Geometry for CircularArc {}
impl DirectedEdge for CircularArc {
    fn start(&self) -> Complex {
        self.circle.get_point(self.start_angle)
    }

    fn end(&self) -> Complex {
        todo!()
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct LineSegment {
    pub start: Complex,
    pub end: Complex,
}

impl Geometry for LineSegment {}
impl DirectedEdge for LineSegment {
    fn start(&self) -> Complex {
        self.start
    }

    fn end(&self) -> Complex {
        self.end
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Ray {
    pub start: Complex,
    pub unit_dir: Complex,
}

impl Geometry for Ray {}
impl DirectedEdge for Ray {
    fn start(&self) -> Complex {
        self.start
    }

    fn end(&self) -> Complex {
        Complex::Infinity
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct DoubleRay(pub Ray, pub Ray);

impl Geometry for DoubleRay {}
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

pub enum PolyEdge {
    Polyline(Vec<Box<dyn DirectedEdge>>),
    Polygon(Vec<Box<dyn DirectedEdge>>),
}

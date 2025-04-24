#[derive(Clone, Copy)]
pub struct CircularArcTo {
    pub radius: f64,
    pub large_arc: bool,
    pub counterclockwise: bool,
    pub end_x: f64,
    pub end_y: f64,
}

#[derive(Clone, Copy)]
pub enum PathCommand {
    MoveTo { x: f64, y: f64 },
    LineTo { x: f64, y: f64 },
    ArcTo(CircularArcTo),
}

#[derive(Clone, Copy)]
pub struct CircularArc {
    pub start_x: f64,
    pub start_y: f64,
    pub arc_to: CircularArcTo,
}

#[derive(Clone)]
pub enum RenderPrimitive {
    Point { x: f64, y: f64 },
    Circle { x: f64, y: f64, radius: f64 },
    LineSegment { x1: f64, y1: f64, x2: f64, y2: f64 },
    CircularArc(CircularArc),
    Polygon(Vec<PathCommand>),
}

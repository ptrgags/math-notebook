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

const FAR_AWAY: f64 = 10000.0;

impl RenderPrimitive {
    /// Render a ray as a line segment from the start point to far off the canvas.
    pub fn make_ray(ray: Ray) -> Self {
        let Ray { start, unit_dir } = ray;
        let end = *unit_dir.get() * FAR_AWAY.into();

        Self::LineSegment(LineSegment { start, end })
    }

    pub fn make_line(line: Line) -> Self {
        let Line {
            unit_normal,
            distance,
        } = line;

        let far_away: Complex = FAR_AWAY.into();
        let &tangent = unit_normal.rot90().get();
        let center: Complex = *unit_normal.get() * distance.into();
        let start: Complex = center + tangent * far_away;
        let end: Complex = center - tangent * far_away;

        Self::LineSegment(LineSegment { start, end })
    }
}

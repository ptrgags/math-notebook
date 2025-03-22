use crate::{
    geometry::{Circle, CircularArc, Line, LineSegment, Ray},
    Complex,
};

#[derive(Clone, Copy)]
pub enum PathCommand {
    MoveTo(Complex),
    LineTo(Complex),
    ArcTo(CircularArc),
}

#[derive(Clone)]
pub enum RenderPrimitive {
    Point(Complex),
    Circle(Circle),
    LineSegment(LineSegment),
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

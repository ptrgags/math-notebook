use crate::{
    geometry::{Circle, CircularArc, Line, LineSegment, Ray},
    Complex,
};

#[derive(Clone, Copy)]
pub enum RenderPrimitive {
    Point(Complex),
    Circle(Circle),
    LineSegment(LineSegment),
    CircularArc(CircularArc),
}

const FAR_AWAY: f64 = 10000.0;

impl RenderPrimitive {
    /// Render a ray as a line segment from the start point to far off the canvas.
    pub fn make_ray(ray: Ray) -> Self {
        let Ray { start, unit_dir } = ray;
        let end = unit_dir * FAR_AWAY.into();

        Self::LineSegment(LineSegment { start, end })
    }

    pub fn make_line(line: Line) -> Self {
        let Line {
            unit_normal,
            distance,
        } = line;

        let far_away: Complex = FAR_AWAY.into();
        let tangent = Complex::I * unit_normal;
        let center: Complex = unit_normal * distance.into();
        let start: Complex = center + tangent * far_away.into();
        let end: Complex = center - tangent * far_away.into();

        Self::LineSegment(LineSegment { start, end })
    }
}

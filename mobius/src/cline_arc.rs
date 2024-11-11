use std::{error::Error, fmt::Display};

use crate::{
    geometry::{
        ArcAngles, ArcDirection, Circle, CircularArc, DoubleRay, GeneralizedCircle, Line,
        LineSegment, Ray,
    },
    isogonal::Isogonal,
    rendering::{RenderPrimitive, Renderable},
    transformable::{Cline, Transformable},
    Complex,
};

#[derive(Debug)]
pub enum ClineArcError {
    // To reduce the size of the enum, the first parameter is a pretty-printed
    // string representation of the cline such that you can see the numeric
    // values for debugging
    PossiblePrecisionError(String, Box<dyn Error + 'static>),
}

impl Display for ClineArcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PossiblePrecisionError(cline_formatted, inner_error) => {
                write!(
                    f,
                    "possible precision issue for ClineArc:\n{}\ncaused by: {}",
                    cline_formatted, inner_error
                )
            }
        }
    }
}

impl Error for ClineArcError {}

#[derive(Clone, Copy, Debug)]
pub enum ClineArcGeometry {
    CircularArc(CircularArc),
    LineSegment(LineSegment),
    // Line segment that starts at infinity and ends at another point
    FromInfinity(Ray),
    // Line segment that starts at a point and ends at infinity
    ToInfinity(Ray),
    // Line segment through infinity. The first ray is from start -> inf,
    // the second ray is from inf -> end
    ThruInfinity(DoubleRay),
}

impl Display for ClineArcGeometry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClineArcGeometry::CircularArc(arc) => arc.fmt(f),
            ClineArcGeometry::LineSegment(LineSegment { start, end }) => {
                write!(f, "Segment({} -> {})", start, end)
            }
            ClineArcGeometry::FromInfinity(Ray { start, unit_dir }) => {
                write!(f, "Ray(inf --{}-> {})", unit_dir, start)
            }
            ClineArcGeometry::ToInfinity(Ray { start, unit_dir }) => {
                write!(f, "Ray({} --{}-> inf)", start, unit_dir)
            }
            ClineArcGeometry::ThruInfinity(DoubleRay(a_inf, b_inf)) => {
                let Ray { start: a, .. } = a_inf;
                let Ray {
                    start: b,
                    unit_dir: dir_ab,
                } = b_inf;
                write!(f, "RayPair(<--{} {}--{}->", a, b, dir_ab)
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ClineArc {
    cline: Cline,
    a: Complex,
    b: Complex,
    c: Complex,
}

impl ClineArc {
    fn compute_line_geometry(&self) -> ClineArcGeometry {
        if let Complex::Infinity = self.a {
            // ray goes inf -> b -> c
            // but it looks like inf <- c
            let to_infinity = (self.b - self.c).normalize().unwrap();
            return ClineArcGeometry::FromInfinity(Ray {
                start: self.c,
                unit_dir: to_infinity,
            });
        }

        if let Complex::Infinity = self.c {
            // ray goes a -> b -> inf
            let to_infinity = (self.b - self.a).normalize().unwrap();
            return ClineArcGeometry::ToInfinity(Ray {
                start: self.a,
                unit_dir: to_infinity,
            });
        }

        if let Complex::Infinity = self.b {
            // ray goes    inf <- a    c -> inf
            let ac = (self.c - self.a).normalize().unwrap();
            return ClineArcGeometry::ThruInfinity(DoubleRay(
                Ray {
                    start: self.a,
                    unit_dir: -ac,
                },
                Ray {
                    start: self.c,
                    unit_dir: ac,
                },
            ));
        }

        // All three points are finite so now we we need to check if
        // if they're in order a -> b -> c. If not, then
        // the line goes through infinity like
        //   inf <- a   c <- b <- inf
        //   inf <- b <- a  c -> inf
        // in which case we just want a ray pair through a and c.
        //
        // we can just check a -> b and b -> c. If the dot product is
        // positive, then the points are in order.
        let in_order = Complex::dot(self.b - self.a, self.c - self.b) > 0.0;

        if in_order {
            ClineArcGeometry::LineSegment(LineSegment {
                start: self.a,
                end: self.c,
            })
        } else {
            let ac = (self.c - self.a).normalize().unwrap();
            ClineArcGeometry::ThruInfinity(DoubleRay(
                Ray {
                    start: self.a,
                    unit_dir: -ac,
                },
                Ray {
                    start: self.c,
                    unit_dir: ac,
                },
            ))
        }
    }

    /// Implementation detail - to go from the ClineArc representation
    /// to a CircularArc, it requires computing the arc a -> b -> c. the middle
    /// point is necessary to disambiguate clockwise from counter-clockwise
    fn compute_circle_geometry(&self, circle: Circle) -> ClineArcGeometry {
        let &Self { a, b, c, .. } = self;

        // Determine if the 3 points circulate counterclockwise or
        // clockwise by forming a triangle ABC and computing
        // (the magnitude of) the wedge product.
        let ab = b - a;
        let ac = c - a;
        let ccw = Complex::wedge(ab, ac) > 0.0;

        // Get the raw angles
        let theta_a = circle.get_angle(a).unwrap();
        let theta_c = circle.get_angle(c).unwrap();

        let direction = if ccw {
            ArcDirection::Counterclockwise
        } else {
            ArcDirection::Clockwise
        };

        let angles = ArcAngles::from_raw_angles(theta_a, theta_c, direction);
        let arc = CircularArc::new(circle, angles);

        ClineArcGeometry::CircularArc(arc)
    }

    pub fn classify(&self) -> ClineArcGeometry {
        match self.cline.classify() {
            GeneralizedCircle::Line(_) => self.compute_line_geometry(),
            GeneralizedCircle::Circle(circle) => self.compute_circle_geometry(circle),
        }
    }
}

impl From<CircularArc> for ClineArc {
    fn from(value: CircularArc) -> Self {
        let CircularArc { circle, angles } = value;
        let Circle { center, radius } = circle;
        let ArcAngles(angle_a, angle_c) = angles;
        let angle_b = 0.5 * (angle_a + angle_c);
        let a = center + Complex::from_polar(radius, angle_a);
        let b = center + Complex::from_polar(radius, angle_b);
        let c = center + Complex::from_polar(radius, angle_c);
        Self {
            cline: circle.into(),
            a,
            b,
            c,
        }
    }
}

impl From<LineSegment> for ClineArc {
    fn from(value: LineSegment) -> Self {
        let LineSegment { start, end } = value;

        let line = Line::from(value);
        let midpoint = (start + end) * (0.5).into();

        Self {
            cline: line.into(),
            a: start,
            b: midpoint,
            c: end,
        }
    }
}

impl Transformable<Isogonal> for ClineArc {
    fn transform(&self, xform: Isogonal) -> Self {
        if cfg!(feature = "debug_cline_arcs") {
            let transformed = self.cline.transform(xform);
            println!(
                "---\nTransform:\n{}\nby {}\n to {}\n{}",
                self,
                xform,
                transformed,
                transformed.classify()
            );
        }

        Self {
            cline: self.cline.transform(xform),
            a: xform * self.a,
            b: xform * self.b,
            c: xform * self.c,
        }
    }
}

impl Renderable for ClineArc {
    fn bake_geometry(&self) -> Result<Vec<RenderPrimitive>, Box<dyn std::error::Error>> {
        let mut result = Vec::new();

        let (first, maybe_second) = match self.classify() {
            ClineArcGeometry::CircularArc(arc) => (RenderPrimitive::CircularArc(arc), None),
            ClineArcGeometry::LineSegment(line_segment) => {
                (RenderPrimitive::LineSegment(line_segment), None)
            }
            ClineArcGeometry::FromInfinity(ray) => (RenderPrimitive::make_ray(ray), None),
            ClineArcGeometry::ToInfinity(ray) => (RenderPrimitive::make_ray(ray), None),
            ClineArcGeometry::ThruInfinity(DoubleRay(start, end)) => {
                let first_ray = RenderPrimitive::make_ray(start);
                let second_ray = RenderPrimitive::make_ray(end);
                (first_ray, Some(second_ray))
            }
        };

        result.push(first);
        if let Some(x) = maybe_second {
            result.push(x);
        }

        Ok(result)
    }
}

impl Display for ClineArc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.classify().fmt(f)
    }
}

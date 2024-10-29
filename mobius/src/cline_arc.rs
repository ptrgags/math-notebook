use std::{f64::consts::TAU, fmt::Display};

use crate::{
    geometry::{Circle, CircularArc, DoubleRay, Line, LineSegment, Ray},
    isogonal::Isogonal,
    rendering::{RenderPrimitive, Renderable},
    transformable::{Cline, GeneralizedCircle, Transformable},
    Complex,
};

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
    pub fn classify(&self) -> ClineArcGeometry {
        match self.cline.classify() {
            GeneralizedCircle::Line(_) => {
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
            GeneralizedCircle::Circle(Circle { center, radius }) => {
                // Determine if the 3 points circulate counterclockwise or
                // clockwise by forming a triangle ABC and computing
                // (the magnitude of) the wedge product.
                let ab = self.b - self.a;
                let ac = self.c - self.a;
                let ccw = Complex::wedge(ab, ac) > 0.0;

                let theta_a = (self.a - center).arg().unwrap();
                let theta_c = (self.c - center).arg().unwrap();
                let a_bigger = theta_a > theta_c;

                let end_angle = if !ccw && !a_bigger {
                    theta_c - TAU
                } else if ccw && a_bigger {
                    theta_c + TAU
                } else {
                    theta_c
                };

                let theta_b = (self.b - center).arg().unwrap();

                ClineArcGeometry::CircularArc(CircularArc {
                    circle: Circle { center, radius },
                    angle_a: theta_a,
                    angle_b: theta_b,
                    angle_c: end_angle,
                })
            }
        }
    }
}

impl From<CircularArc> for ClineArc {
    fn from(value: CircularArc) -> Self {
        let CircularArc {
            circle,
            angle_a,
            angle_b,
            angle_c,
        } = value;

        let Circle { center, radius } = circle;
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

        let unit_tangent = (end - start).normalize().unwrap();
        let unit_normal = Complex::I * unit_tangent;

        let distance = Complex::dot(unit_normal, start);
        let line = Line::new(unit_normal, distance).unwrap();

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
        Self {
            cline: self.cline.transform(xform),
            a: xform * self.a,
            b: xform * self.b,
            c: xform * self.c,
        }
    }
}

impl Renderable for ClineArc {
    fn bake_geometry(&self) -> Vec<RenderPrimitive> {
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

        result
    }
}

impl Display for ClineArc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.classify().fmt(f)
    }
}

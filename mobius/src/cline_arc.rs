use std::{f64::consts::TAU, fmt::Display};

use crate::{
    cline::{Cline, GeneralizedCircle},
    Complex, Mobius,
};

#[derive(Clone, Copy, Debug)]
pub enum ClineArcGeometry {
    CircularArc {
        center: Complex,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
    },
    LineSegment {
        a: Complex,
        b: Complex,
    },
    // Ray from infinity to a point
    Ray {
        start: Complex,
        dir: Complex,
    },
    // Line segment through infinity
    RayPair {
        a: Complex,
        b: Complex,
        // Unit vector
        // Direction from a -> b. One ray
        // is b -> infinity in the ab direction
        // the other is infinity <- a in the-ab direction
        dir_ab: Complex,
    },
}

impl Display for ClineArcGeometry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClineArcGeometry::CircularArc {
                center,
                radius,
                start_angle,
                end_angle,
            } => write!(
                f,
                "Arc({}, {:.3}, {:.3}° -> {:.3}°)",
                center,
                radius,
                start_angle.to_degrees(),
                end_angle.to_degrees()
            ),
            ClineArcGeometry::LineSegment { a, b } => write!(f, "Segment({} -> {})", a, b),
            ClineArcGeometry::Ray { start, dir } => write!(f, "Ray({} --{}->)", start, dir),
            ClineArcGeometry::RayPair { a, b, dir_ab } => {
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
    pub fn from_circle_and_angles(
        center: Complex,
        radius: f64,
        theta_a: f64,
        theta_b: f64,
        theta_c: f64,
    ) -> Self {
        let circle = Cline::circle(center, radius);
        let a = center + Complex::from_polar(radius, theta_a);
        let b = center + Complex::from_polar(radius, theta_b);
        let c = center + Complex::from_polar(radius, theta_c);
        Self {
            cline: circle,
            a,
            b,
            c,
        }
    }

    pub fn line_segment(start: Complex, end: Complex) -> Self {
        let unit_tangent = (end - start).normalize().unwrap();
        let unit_normal = Complex::I * unit_tangent;

        let distance = Complex::dot(unit_normal, start);
        let line = Cline::line(unit_normal, distance).unwrap();

        let midpoint = (start + end) * (0.5).into();

        Self {
            cline: line,
            a: start,
            b: midpoint,
            c: end,
        }
    }

    pub fn classify(&self) -> ClineArcGeometry {
        match self.cline.classify() {
            GeneralizedCircle::Line {
                unit_normal: _,
                distance: _,
            } => {
                if let Complex::Infinity = self.a {
                    // ray goes inf -> b -> c
                    // but it looks like inf <- c
                    let to_infinity = (self.b - self.c).normalize().unwrap();
                    return ClineArcGeometry::Ray {
                        start: self.c,
                        dir: to_infinity,
                    };
                }

                if let Complex::Infinity = self.c {
                    // ray goes a -> b -> inf
                    let to_infinity = (self.b - self.a).normalize().unwrap();
                    return ClineArcGeometry::Ray {
                        start: self.a,
                        dir: to_infinity,
                    };
                }

                if let Complex::Infinity = self.b {
                    // ray goes    inf <- a    c -> inf
                    let ac = (self.c - self.a).normalize().unwrap();
                    return ClineArcGeometry::RayPair {
                        a: self.a,
                        b: self.c,
                        dir_ab: ac,
                    };
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
                    ClineArcGeometry::LineSegment {
                        a: self.a,
                        b: self.c,
                    }
                } else {
                    let ac = (self.c - self.a).normalize().unwrap();
                    ClineArcGeometry::RayPair {
                        a: self.a,
                        b: self.c,
                        dir_ab: ac,
                    }
                }
            }
            GeneralizedCircle::Circle { center, radius } => {
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
                ClineArcGeometry::CircularArc {
                    center,
                    radius,
                    start_angle: theta_a,
                    end_angle,
                }
            }
        }
    }

    pub fn transform(mobius: Mobius, arc: ClineArc) -> Self {
        Self {
            cline: Cline::transform(mobius, arc.cline),
            a: mobius * arc.a,
            b: mobius * arc.b,
            c: mobius * arc.c,
        }
    }
}

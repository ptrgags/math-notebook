use std::fmt::Display;

use crate::Complex;

use super::{circle::Circle, DirectedEdge, Geometry};

// Directed circular arc through 3 points on a circular arc
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct CircularArc {
    pub circle: Circle,
    pub angle_a: f64,
    pub angle_b: f64,
    pub angle_c: f64,
}

impl Geometry for CircularArc {}
impl DirectedEdge for CircularArc {
    fn start(&self) -> Complex {
        self.circle.get_point(self.angle_a)
    }

    fn end(&self) -> Complex {
        self.circle.get_point(self.angle_c)
    }
}

impl Display for CircularArc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let CircularArc {
            circle: Circle { center, radius },
            angle_a,
            angle_b,
            angle_c,
        } = self;
        write!(
            f,
            "Arc(c={}, r={:.3}, {:.3}° -> {:.3}° -> {:.3}°)",
            center,
            radius,
            angle_a.to_degrees(),
            angle_b.to_degrees(),
            angle_c.to_degrees()
        )
    }
}

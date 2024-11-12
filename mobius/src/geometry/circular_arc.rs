use std::fmt::Display;

use thiserror::Error;

use crate::Complex;

use super::{circle::Circle, ArcAngles, ArcAnglesParseError, ArcDirection, DirectedEdge, Geometry};

#[derive(Debug, Error)]
pub enum CircularArcError {
    #[error("{0}")]
    BadAngles(#[from] ArcAnglesParseError),
    #[error("duplicate point: {0}")]
    DuplicatePoint(Complex),
}

// Directed circular arc through 3 points on a circular arc
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct CircularArc {
    pub circle: Circle,
    pub angles: ArcAngles,
}

impl CircularArc {
    pub fn new(circle: Circle, angles: ArcAngles) -> Self {
        Self { circle, angles }
    }

    pub fn direction(&self) -> ArcDirection {
        self.angles.direction()
    }

    pub fn interpolate(&self, t: f64) -> Complex {
        self.circle.get_point(self.angles.interpolate(t))
    }

    pub fn reverse(&self) -> Self {
        Self {
            circle: self.circle,
            angles: self.angles.reverse(),
        }
    }

    pub fn complement(&self) -> Self {
        Self {
            circle: self.circle,
            angles: self.angles.complement(),
        }
    }
}

impl Geometry for CircularArc {}
impl DirectedEdge for CircularArc {
    fn start(&self) -> Complex {
        let ArcAngles(a, _) = self.angles;
        self.circle.get_point(a)
    }

    fn end(&self) -> Complex {
        let ArcAngles(_, b) = self.angles;
        self.circle.get_point(b)
    }
}

impl Display for CircularArc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let CircularArc {
            circle: Circle { center, radius },
            angles,
        } = self;
        write!(f, "Arc(c={}, r={:.3}, {})", center, radius, angles)
    }
}

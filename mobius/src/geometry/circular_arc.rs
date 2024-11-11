use std::{error::Error, fmt::Display};

use crate::Complex;

use super::{circle::Circle, ArcAngles, ArcAnglesParseError, ArcDirection, DirectedEdge, Geometry};

#[derive(Debug)]
pub enum CircularArcError {
    BadAngles(ArcAnglesParseError),
    DuplicatePoint(Complex),
    PointAtCenter(Complex),
}

impl Display for CircularArcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CircularArcError::BadAngles(err) => err.fmt(f),
            CircularArcError::DuplicatePoint(point) => write!(f, "Duplicate point: {}", point),
            CircularArcError::PointAtCenter(point) => {
                write!(f, "Point at center of circle not allowed: {}", point)
            }
        }
    }
}

impl From<ArcAnglesParseError> for CircularArcError {
    fn from(value: ArcAnglesParseError) -> Self {
        Self::BadAngles(value)
    }
}

impl Error for CircularArcError {}

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

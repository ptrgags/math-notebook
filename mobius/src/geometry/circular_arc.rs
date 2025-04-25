use std::{f64::consts::PI, fmt::Display};

use rendering::{
    primitive::PathPrimitive, CircularArc as ArcPrimitive, CircularArcTo, PathCommand,
    RenderPrimitive, Renderable,
};
use thiserror::Error;

use crate::Complex;

use super::{circle::Circle, ArcAngles, ArcAnglesError, ArcDirection, DirectedEdge, Geometry};

#[derive(Debug, Error)]
pub enum CircularArcError {
    #[error("{0}")]
    BadAngles(#[from] ArcAnglesError),
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

    fn get_arc_to(&self) -> CircularArcTo {
        let &CircularArc { circle, angles } = self;
        let ArcAngles(start_angle, end_angle) = angles;
        let end = self.end();

        let counterclockwise = angles.direction() == ArcDirection::Counterclockwise;
        // ArcAngles guarantees that the total angle of the arc is in [0, 2pi). If it's
        // greater than pi in magnitude, we want to stroke the long way around the circle.
        let large_arc = (end_angle - start_angle).abs() > PI;
        CircularArcTo {
            radius: circle.radius,
            large_arc,
            counterclockwise,
            end_x: end.real(),
            end_y: end.imag(),
        }
    }
}

impl Renderable for CircularArc {
    fn render(&self) -> Result<RenderPrimitive, Box<dyn std::error::Error>> {
        let start = self.start();
        Ok(RenderPrimitive::CircularArc(ArcPrimitive {
            start_x: start.real(),
            start_y: start.imag(),
            arc_to: self.get_arc_to(),
        }))
    }
}

impl PathPrimitive for CircularArc {
    fn to_path_command(&self) -> PathCommand {
        PathCommand::ArcTo(self.get_arc_to())
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

use rendering::{PathCommand, RenderPrimitive};

use crate::{interpolation::lerp_complex, Complex};

use super::{DirectedEdge, Geometry};

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct LineSegment {
    pub start: Complex,
    pub end: Complex,
}

impl LineSegment {
    pub fn new(start: Complex, end: Complex) -> Self {
        Self { start, end }
    }

    pub fn reverse(&self) -> Self {
        let &Self { start, end } = self;
        Self {
            start: end,
            end: start,
        }
    }

    pub fn interpolate(&self, t: f64) -> Complex {
        lerp_complex(self.start, self.end, t)
    }

    pub fn to_primitive(&self) -> RenderPrimitive {
        let &Self { start, end } = self;
        RenderPrimitive::LineSegment {
            x1: start.real(),
            y1: start.imag(),
            x2: end.real(),
            y2: end.imag(),
        }
    }

    pub fn to_path_command(&self) -> PathCommand {
        PathCommand::LineTo {
            x: self.end.real(),
            y: self.end.imag(),
        }
    }
}

impl Geometry for LineSegment {}
impl DirectedEdge for LineSegment {
    fn start(&self) -> Complex {
        self.start
    }

    fn end(&self) -> Complex {
        self.end
    }
}

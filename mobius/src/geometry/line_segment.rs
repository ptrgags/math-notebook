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

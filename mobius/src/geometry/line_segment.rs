use crate::Complex;

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

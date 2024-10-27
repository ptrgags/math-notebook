use crate::Complex;

use super::{DirectedEdge, Geometry};

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Ray {
    pub start: Complex,
    pub unit_dir: Complex,
}

impl Geometry for Ray {}
impl DirectedEdge for Ray {
    fn start(&self) -> Complex {
        self.start
    }

    fn end(&self) -> Complex {
        Complex::Infinity
    }
}

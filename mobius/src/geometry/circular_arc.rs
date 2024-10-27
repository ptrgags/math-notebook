use crate::Complex;

use super::{circle::Circle, DirectedEdge, Geometry};

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct CircularArc {
    pub circle: Circle,
    pub start_angle: f64,
    pub end_angle: f64,
}

impl Geometry for CircularArc {}
impl DirectedEdge for CircularArc {
    fn start(&self) -> Complex {
        self.circle.get_point(self.start_angle)
    }

    fn end(&self) -> Complex {
        todo!()
    }
}

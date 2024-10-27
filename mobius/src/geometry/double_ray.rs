use crate::Complex;

use super::{ray::Ray, DirectedEdge, Geometry};

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct DoubleRay(pub Ray, pub Ray);

impl Geometry for DoubleRay {}
impl DirectedEdge for DoubleRay {
    fn start(&self) -> Complex {
        let Self(a, _) = self;
        a.start
    }

    fn end(&self) -> Complex {
        let Self(_, b) = self;
        b.start
    }
}

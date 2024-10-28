use crate::isogonal::Isogonal;

use super::{ClineArcTile, Transformable};

pub struct Motif {
    parts: Vec<(ClineArcTile, usize)>
}

impl Motif {
    pub fn new(parts: Vec<(ClineArcTile, usize)>) -> Self {
        Self {
            parts
        }
    }
}

impl Transformable<Isogonal> for Motif {
    fn transform(&self, xform: Isogonal) -> Self {
        let parts = self.parts.iter().map(|(tile, id)| (tile.transform(xform), *id)).collect();

        Self {
            parts
        }
    }
}
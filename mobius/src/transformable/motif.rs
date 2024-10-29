use std::fmt::Display;

use crate::isogonal::Isogonal;

use super::{ClineArcTile, Transformable};

#[derive(Clone)]
pub struct Motif {
    parts: Vec<(ClineArcTile, usize)>,
}

impl Motif {
    pub fn new(parts: Vec<(ClineArcTile, usize)>) -> Self {
        Self { parts }
    }

    pub fn iter(&self) -> impl Iterator<Item = &(ClineArcTile, usize)> {
        self.parts.iter()
    }

    pub fn union(a: Self, b: Self) -> Self {
        let parts: Vec<(ClineArcTile, usize)> =
            a.parts.into_iter().chain(b.parts.into_iter()).collect();

        Self { parts }
    }
}

impl Transformable<Isogonal> for Motif {
    fn transform(&self, xform: Isogonal) -> Self {
        let parts = self
            .parts
            .iter()
            .map(|(tile, id)| (tile.transform(xform), *id))
            .collect();

        Self { parts }
    }
}

impl Display for Motif {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (tile, id) in self.iter() {
            write!(f, "{}\n{}\n", tile, id)?;
        }
        Ok(())
    }
}

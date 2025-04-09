use std::fmt::Display;

use crate::isogonal::Isogonal;

use super::Transformable;

#[derive(Clone)]
pub struct Motif<T> {
    parts: Vec<(T, usize)>,
}

impl<T> Motif<T> {
    pub fn new(parts: Vec<(T, usize)>) -> Self {
        Self { parts }
    }

    pub fn iter(&self) -> impl Iterator<Item = &(T, usize)> {
        self.parts.iter()
    }

    pub fn union(a: Self, b: Self) -> Self {
        let parts: Vec<(T, usize)> = a.parts.into_iter().chain(b.parts).collect();

        Self { parts }
    }
}

impl<T: Transformable<Isogonal>> Transformable<Isogonal> for Motif<T> {
    fn transform(&self, xform: Isogonal) -> Self {
        let parts = self
            .parts
            .iter()
            .map(|(tile, id)| (tile.transform(xform), *id))
            .collect();

        Self { parts }
    }
}

impl<T: Transformable<Isogonal> + Display> Display for Motif<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (tile, id) in self.iter() {
            write!(f, "{}\n{}\n", tile, id)?;
        }
        Ok(())
    }
}

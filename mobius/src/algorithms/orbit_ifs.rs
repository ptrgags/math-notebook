use std::collections::VecDeque;

use abstraction::GroupAction;

use crate::{isogonal::Isogonal, transformable::Transformable};

use super::{
    point_set::{PointSet, Set},
    IsogonalTile,
};

pub struct OrbitIFS {
    initial_tile: IsogonalTile,
}

impl OrbitIFS {
    pub fn new(initial_tile: IsogonalTile) -> Self {
        Self { initial_tile }
    }

    pub fn orbit(&self, max_depth: usize, quantize_bits: i32) -> OrbitIFSIterator {
        OrbitIFSIterator::new(self.initial_tile.clone(), max_depth, quantize_bits)
    }

    pub fn apply<T: Transformable<Isogonal>>(
        &self,
        primitive: &T,
        max_depth: usize,
        quantize_bits: i32,
    ) -> Vec<T> {
        self.orbit(max_depth, quantize_bits)
            .map(|xform| primitive.transform(xform))
            .collect()
    }
}

pub struct OrbitIFSIterator {
    max_depth: usize,
    // Queue contains pairs of (depth, tile)
    queue: VecDeque<(usize, IsogonalTile)>,
    visited: PointSet,
}

impl OrbitIFSIterator {
    fn new(initial_tile: IsogonalTile, max_depth: usize, quantize_bits: i32) -> Self {
        Self {
            max_depth,
            queue: VecDeque::from([(0, initial_tile)]),
            visited: PointSet::new(quantize_bits),
        }
    }

    fn pop_next_unvisited(&mut self) -> Option<(usize, IsogonalTile)> {
        while let Some((depth, tile)) = self.queue.pop_front() {
            if self.visited.contains(&tile.representative) {
                continue;
            }
            return Some((depth, tile));
        }

        return None;
    }
}

impl Iterator for OrbitIFSIterator {
    type Item = Isogonal;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((depth, tile)) = self.pop_next_unvisited() {
            self.visited.insert(tile.representative.clone());

            if depth < self.max_depth {
                let unvisited_neighbors: Vec<IsogonalTile> = tile
                    .get_neighbors()
                    .into_iter()
                    .filter(|neighbor| !self.visited.contains(&neighbor.representative))
                    .collect();

                for neighbor in unvisited_neighbors {
                    self.queue.push_back((depth + 1, neighbor));
                }
            }
            Some(tile.xform)
        } else {
            None
        }
    }
}

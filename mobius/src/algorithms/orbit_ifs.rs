use std::collections::VecDeque;

use abstraction::{Group, GroupAction};

use crate::{quantized_hash::QuantizedHash, transformable::Transformable};

use super::{orbit_tile::OrbitTile, point_set::Set};

pub struct OrbitIFS<G, P> {
    initial_tile: OrbitTile<G, P>,
}

impl<G, P> OrbitIFS<G, P>
where
    G: Group + GroupAction<P>,
    P: Clone + QuantizedHash,
{
    pub fn new(initial_tile: OrbitTile<G, P>) -> Self {
        Self { initial_tile }
    }

    pub fn orbit<S: Set<P>>(
        &self,
        max_depth: usize,
        quantize_bits: i32,
    ) -> OrbitIFSIterator<G, S, P> {
        OrbitIFSIterator::new(self.initial_tile.clone(), max_depth, quantize_bits)
    }

    pub fn apply<T: Transformable<G>, S: Set<P>>(
        &self,
        primitive: &T,
        max_depth: usize,
        quantize_bits: i32,
    ) -> Vec<T> {
        self.orbit::<S>(max_depth, quantize_bits)
            .map(|xform| primitive.transform(xform))
            .collect()
    }
}

pub struct OrbitIFSIterator<G, S, P>
where
    S: Set<P>,
{
    max_depth: usize,
    // Queue contains pairs of (depth, tile)
    queue: VecDeque<(usize, OrbitTile<G, P>)>,
    visited: S,
    quantize_bits: i32,
}

impl<G, S, P> OrbitIFSIterator<G, S, P>
where
    S: Set<P>,
    P: Clone,
{
    fn new(initial_tile: OrbitTile<G, P>, max_depth: usize, quantize_bits: i32) -> Self {
        Self {
            max_depth,
            queue: VecDeque::from([(0, initial_tile)]),
            visited: Default::default(),
            quantize_bits,
        }
    }

    fn pop_next_unvisited(&mut self) -> Option<(usize, OrbitTile<G, P>)> {
        while let Some((depth, tile)) = self.queue.pop_front() {
            if self.visited.contains(&tile.representative) {
                continue;
            }
            return Some((depth, tile));
        }

        return None;
    }
}

impl<G, P, S> Iterator for OrbitIFSIterator<G, S, P>
where
    G: Group + GroupAction<P>,
    S: Set<P>,
    P: Clone,
{
    type Item = G;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((depth, tile)) = self.pop_next_unvisited() {
            self.visited.insert(tile.representative.clone());

            if depth < self.max_depth {
                let unvisited_neighbors: Vec<OrbitTile<G, P>> = tile
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

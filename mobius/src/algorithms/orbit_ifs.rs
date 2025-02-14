use std::collections::{HashSet, VecDeque};

use abstraction::{Group, GroupAction};

use crate::{quantized_hash::QuantizedHash, transformable::Transformable};

use super::orbit_tile::OrbitTile;

const QUANTIZE_BITS: i32 = 16;

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

    pub fn orbit(&self, max_depth: usize) -> OrbitIFSIterator<G, P> {
        OrbitIFSIterator::new(self.initial_tile.clone(), max_depth)
    }

    pub fn apply<T: Transformable<G>>(&self, primitive: &T, max_depth: usize) -> Vec<T> {
        self.orbit(max_depth)
            .map(|xform| primitive.transform(xform))
            .collect()
    }
}

pub struct OrbitIFSIterator<G, P: QuantizedHash> {
    max_depth: usize,
    // Queue contains pairs of (depth, tile)
    queue: VecDeque<(usize, OrbitTile<G, P>)>,
    visited: HashSet<P::QuantizedType>,
}

impl<G, P> OrbitIFSIterator<G, P>
where
    P: QuantizedHash,
{
    fn new(initial_tile: OrbitTile<G, P>, max_depth: usize) -> Self {
        Self {
            max_depth,
            queue: VecDeque::from([(0, initial_tile)]),
            visited: HashSet::new(),
        }
    }
}

impl<G, P> Iterator for OrbitIFSIterator<G, P>
where
    G: Group + GroupAction<P>,
    P: Clone + QuantizedHash,
{
    type Item = G;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((depth, tile)) = self.queue.pop_front() {
            let hash = tile.quantize(QUANTIZE_BITS);
            self.visited.insert(hash);

            if depth < self.max_depth {
                let unvisited_neighbors: Vec<OrbitTile<G, P>> = tile
                    .get_neighbors()
                    .into_iter()
                    .filter(|neighbor| !self.visited.contains(&neighbor.quantize(QUANTIZE_BITS)))
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

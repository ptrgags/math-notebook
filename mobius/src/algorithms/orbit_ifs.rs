use std::collections::HashSet;

use abstraction::Group;

use crate::{quantized_hash::QuantizedHash, transformable::Transformable};

const QUANTIZE_BITS: i32 = 16;

// A point that can be transformed by the group operation, and also can be
// hashed for use in a hash set. This is used to detect equivalent
// transformations
pub trait OrbitPoint<G>: Transformable<G> + Clone + QuantizedHash {}

impl<G> OrbitPoint<G> for G where G: Transformable<G> + Clone + QuantizedHash {}

#[derive(Clone)]
pub struct OrbitTile<G: Group, P: OrbitPoint<G>> {
    xform: G,
    neighbor_xforms: Vec<G>,
    // Pick one point in the interior of the fundamental domain of the tile.
    // This is used to hash tiles so we don't repeat work.
    // It's important not to pick a point on the boundary, else it might exist
    // in two different tiles at once!
    representative: P,
}

impl<G: Group, P: OrbitPoint<G>> OrbitTile<G, P> {
    pub fn new(xform: G, neighbor_xforms: Vec<G>, representative: P) -> Self {
        Self {
            xform,
            neighbor_xforms,
            representative,
        }
    }

    fn get_neighbor(&self, to_neighbor: G) -> Self {
        // All points in the tile are transformed (including the representative)
        // transform directly
        let xform = to_neighbor.clone() * self.xform.clone();
        let representative = self.representative.transform(to_neighbor.clone());

        // To find the corresponding arrows in the neighbor tile, we have to
        // conjugate
        let neighbor_xforms: Vec<G> = self
            .neighbor_xforms
            .iter()
            .cloned()
            .map(|x| G::sandwich(to_neighbor.clone(), x))
            .collect();

        Self {
            xform,
            neighbor_xforms,
            representative,
        }
    }

    pub fn get_neighbors(&self) -> Vec<Self> {
        self.neighbor_xforms
            .iter()
            .cloned()
            .map(|xform| self.get_neighbor(xform))
            .collect()
    }
}

// The tile's hash is its representative
impl<G: Group, P: OrbitPoint<G>> QuantizedHash for OrbitTile<G, P> {
    type QuantizedType = P::QuantizedType;

    fn quantize(&self, quantize_bits: i32) -> Self::QuantizedType {
        self.representative.quantize(quantize_bits)
    }
}

pub struct OrbitIFS<G: Group, P: OrbitPoint<G>> {
    initial_tile: OrbitTile<G, P>,
}

impl<G: Group, P: OrbitPoint<G>> OrbitIFS<G, P> {
    pub fn new(initial_tile: OrbitTile<G, P>) -> Self {
        Self { initial_tile }
    }

    pub fn orbit(&self, max_depth: usize) -> OrbitIFSIterator<G, P> {
        OrbitIFSIterator::new(self.initial_tile.clone(), max_depth)
    }
}

pub struct OrbitIFSIterator<G: Group, P: OrbitPoint<G>> {
    max_depth: usize,
    // Stack contains pairs of (depth, tile)
    stack: Vec<(usize, OrbitTile<G, P>)>,
    visited: HashSet<P::QuantizedType>,
}

impl<G: Group, P: OrbitPoint<G>> OrbitIFSIterator<G, P> {
    fn new(initial_tile: OrbitTile<G, P>, max_depth: usize) -> Self {
        Self {
            max_depth,
            stack: vec![(0, initial_tile)],
            visited: HashSet::new(),
        }
    }

    /// It's often possible to reach the same tile from multiple paths, so
    /// there's a chance that a stack entry will have already been visited
    /// by the time it gets popped. So keep popping until we find something
    /// unvisited or exhaust the stack.
    pub fn pop_next_unvisited(&mut self) -> Option<(usize, OrbitTile<G, P>)> {
        while let Some((depth, tile)) = self.stack.pop() {
            if self.visited.contains(&tile.quantize(QUANTIZE_BITS)) {
                continue;
            }

            Some((depth, tile));
        }

        None
    }
}

impl<G: Group, P: OrbitPoint<G>> Iterator for OrbitIFSIterator<G, P> {
    type Item = G;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((depth, tile)) = self.pop_next_unvisited() {
            let hash = tile.quantize(QUANTIZE_BITS);
            self.visited.insert(hash);

            if depth < self.max_depth {
                let unvisited_neighbors: Vec<OrbitTile<G, P>> = tile
                    .get_neighbors()
                    .into_iter()
                    .filter(|neighbor| !self.visited.contains(&neighbor.quantize(QUANTIZE_BITS)))
                    .collect();

                for neighbor in unvisited_neighbors {
                    self.stack.push((depth + 1, neighbor));
                }
            }

            Some(tile.xform)
        } else {
            None
        }
    }
}

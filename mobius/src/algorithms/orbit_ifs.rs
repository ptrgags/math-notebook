use std::collections::HashSet;

use abstraction::{Group, GroupAction};

use crate::{quantized_hash::QuantizedHash, transformable::Transformable};

const QUANTIZE_BITS: i32 = 16;

#[derive(Clone)]
pub struct OrbitTile<G, P> {
    xform: G,
    neighbor_xforms: Vec<G>,
    // Pick one point in the interior of the fundamental domain of the tile.
    // This is used to hash tiles so we don't repeat work.
    // It's important not to pick a point on the boundary, else it might exist
    // in two different tiles at once!
    representative: P,
}

impl<G, P> OrbitTile<G, P>
where
    G: Group + GroupAction<P>,
    P: Clone,
{
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
        let representative = to_neighbor.clone() * self.representative.clone();

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
impl<G, P> QuantizedHash for OrbitTile<G, P>
where
    P: QuantizedHash,
{
    type QuantizedType = P::QuantizedType;

    fn quantize(&self, quantize_bits: i32) -> Self::QuantizedType {
        self.representative.quantize(quantize_bits)
    }
}

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
    // Stack contains pairs of (depth, tile)
    stack: Vec<(usize, OrbitTile<G, P>)>,
    visited: HashSet<P::QuantizedType>,
}

impl<G, P> OrbitIFSIterator<G, P>
where
    P: QuantizedHash,
{
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

            return Some((depth, tile));
        }

        None
    }
}

impl<G, P> Iterator for OrbitIFSIterator<G, P>
where
    G: Group + GroupAction<P>,
    P: Clone + QuantizedHash,
{
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

use std::{collections::HashSet, hash::Hash};

use abstraction::Group;

use crate::transformable::Transformable;

// A point that can be transformed by the group operation, and also can be
// hashed for use in a hash set. This is used to detect equivalent
// transformations
pub trait OrbitPoint<G>: Transformable<G> + Clone + Eq + Hash {}

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

    pub fn get_neighbor(&self, index: usize) -> Option<Self> {
        if index >= self.neighbor_xforms.len() {
            return None;
        }

        // Get the transform to step into the neighbor tile
        let to_neighbor = self.neighbor_xforms[index].clone();

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

        Some(Self {
            xform,
            neighbor_xforms,
            representative,
        })
    }
}

pub struct OrbitIFS<G: Group, P: OrbitPoint<G>> {
    initial_tile: OrbitTile<G, P>,
}

impl<G: Group, P: OrbitPoint<G>> OrbitIFS<G, P> {
    pub fn new(initial_tile: OrbitTile<G, P>) -> Self {
        Self { initial_tile }
    }
}

pub struct OrbitIFSIterator<'a, G: Group, P: OrbitPoint<G>> {
    ifs: &'a OrbitIFS<G, P>,
    max_depth: usize,
    stack: Vec<OrbitTile<G, P>>,
    visited: HashSet<P>,
}

impl<'a, G: Group, P: OrbitPoint<G>> OrbitIFSIterator<'a, G, P> {
    fn new(ifs: &'a OrbitIFS<G, P>, max_depth: usize) -> Self {
        let initial_tile = ifs.initial_tile.clone();
        Self {
            ifs,
            max_depth,
            stack: vec![initial_tile],
            visited: HashSet::new(),
        }
    }

    /// It's often possible to reach the same tile from multiple paths, so
    /// there's a chance that a stack entry will have already been visited
    /// by the time it gets popped. So keep popping until we find something
    /// unvisited or exhaust the stack.
    pub fn pop_next_unvisited(&mut self) -> Option<OrbitTile<G, P>> {
        while let Some(tile) = self.stack.pop() {
            if self.visited.contains(&tile.representative) {
                continue;
            }

            Some(tile);
        }

        None
    }
}

impl<'a, G: Group, P: OrbitPoint<G>> Iterator for OrbitIFSIterator<'a, G, P> {
    type Item = G;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(tile) = self.pop_next_unvisited() {}
        None
    }
}

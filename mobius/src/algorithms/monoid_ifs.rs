use std::ops::Index;

use abstraction::{monoid::Monoid, semigroup::Semigroup};

use crate::transformable::Transformable;

/// Iterated Function System that can be applied to transformable objects
/// via a depth-limited DFS (i.e. the "deterministic algorithm" in
/// _Fractals Everywhere_ by Michael F. Barnsley)
pub struct MonoidIFS<M: Monoid> {
    xforms: Vec<M>,
}

impl<M: Monoid> MonoidIFS<M> {
    pub fn new(xforms: Vec<M>) -> Self {
        Self { xforms }
    }

    pub fn iter(&self) -> impl Iterator<Item = &M> {
        self.xforms.iter()
    }

    pub fn dfs(&self, max_depth: usize) -> MonoidDFSIterator<M> {
        MonoidDFSIterator::new(self, max_depth)
    }

    pub fn apply<T: Transformable<M>>(
        &self,
        primitive: &T,
        min_depth: usize,
        max_depth: usize,
    ) -> Vec<T> {
        self.dfs(max_depth)
            .filter_map(|(depth, xform)| {
                if depth >= min_depth {
                    Some(primitive.transform(xform))
                } else {
                    None
                }
            })
            .collect()
    }

    /// When T values can be combined, this method is convenient for flattening
    /// the results of apply() into a single T
    pub fn flat_apply<T>(&self, primitive: &T, min_depth: usize, max_depth: usize) -> T
    where
        T: Transformable<M> + Semigroup,
    {
        let transformed = self.apply(primitive, min_depth, max_depth);
        Semigroup::sconcat(&transformed)
    }
}

impl<M: Monoid> Index<usize> for MonoidIFS<M> {
    type Output = M;

    fn index(&self, index: usize) -> &Self::Output {
        &self.xforms[index]
    }
}

pub struct MonoidDFSIterator<'a, M: Monoid> {
    ifs: &'a MonoidIFS<M>,
    max_depth: usize,
    // pairs of (depth, xform)
    stack: Vec<(usize, M)>,
}

impl<'a, M: Monoid> MonoidDFSIterator<'a, M> {
    fn new(ifs: &'a MonoidIFS<M>, max_depth: usize) -> Self {
        Self {
            ifs,
            max_depth,
            stack: vec![(0, M::identity())],
        }
    }
}

impl<'a, M: Monoid> Iterator for MonoidDFSIterator<'a, M> {
    type Item = (usize, M);

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            None => None,
            Some((depth, xform)) => {
                if depth < self.max_depth {
                    for next_xform in self.ifs.iter().cloned() {
                        self.stack.push((depth + 1, next_xform * xform.clone()));
                    }
                }
                Some((depth, xform))
            }
        }
    }
}

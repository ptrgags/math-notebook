use std::ops::Index;

use abstraction::{semigroup::Semigroup, Monoid};

use crate::transformable::Transformable;

/// Iterated Function System that can be applied to transformable objects
/// via a depth-limited DFS (i.e. the "deterministic algorithm" in
/// _Fractals Everywhere_ by Michael F. Barnsley)
pub struct MonoidIFS<S: Monoid> {
    xforms: Vec<S>,
}

impl<S: Monoid> MonoidIFS<S> {
    pub fn new(xforms: Vec<S>) -> Self {
        Self { xforms }
    }

    pub fn iter(&self) -> impl Iterator<Item = &S> {
        self.xforms.iter()
    }

    pub fn dfs(&self, max_depth: usize) -> MonoidDFSIterator<S> {
        MonoidDFSIterator::new(self, max_depth)
    }

    pub fn apply<T: Transformable<S>>(
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
        T: Transformable<S> + Semigroup,
    {
        let transformed = self.apply(primitive, min_depth, max_depth);
        Semigroup::sconcat(&transformed)
    }
}

impl<S: Monoid> Index<usize> for MonoidIFS<S> {
    type Output = S;

    fn index(&self, index: usize) -> &Self::Output {
        &self.xforms[index]
    }
}

pub struct MonoidDFSIterator<'a, S: Monoid> {
    ifs: &'a MonoidIFS<S>,
    max_depth: usize,
    // pairs of (depth, xform)
    stack: Vec<(usize, S)>,
}

impl<'a, S: Monoid> MonoidDFSIterator<'a, S> {
    fn new(ifs: &'a MonoidIFS<S>, max_depth: usize) -> Self {
        Self {
            ifs,
            max_depth,
            stack: vec![(0, S::identity())],
        }
    }
}

impl<'a, S: Monoid> Iterator for MonoidDFSIterator<'a, S> {
    type Item = (usize, S);

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

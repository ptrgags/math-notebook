use std::ops::Index;

use abstraction::{Group, Semigroup};

use crate::{transformable::Transformable, Mobius};

/// Iterated Function System. This is still in a prototype stage
pub struct SemigroupIFS<S: Semigroup> {
    xforms: Vec<S>,
}

impl<S: Semigroup> SemigroupIFS<S> {
    pub fn new(xforms: Vec<S>) -> Self {
        Self { xforms }
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &S> {
        self.xforms.iter()
    }

    pub fn dfs(&self, max_depth: usize) -> SemigroupDFSIterator<S> {
        SemigroupDFSIterator::new(self, max_depth)
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
}

impl<S: Semigroup> Index<usize> for SemigroupIFS<S> {
    type Output = S;

    fn index(&self, index: usize) -> &Self::Output {
        &self.xforms[index]
    }
}

pub struct SemigroupDFSIterator<'a, S: Semigroup> {
    ifs: &'a SemigroupIFS<S>,
    max_depth: usize,
    // pairs of (depth, xform)
    stack: Vec<(usize, S)>,
}

impl<'a, S: Semigroup> SemigroupDFSIterator<'a, S> {
    fn new(ifs: &'a SemigroupIFS<S>, max_depth: usize) -> Self {
        Self {
            ifs,
            max_depth,
            stack: vec![(0, S::identity())],
        }
    }
}

impl<'a, S: Semigroup> Iterator for SemigroupDFSIterator<'a, S> {
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

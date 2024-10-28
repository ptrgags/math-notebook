use abstraction::{Group, Semigroup};

use crate::{
    address::{FractalAddress, Symbol},
    transformable::Transformable,
};

/// Iterated function system for a group. The depth-first-search iterator
/// for this IFS avoids backtracking.
pub struct GroupIFS<G: Group> {
    /// List of transformations from the symmetry group.
    /// This is twice the length of the input to store the inverses
    /// explicitly. E.g. if the input was [a, b, c], the IFS will store
    /// [a, b, c, A, B, C]
    xforms: Vec<G>,
}

impl<G: Group> GroupIFS<G> {
    pub fn new(xforms_no_inverses: Vec<G>) -> Self {
        let inverses: Vec<G> = xforms_no_inverses.iter().map(|x| x.inverse()).collect();
        let xforms: Vec<G> = xforms_no_inverses
            .into_iter()
            .chain(inverses.into_iter())
            .collect();

        Self { xforms }
    }

    pub fn get_index(&self, symbol: Symbol) -> usize {
        match symbol {
            Symbol::Forward(i) => i,
            Symbol::Inverse(i) => i + self.xforms.len() / 2,
        }
    }

    pub fn get_symbol(&self, index: usize) -> Symbol {
        let n = self.xforms.len() / 2;
        if index < n {
            Symbol::Forward(index)
        } else {
            Symbol::Inverse(index - n)
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &G> {
        self.xforms.iter()
    }

    /// Iterate over all the
    pub fn iter_no_inverse(&self, last_xform_index: usize) -> impl Iterator<Item = &G> {
        let n = self.xforms.len() / 2;
        let start_index = last_xform_index + n + 1;
        let end_index = last_xform_index + 3 * n;

        (start_index..end_index).map(|i| &self.xforms[i % self.xforms.len()])
    }

    pub fn dfs(&self, max_depth: usize) -> GroupDFSIterator<G> {
        GroupDFSIterator::new(self, max_depth)
    }

    pub fn apply<T: Transformable<G>>(
        &self,
        primitive: &T,
        min_depth: usize,
        max_depth: usize,
    ) -> Vec<T> {
        self.dfs(max_depth)
            .inspect(|(address, _)| println!("{}", address))
            .filter_map(|(address, xform)| {
                if address.len() >= min_depth {
                    Some(primitive.transform(xform))
                } else {
                    None
                }
            })
            .collect()
    }
}

pub struct GroupDFSIterator<'a, G: Group> {
    ifs: &'a GroupIFS<G>,
    max_depth: usize,
    // pairs of (address, group)
    stack: Vec<(FractalAddress, G)>,
}

impl<'a, G: Group> GroupDFSIterator<'a, G> {
    fn new(ifs: &'a GroupIFS<G>, max_depth: usize) -> Self {
        Self {
            ifs,
            max_depth,
            stack: vec![(FractalAddress::identity(), G::identity())],
        }
    }
}

impl<'a, G: Group> Iterator for GroupDFSIterator<'a, G> {
    type Item = (FractalAddress, G);

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            None => None,
            Some((address, val)) => {
                if address == FractalAddress::identity() {
                    // For the first step, we can choose any transformation

                    for (i, next_val) in self.ifs.iter().enumerate() {
                        let child_address = address.clone() * self.ifs.get_symbol(i).into();
                        let child_val = val.clone() * next_val.clone();
                        self.stack.push((child_address, child_val))
                    }
                } else if address.len() < self.max_depth {
                    // Iterate over all of the transformations in the IFS
                    // except for the inverse of the last one we just applied
                    let last_xform_index = self.ifs.get_index(address.rightmost());
                    let n = self.ifs.xforms.len() / 2;

                    // The inverse of the last xform's inverse is index + n
                    let start = last_xform_index + (n + 1);
                    let end = start + (3 * n + 1);
                    for i in start..end {
                        let index = i % self.ifs.xforms.len();
                        let next_val = self.ifs.xforms[index].clone();
                        let child_address = address.clone() * self.ifs.get_symbol(index).into();
                        let child_val = val.clone() * next_val;
                        self.stack.push((child_address, child_val))
                    }
                }
                Some((address, val))
            }
        }
    }
}

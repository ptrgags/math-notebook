use std::ops::Index;

use abstraction::{semigroup::Semigroup, Group, Monoid};

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
        let xforms: Vec<G> = xforms_no_inverses.into_iter().chain(inverses).collect();

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
            .filter_map(|(address, xform)| {
                if address.len() >= min_depth {
                    Some(primitive.transform(xform))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn flat_apply<T>(&self, primitive: &T, min_depth: usize, max_depth: usize) -> T
    where
        T: Transformable<G> + Semigroup,
    {
        let applied = self.apply(primitive, min_depth, max_depth);
        Semigroup::sconcat(&applied)
    }
}

impl<G: Group> Index<usize> for GroupIFS<G> {
    type Output = G;

    fn index(&self, index: usize) -> &Self::Output {
        &self.xforms[index]
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
                let xform_count = self.ifs.xforms.len();
                let generator_count = xform_count / 2;

                if address.len() < self.max_depth && address == FractalAddress::identity() {
                    // For the first step, we can choose any of the xforms
                    // Push them to the stack in reverse order;
                    for i in (0..xform_count).rev() {
                        let child_address = FractalAddress::from(self.ifs.get_symbol(i));
                        let child_val = self.ifs[i].clone();
                        self.stack.push((child_address, child_val));
                    }
                } else if address.len() < self.max_depth {
                    // Iterate over all of the transformations in the IFS
                    // except for the inverse of the last one we just applied.

                    // The transformations are stored abc...zABC...Z
                    // the last transformation is at the index matching
                    // the rightmost symbol of the address.
                    // its inverse is at that index + generator_count. Start
                    // one element past that and continue until we've seen
                    // all the transforms except the inverse
                    let last_xform_index = self.ifs.get_index(address.rightmost());
                    let start = last_xform_index + (generator_count + 1);
                    let end = start + xform_count - 1;

                    // Again, push onto the stack in reverse order
                    for i in (start..end).rev() {
                        let index = i % xform_count;
                        let next_val = self.ifs[index].clone();
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

#[cfg(test)]
mod test {
    use crate::{scale, translation, Complex, Mobius};

    use pretty_assertions::assert_eq;
    use test_case::test_case;

    use super::*;

    fn make_ifs(dimensions: usize) -> GroupIFS<Mobius> {
        let a = translation(Complex::ONE).unwrap();
        let b = translation(Complex::I).unwrap();
        let c = scale(2.0).unwrap();
        match dimensions {
            1 => GroupIFS::new(vec![a]),
            2 => GroupIFS::new(vec![a, b]),
            3 => GroupIFS::new(vec![a, b, c]),
            _ => panic!("only testing with 1-3 dimensions"),
        }
    }

    #[test_case(0, Symbol::Forward(0))]
    #[test_case(1, Symbol::Forward(1))]
    pub fn get_symbol_with_small_index_returns_forward(index: usize, expected: Symbol) {
        let ifs = make_ifs(2);

        let symbol = ifs.get_symbol(index);

        assert_eq!(symbol, expected);
    }

    #[test_case(2, Symbol::Inverse(0))]
    #[test_case(3, Symbol::Inverse(1))]
    pub fn get_symbol_with_large_index_returns_inverse(index: usize, expected: Symbol) {
        let ifs = make_ifs(2);

        let symbol = ifs.get_symbol(index);

        assert_eq!(symbol, expected);
    }

    #[test_case(Symbol::Forward(0), 0)]
    #[test_case(Symbol::Forward(1), 1)]
    pub fn get_index_with_forward_returns_small_index(symbol: Symbol, expected: usize) {
        let ifs = make_ifs(2);

        let index = ifs.get_index(symbol);

        assert_eq!(index, expected);
    }

    #[test_case(Symbol::Inverse(0), 2)]
    #[test_case(Symbol::Inverse(1), 3)]
    pub fn get_index_with_inverse_returns_large_index(symbol: Symbol, expected: usize) {
        let ifs = make_ifs(2);

        let index = ifs.get_index(symbol);

        assert_eq!(index, expected);
    }

    fn make_expected_traversal(values: Vec<(&str, Mobius)>) -> Vec<(FractalAddress, Mobius)> {
        values
            .iter()
            .map(|(addr, x)| (FractalAddress::try_from(*addr).unwrap(), *x))
            .collect()
    }

    #[test]
    pub fn dfs_depth0_returns_identity() {
        let a = translation(Complex::new(1.0, 1.0)).unwrap();
        let ifs = GroupIFS::new(vec![a]);

        let results: Vec<(FractalAddress, Mobius)> = ifs.dfs(0).collect();

        assert_eq!(
            &results,
            &[(FractalAddress::identity(), Mobius::identity())]
        );
    }

    #[test]
    pub fn dfs_depth1_returns_identity_and_generators() {
        let a = translation(Complex::ONE).unwrap();
        let b = translation(Complex::I).unwrap();
        let c = scale(2.0).unwrap();
        let ifs = GroupIFS::new(vec![a, b, c]);

        let results: Vec<(FractalAddress, Mobius)> = ifs.dfs(1).collect();

        let expected = make_expected_traversal(vec![
            ("", Mobius::identity()),
            ("a", a),
            ("b", b),
            ("c", c),
            ("A", a.inverse()),
            ("B", b.inverse()),
            ("C", c.inverse()),
        ]);
        assert_eq!(&results, &expected)
    }

    #[test]
    pub fn dfs_depth2_iterates_in_cyclic_order() {
        let a = translation(Complex::ONE).unwrap();
        let b = translation(Complex::I).unwrap();
        let c = scale(2.0).unwrap();
        let ifs = GroupIFS::new(vec![a, b, c]);

        let results: Vec<(FractalAddress, Mobius)> = ifs.dfs(2).collect();

        let expected = make_expected_traversal(vec![
            ("", Mobius::identity()),
            ("a", a),
            // Skip aA
            ("aB", a * b.inverse()),
            ("aC", a * c.inverse()),
            ("aa", a * a),
            ("ab", a * b),
            ("ac", a * c),
            ("b", b),
            // skip bB
            ("bC", b * c.inverse()),
            ("ba", b * a),
            ("bb", b * b),
            ("bc", b * c),
            ("bA", b * a.inverse()),
            ("c", c),
            // skip cC,
            ("ca", c * a),
            ("cb", c * b),
            ("cc", c * c),
            ("cA", c * a.inverse()),
            ("cB", c * b.inverse()),
            ("A", a.inverse()),
            // Skip Aa
            ("Ab", a.inverse() * b),
            ("Ac", a.inverse() * c),
            ("AA", a.inverse() * a.inverse()),
            ("AB", a.inverse() * b.inverse()),
            ("AC", a.inverse() * c.inverse()),
            ("B", b.inverse()),
            // Skip Bb
            ("Bc", b.inverse() * c),
            ("BA", b.inverse() * a.inverse()),
            ("BB", b.inverse() * b.inverse()),
            ("BC", b.inverse() * c.inverse()),
            ("Ba", b.inverse() * a),
            ("C", c.inverse()),
            // Skip Cc
            ("CA", c.inverse() * a.inverse()),
            ("CB", c.inverse() * b.inverse()),
            ("CC", c.inverse() * c.inverse()),
            ("Ca", c.inverse() * a),
            ("Cb", c.inverse() * b),
        ]);
        assert_eq!(&results, &expected)
    }
}

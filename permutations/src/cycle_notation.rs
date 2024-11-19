use std::collections::HashSet;

use crate::permutation_error::PermutationError;

pub struct DisjointCycles<const N: usize>(pub Vec<Vec<usize>>);

impl<const N: usize> DisjointCycles<N> {
    pub fn new(cycles: Vec<Vec<usize>>) -> Result<Self, PermutationError> {
        let mut unique_values: HashSet<usize> = HashSet::new();
        let mut value_count = 0;

        let each_element = cycles.iter().flat_map(|cycle| cycle.iter());
        for &x in each_element {
            if x >= N {
                return Err(PermutationError::ValueOutOfRange(x, N));
            }
            value_count += 1;
            unique_values.insert(x);
        }

        if unique_values.len() < value_count {
            return Err(PermutationError::OverlappingCycles);
        }

        Ok(Self(cycles))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn new_with_out_of_range_element_returns_error() {
        let result = DisjointCycles::<4>::new(vec![vec![1, 2, 10]]);

        assert!(matches!(
            result,
            Err(PermutationError::ValueOutOfRange(_, _))
        ))
    }

    #[test]
    pub fn new_with_overlapping_cycles_returns_error() {
        let result = DisjointCycles::<4>::new(vec![vec![1, 2, 3], vec![0, 2]]);

        assert!(matches!(result, Err(PermutationError::OverlappingCycles)))
    }

    #[test]
    pub fn new_with_disjoint_cycles_returns_struct() {
        let cycles: Vec<Vec<usize>> = vec![vec![0, 1], vec![2, 3]];

        let DisjointCycles(result) = DisjointCycles::<4>::new(cycles.clone()).unwrap();

        assert_eq!(&cycles[..], &result[..])
    }
}

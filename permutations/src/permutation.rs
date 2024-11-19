use std::{collections::HashSet, fmt::Display, ops::Mul, str::FromStr};

use abstraction::{Group, Monoid};

use crate::{disjoint_cycles::DisjointCycles, permutation_error::PermutationError};

/// Mathematical permutation of N elements. An element of the symmetric group S_N
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Permutation<const N: usize> {
    values: [usize; N],
}

impl<const N: usize> Permutation<N> {
    pub fn new(values: [usize; N]) -> Result<Self, PermutationError> {
        let unique_values: HashSet<usize> = HashSet::from_iter(values.iter().cloned());
        if unique_values.len() < values.len() {
            return Err(PermutationError::RepeatElement);
        }

        for value in values {
            if value >= N {
                return Err(PermutationError::ValueOutOfRange(value, N));
            }
        }

        Ok(Self { values })
    }

    pub fn from_disjoint_cycles(
        disjoint_cycles: DisjointCycles<N>,
    ) -> Result<Self, PermutationError> {
        let mut combined = [0; N];
        for i in 0..N {
            combined[i] = i;
        }

        let DisjointCycles(cycles) = disjoint_cycles;
        for cycle in cycles.iter().rev() {
            for (i, &element) in cycle.iter().enumerate() {
                combined[element] = cycle[(i + 1) % cycle.len()];
            }
        }

        Self::new(combined)
    }

    /// Compute the cycle decomposition for the permutation.
    pub fn cycle_decomposition(&self) -> DisjointCycles<N> {
        let mut visited = [false; N];
        let mut result = Vec::new();

        for start_element in 0..N {
            if visited[start_element] {
                continue;
            }
            visited[start_element] = true;
            let mut cycle = vec![start_element];

            let mut current = self.values[start_element];
            while current != start_element {
                visited[current] = true;
                cycle.push(current);
                current = self.values[current];
            }

            if cycle.len() > 1 {
                result.push(cycle);
            }
        }

        DisjointCycles::new(result).expect("unable to create DisjointCycles from Permutation")
    }
}

impl<const N: usize> FromStr for Permutation<N> {
    type Err = PermutationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let disjoint_cycles = s.parse::<DisjointCycles<N>>()?;

        Self::from_disjoint_cycles(disjoint_cycles)
    }
}

impl<const N: usize> Display for Permutation<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.cycle_decomposition().fmt(f)
    }
}

impl<const N: usize> Mul for Permutation<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut product = [0; N];

        for (i, rhs_value) in rhs.values.iter().enumerate() {
            product[i] = self.values[*rhs_value];
        }

        Self { values: product }
    }
}

impl<const N: usize> Monoid for Permutation<N> {
    fn identity() -> Self {
        let mut values = [0; N];
        for (i, value) in values.iter_mut().enumerate() {
            *value = i;
        }

        Self { values }
    }
}

impl<const N: usize> Group for Permutation<N> {
    fn inverse(&self) -> Self {
        let mut result = [0usize; N];

        for (i, x) in self.values.iter().enumerate() {
            result[*x] = i;
        }

        Self { values: result }
    }
}

#[cfg(test)]
mod test {
    use abstraction::{test_associativity, test_group, test_identity, test_inverse};

    use super::*;

    #[test]
    pub fn new_with_out_of_range_element_returns_error() {
        let result = Permutation::new([0, 1, 2, 10]);

        assert!(matches!(
            result,
            Err(PermutationError::ValueOutOfRange(_, _))
        ));
    }

    #[test]
    pub fn new_with_duplicate_element_returns_error() {
        let result = Permutation::new([0, 2, 2, 1]);

        assert!(matches!(result, Err(PermutationError::RepeatElement)));
    }

    #[test]
    pub fn new_returns_permutation_for_valid_elements() {
        let result = Permutation::new([0, 2, 3, 1]);

        assert!(result.is_ok());
    }

    #[test]
    pub fn from_disjoint_cycles_computes_correct_permutation() {
        // (0 2)(3 4)(1 5)
        // is the same as
        // [0 1 2 3 4 5]
        // [2 5 0 4 3 1]
        let cycles = DisjointCycles::<6>::new(vec![vec![0, 2], vec![3, 4], vec![1, 5]]).unwrap();
        let result = Permutation::from_disjoint_cycles(cycles).unwrap();
        let expected = Permutation::new([2, 5, 0, 4, 3, 1]).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    pub fn multiplication_applies_a_after_b() {
        let a = Permutation::new([0, 2, 3, 1]).unwrap();
        let b = Permutation::new([0, 1, 3, 2]).unwrap();

        let result = a * b;

        // a = (1 2 3)
        // b = (2 3)
        // ab = (1 2 3)(2 3) = (1 2) <-- we want this one
        // ba = (2 3)(1 2 3) = (1 3)
        let expected = Permutation::new([0, 2, 1, 3]).unwrap();
        assert_eq!(result, expected);
    }

    test_identity!(
        Permutation<4>,
        [
            (
                swap,
                Permutation {
                    values: [1, 0, 2, 3]
                }
            ),
            (
                three_cycle,
                Permutation {
                    values: [1, 2, 0, 3]
                }
            ),
            (
                four_cycle,
                Permutation {
                    values: [1, 2, 3, 0]
                }
            ),
            (
                double_swap,
                Permutation {
                    values: [1, 0, 3, 2]
                }
            )
        ]
    );

    test_associativity!(
        Permutation<6>,
        [
            (
                disjoint_swaps,
                Permutation {
                    values: [1, 0, 2, 3, 4, 5]
                },
                Permutation {
                    values: [0, 1, 3, 2, 4, 5]
                },
                Permutation {
                    values: [0, 1, 2, 3, 5, 4]
                }
            ),
            (
                overlapping_swaps,
                Permutation {
                    values: [1, 0, 2, 3, 4, 5]
                },
                Permutation {
                    values: [0, 2, 1, 3, 4, 5]
                },
                Permutation {
                    values: [0, 1, 3, 2, 4, 5]
                }
            )
        ]
    );

    test_inverse!(
        Permutation<3>,
        [(three_cycle, Permutation { values: [1, 2, 0] })]
    );

    test_group!(
        Permutation<4>,
        [(
            octahedral_xy,
            Permutation {
                values: [3, 2, 0, 1]
            },
            Permutation {
                values: [2, 0, 3, 1]
            }
        )]
    );
}

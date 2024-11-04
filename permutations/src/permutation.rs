use std::{collections::HashSet, ops::Mul};

use abstraction::{Group, Semigroup};

/// Mathematical permutation of N elements. An element of the symmetric group S_N
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Permutation<const N: usize> {
    values: [usize; N],
}

impl<const N: usize> Permutation<N> {
    pub fn new(values: [usize; N]) -> Result<Self, String> {
        let unique_values: HashSet<usize> = HashSet::from_iter(values.iter().cloned());
        if unique_values.len() < values.len() {
            return Err(String::from("values must not have repeat elements"));
        }

        for value in values {
            if value > values.len() {
                return Err(String::from("values must be less than length of slice"));
            }
        }

        Ok(Self { values })
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

impl<const N: usize> Semigroup for Permutation<N> {
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
    pub fn new_returns_error_for_out_of_range_element() {
        let result = Permutation::new([0, 1, 2, 10]);

        assert!(result.is_err_and(|e| e.contains("values must be less than length of slice")));
    }

    #[test]
    pub fn new_returns_error_for_duplicate_element() {
        let result = Permutation::new([0, 2, 2, 1]);

        assert!(result.is_err_and(|e| e.contains("values must not have repeat elements")));
    }

    #[test]
    pub fn new_returns_permutation_for_valid_elements() {
        let result = Permutation::new([0, 2, 3, 1]);

        assert!(result.is_ok());
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

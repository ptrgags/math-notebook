use std::{collections::HashSet, num::ParseIntError, str::FromStr};

use crate::permutation_error::PermutationError;

fn parse_parentheses(cycle_str: &str) -> Result<Vec<&str>, PermutationError> {
    let mut slice_indices = Vec::new();
    let mut start_index = 0;
    let mut inside = false;
    for (i, c) in cycle_str.chars().enumerate() {
        match c {
            '(' if inside => {
                return Err(PermutationError::NestedCycle(cycle_str.into()));
            }
            ')' if !inside => {
                return Err(PermutationError::UnmatchedParenthesis(cycle_str.into()));
            }
            '(' => {
                start_index = i + 1;
                inside = true;
            }
            ')' => {
                slice_indices.push((start_index, i));
                inside = false;
            }
            x if !x.is_whitespace() && !inside => {
                return Err(PermutationError::ValueOutsideParentheses(cycle_str.into()))
            }
            _ => {}
        }
    }

    // Unfinished cycle
    if inside {
        return Err(PermutationError::UnmatchedParenthesis(cycle_str.into()));
    }

    let slices: Vec<&str> = slice_indices
        .into_iter()
        .map(|(a, b)| &cycle_str[a..b])
        .collect();
    Ok(slices)
}

fn parse_cycle(cycle_str: &str) -> Result<Vec<usize>, PermutationError> {
    let result: Result<Vec<usize>, ParseIntError> = cycle_str
        .split_whitespace()
        .map(|x| x.parse::<usize>())
        .collect();

    Ok(result?)
}

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

impl<const N: usize> FromStr for DisjointCycles<N> {
    type Err = PermutationError;

    /// Grammar:
    /// disjoint_cycle = cycle*
    /// cycle = '(' USIZE* ')'
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cycles: Result<Vec<Vec<usize>>, PermutationError> = parse_parentheses(s)?
            .into_iter()
            .map(|s| parse_cycle(s))
            .collect();

        Self::new(cycles?)
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

    #[test]
    pub fn parse_with_empty_string_returns_identity() {
        let DisjointCycles(result) = "".parse::<DisjointCycles<4>>().unwrap();

        let expected: Vec<Vec<usize>> = vec![];
        assert_eq!(&result[..], &expected[..]);
    }

    #[test]
    pub fn parse_with_missing_left_parenthesis_returns_error() {
        let result = "1 2)".parse::<DisjointCycles<4>>();

        // You could interpret this case as "values outside the parentheses"
        // or an unmatched parenthesis.
        assert!(
            matches!(result, Err(PermutationError::UnmatchedParenthesis(_)))
                || matches!(result, Err(PermutationError::ValueOutsideParentheses(_)))
        );
    }

    #[test]
    pub fn parse_with_missing_right_parenthesis_returns_error() {
        let result = "(1 2".parse::<DisjointCycles<4>>();

        assert!(matches!(
            result,
            Err(PermutationError::UnmatchedParenthesis(_))
        ));
    }

    #[test]
    pub fn parse_with_top_level_number_returns_error() {
        let result = "1(2 3)".parse::<DisjointCycles<4>>();

        assert!(matches!(
            result,
            Err(PermutationError::ValueOutsideParentheses(_))
        ))
    }

    #[test]
    pub fn parse_with_whitespace_between_cycles_is_handled_gracefully() {
        let DisjointCycles(result) = "(0 3) (1 2)".parse::<DisjointCycles<4>>().unwrap();

        let expected: Vec<Vec<usize>> = vec![vec![0, 3], vec![1, 2]];
        assert_eq!(&result[..], &expected[..]);
    }

    #[test]
    pub fn parse_with_whitespace_before_first_element_is_handled_gracefully() {
        let DisjointCycles(result) = "( 1 2)".parse::<DisjointCycles<4>>().unwrap();

        let expected: Vec<Vec<usize>> = vec![vec![1, 2]];
        assert_eq!(&result[..], &expected[..]);
    }

    #[test]
    pub fn parse_with_whitespace_after_last_element_is_handled_gracefully() {
        let DisjointCycles(result) = "(1 2 )".parse::<DisjointCycles<4>>().unwrap();

        let expected: Vec<Vec<usize>> = vec![vec![1, 2]];
        assert_eq!(&result[..], &expected[..]);
    }

    #[test]
    pub fn parse_with_nested_parentheses_returns_error() {
        let result = "(1 (2 3) 4)".parse::<DisjointCycles<5>>();

        assert!(matches!(result, Err(PermutationError::NestedCycle(_))))
    }

    #[test]
    pub fn parse_with_bad_digit_returns_error() {
        let result = "(1 2 a 3)".parse::<DisjointCycles<4>>();

        assert!(matches!(result, Err(PermutationError::BadInt(_))))
    }

    #[test]
    pub fn parse_with_single_cycle_parses_correctly() {
        let DisjointCycles(cycles) = "(0 2 3)".parse::<DisjointCycles<4>>().unwrap();

        let expected: Vec<Vec<usize>> = vec![vec![0, 2, 3]];
        assert_eq!(&cycles[..], &expected[..])
    }

    #[test]
    pub fn parse_with_multiple_cycles_parses_correctly() {
        let DisjointCycles(cycles) = "(0 3 4)(1 2)".parse::<DisjointCycles<5>>().unwrap();

        let expected: Vec<Vec<usize>> = vec![vec![0, 3, 4], vec![1, 2]];
        assert_eq!(&cycles[..], &expected[..])
    }
}

use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum PermutationError {
    #[error("could not parse usize: {0}")]
    BadInt(#[from] ParseIntError),
    #[error("permutations must not repeat elements")]
    RepeatElement,
    #[error("cycles must be disjoint")]
    OverlappingCycles,
    #[error("permutation value must be in [0, {1}), got {0}")]
    ValueOutOfRange(usize, usize),
    #[error("nested cycle not allowed: {0}")]
    NestedCycle(String),
    #[error("unmatched parenthesis: {0}")]
    UnmatchedParenthesis(String),
    #[error("Values must be enclosed in parentheses: {0}")]
    ValueOutsideParentheses(String),
}

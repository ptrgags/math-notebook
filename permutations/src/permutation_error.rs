use thiserror::Error;

#[derive(Debug, Error)]
pub enum PermutationError {
    #[error("permutations must not repeat elements")]
    RepeatElement,
    #[error("cycles must be disjoint")]
    OverlappingCycles,
    #[error("permutation value must be in [0, {1}), got {0}")]
    ValueOutOfRange(usize, usize),
}

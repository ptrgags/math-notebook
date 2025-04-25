# Abstraction

This Rust package is for representing abstract math concepts as strongly typed
abstractions.

## Contents

- Group-like structures
    - `semigroup::Semigroup` 
        - a type that can be combined with an associative binary operation
        - `sconcat` allows flattening a `&[Semigroup] -> Semigroup` for non-empty slices
    - `monoid::Monoid` 
        - a semigroup with an identity element
        - `mconcat` is similar to `sconcat` but allows empty slices
    - `group::Group` - a monoid where every element has an inverse
- `dfs.rs` - Depth-first search trait. ⚠️ I ended up not using it so might
consider removing it.
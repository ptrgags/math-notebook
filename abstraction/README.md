# Abstraction

This package is for representing abstract math concepts as strongly typed
abstractions.

### Contents

- `lib.rs`
    - `trait Monoid` Denoting a type that is closed under a binary operation, is associative, and has an identity element
    - `trait Group` Like monoid, but it allows inverses.
    - both of these are useful for defining mathematical transformations used in making fractals and tilings
- `dfs.rs` - Depth-first search. I ended up not using this module, so not sure
    if I'll keep this around.

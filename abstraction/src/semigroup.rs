use std::ops::Mul;

/// A Semigroup is a set with an associative binary operator.
/// See [Semigroup on Wikipedia](https://en.wikipedia.org/wiki/Semigroup)
/// This implementation is inspired by [Haskell's Semigroup typeclass](https://hackage.haskell.org/package/base-4.21.0.0/docs/Data-Semigroup.html)
pub trait Semigroup: Clone + Mul<Self, Output = Self> {
    /// Reduces a slice of semigroup values to a single value.
    fn sconcat(values: &[Self]) -> Self
    where
        Self: Sized,
    {
        if values.len() == 0 {
            panic!("sconcat: empty slice not allowed");
        }

        values.iter().cloned().reduce(|accum, x| accum * x).unwrap()
    }
}

use std::ops::Mul;

/// A Semigroup is a set with an associative binary operator.
/// See [Semigroup on Wikipedia](https://en.wikipedia.org/wiki/Semigroup)
/// This implementation is inspired by [Haskell's Semigroup typeclass](https://hackage.haskell.org/package/base-4.21.0.0/docs/Data-Semigroup.html)
///
/// Associativity isn't easily expressed as a type, so see the test macros in this file
pub trait Semigroup: Clone + Mul<Self, Output = Self> {
    /// Reduces a slice of semigroup values to a single value.
    fn sconcat(values: &[Self]) -> Self
    where
        Self: Sized,
    {
        if values.is_empty() {
            panic!("sconcat: empty slice not allowed");
        }

        values.iter().cloned().reduce(|accum, x| accum * x).unwrap()
    }
}

/// Test that (ab)c = a(bc)
#[macro_export]
macro_rules! test_associativity {
    ($t:ty, $label:ident, $a:expr, $b:expr, $c:expr) => {
        #[test]
        fn $label() {
            let a = $a;
            let b = $b;
            let c = $c;

            let ab_c = (a.clone() * b.clone()) * c.clone();
            let a_bc = a * (b * c);

            assert_eq!(ab_c, a_bc);
        }
    };
    ($t:ty, [$(($label: ident, $a: expr, $b:expr, $c: expr)),*]) => {
        mod associativity_law {
            use super::*;
            $(test_associativity!($t, $label, $a, $b, $c);)*
        }
    }
}

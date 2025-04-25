use crate::semigroup::Semigroup;

pub struct PowerIterator<S: Monoid> {
    element: S,
    current: S,
    power: usize,
}

impl<S: Monoid> PowerIterator<S> {
    pub fn new(element: S) -> Self {
        Self {
            element,
            current: S::identity(),
            power: 0,
        }
    }
}

impl<S> Iterator for PowerIterator<S>
where
    S: PartialEq + Monoid,
{
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.clone();
        self.current = self.element.clone() * self.current.clone();

        if self.power > 0 && current == S::identity() {
            // Cycle detected, short-circuit
            None
        } else {
            Some(current)
        }
    }
}

/// A monoid is a set S along with a binary operation (*) such that
/// - S is closed under *. This is realized as requiring Mul<S, Output=S>
/// - S has an identity element I, such that I * x = x * I = x for all x in S
/// - the binary operation is associative. a(bc) = (ab)c for all a, b, c in S
///
/// Another way of saying the same thing: a monoid is a semigroup + identity
pub trait Monoid: Semigroup {
    /// The identity element. This must satisfy
    /// T::identity() * element = element
    /// element * T::identity() = element
    fn identity() -> Self;

    /// Similar to Semigroup::sconcat, but now the list can be empty
    fn mconcat(values: &[Self]) -> Self {
        values
            .iter()
            .cloned()
            .fold(Self::identity(), |accum, x| accum * x)
    }

    /// Raise an element to a specific power
    fn pow(&self, exponent: usize) -> Self {
        if exponent == 0 {
            Self::identity()
        } else if exponent % 2 == 0 {
            let half = self.pow(exponent / 2);
            half.clone() * half
        } else {
            let smaller_half = self.pow((exponent - 1) / 2);
            self.clone() * smaller_half.clone() * smaller_half
        }
    }

    /// Iterate over I, a, a^2, a^3, ...
    /// For elements with finite order, this will stop if the product
    /// equals Self::identity(). Eg.
    /// if a is a 3-cycle permutation, you'd get I, a, a^2, but not a^3 = I
    fn power_iter(&self) -> PowerIterator<Self> {
        PowerIterator::new(self.clone())
    }
}

/// Test that identity * identity = identity (idempotent)
/// Test that identity * x = x * identity = x for each x (identity law)
#[macro_export]
macro_rules! test_identity {
    ($t:ty, $label:ident, $x:expr) => {
        #[test]
        fn $label() {
            let x = $x;
            let i = <$t>::identity();

            let left = i.clone() * x.clone();
            let right = x.clone() * i;

            assert_eq!(left, right);
            assert_eq!(left, x);
        }
    };
    ($t:ty, [$(($label:ident, $e:expr)),*]) => {
        #[test]
        fn identity_is_idempotent() {
            let i = <$t>::identity();

            let result = i.clone() * i.clone();

            assert_eq!(result, i);
        }

        mod identity_law {
            use super::*;
            $(test_identity!($t, $label, $e);)*
        }
    };
}

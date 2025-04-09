pub mod dfs;

use std::ops::Mul;

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

impl<S: Monoid> Iterator for PowerIterator<S> {
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
///   This isn't easily representable in a type, so it's up to the
///   implementation to make sure this is valid.
pub trait Monoid: PartialEq + Clone + Mul<Self, Output = Self> {
    /// The identity element. This must satisfy
    /// T::identity() * element = element
    /// element * T::identity() = element
    fn identity() -> Self;

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

/// A group is a monoid with the additional requirement that elements
/// must have inverses.
pub trait Group: Monoid {
    /// Multiplicative inverse a^-1 such that
    /// a * a^-1 = a^-1 * a = I
    fn inverse(&self) -> Self;

    /// Raise the element to a power. If the exponent is negative, use the
    /// inverse element instead.
    fn pow(&self, exponent: isize) -> Self {
        if exponent < 0 {
            Monoid::pow(&self.inverse(), -exponent as usize)
        } else {
            Monoid::pow(self, exponent as usize)
        }
    }

    /// Iterate over I, a^-1, a^-2, ...
    /// If a has a finite order, this will stop when
    /// the cycle reaches identity.
    fn inv_power_iter(&self) -> PowerIterator<Self> {
        self.inverse().power_iter()
    }

    /// Difference between two transforms
    /// diff(b, a) = ba^-1
    fn difference(b: Self, a: Self) -> Self {
        b * a.inverse()
    }

    /// Sandwich product (aka conjugate) aba^(-1).
    /// This transforms b to do the same thing from the perspective of a
    fn sandwich(bread: Self, filling: Self) -> Self {
        bread.clone() * filling * bread.inverse()
    }

    /// commutator [a, b] = diff(ab, ba) = ab(ba)^-1 = aba^(-1)b^(-1)
    fn commutator(a: Self, b: Self) -> Self {
        a.clone() * b.clone() * a.inverse() * b.inverse()
    }
}

pub trait GroupAction<X>: Group + Mul<X, Output = X> {}
impl<G, X> GroupAction<X> for G where G: Group + Mul<X, Output = X> {}

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

/// Check the inverse law: a * a^-1 = a^-1 * a = I
#[macro_export]
macro_rules! test_inverse {
    ($t:ty, $label:ident, $a:expr) => {
        #[test]
        fn $label() {
            let a = $a;
            let a_inv = $a.inverse();
            let identity = <$t>::identity();

            let a_a_inv = a.clone() * a_inv.clone();
            let a_inv_a = a_inv * a;

            assert_eq!(a_a_inv, a_inv_a);
            assert_eq!(a_a_inv, identity);
        }
    };
    ($t:ty, [$(($label:ident, $a:expr)),*]) => {
        mod inverse_law {
            use super::*;

            $(test_inverse!($t, $label, $a);)*
        }
    };
}

#[macro_export]
macro_rules! test_group {
    ($t:ty, $label:ident, $a:expr, $b:expr) => {
        mod $label {
            use super::*;

            #[test]
            fn diff_maps_a_to_b() {
                let a = $a;
                let b = $b;
                let diff = <$t>::difference(b, a);

                let result = diff * a;

                assert_eq!(result, b);
            }

            #[test]
            fn diff_ab_is_inverse_of_diff_ba() {
                let a = $a;
                let b = $b;
                let diff_ab = <$t>::difference(a, b);
                let diff_ba = <$t>::difference(b, a);

                let diff_ab_inv = diff_ab.inverse();

                assert_eq!(diff_ab_inv, diff_ba);
            }

            #[test]
            fn self_sandwich_is_idempotent() {
                let a = $a;

                let result = <$t>::sandwich(a, a);

                assert_eq!(result, a);
            }

            #[test]
            fn element_commutes_with_itself() {
                let a = $a;
                let identity = <$t>::identity();

                let result = <$t>::commutator(a, a);

                assert_eq!(result, identity);
            }

            fn commutator_is_difference_ab_ba() {
                let a = $a;
                let b = $b;
                let ab = a * b;
                let ba = b * a;

                let comm = <$t>::commutator(a, b);
                let diff = <$t>::difference(ab, ba);

                assert_eq!(comm, diff);
            }
        }
    };

    ($t:ty, [$(($label:ident, $a:expr, $b:expr)),*]) => {
        mod group_ops {
            use super::*;

            $(test_group!($t, $label, $a, $b);)*
        }
    };
}

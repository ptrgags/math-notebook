use std::ops::Mul;

use crate::monoid::{Monoid, PowerIterator};

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

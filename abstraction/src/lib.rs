use std::ops::Mul;

/// A semigroup is a set S along with a binary operation (*) such that
/// - S is closed under *. This is realized as requiring Mul<S, Output=S>
/// - S has an identity element I, such that I * x = x * I = x for all x in S
/// - the binary operation is associative. a(bc) = (ab)c for all a, b, c in S
///   This isn't easily representable in a type, so it's up to the
///   implementation to make sure this is valid.
pub trait Semigroup: Copy + Mul<Self, Output = Self> {
    /// The identity element. This must satisfy
    /// T::identity() * element = element
    /// element * T::identity() = element
    fn identity() -> Self;
}

/// A group is a semigroup with the additional requirement that elements
/// must have inverses.
pub trait Group: Semigroup {
    /// Multiplicative inverse a^-1 such that
    /// a * a^-1 = a^-1 * a = I
    fn inverse(&self) -> Self;

    /// Difference between two transforms
    /// diff(b, a) = ba^-1
    fn difference(b: Self, a: Self) -> Self {
        b * a.inverse()
    }

    /// Sandwich product (aka conjugate) aba^(-1).
    /// This transforms b to do the same thing from the perspective of a
    fn sandwich(bread: Self, filling: Self) -> Self {
        bread * filling * bread.inverse()
    }

    /// commutator [a, b] = diff(ab, ba) = ab(ba)^-1 = aba^(-1)b^(-1)
    fn commutator(a: Self, b: Self) -> Self {
        a * b * a.inverse() * b.inverse()
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

            let left = i * x;
            let right = x * i;

            assert_eq!(left, right);
            assert_eq!(left, x);
        }
    };
    ($t:ty, [$(($label:ident, $e:expr)),*]) => {
        #[test]
        fn identity_is_idempotent() {
            let i = <$t>::identity();

            let result = i * i;

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

            let ab_c = (a * b) * c;
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

            let a_a_inv = a * a_inv;
            let a_inv_a = a_inv * a;

            assert_eq!(a_a_inv, a_inv_a);
            assert_eq!(a_a_inv, identity);
        }
    };
    ($t:ty, [$(($label:ident, $a:expr)),*]) => {
        mod identity_law {
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

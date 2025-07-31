/// A basis blade in GA is a wedge product of basis vectors. This struct
/// helps with the bookkeeping of the basis vectors, but not the coefficient
/// which will be handled by Multivector.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct BasisBlade(u8);

impl BasisBlade {
    /// Every geometric algebra has scalars, i.e. the absence of scalars
    pub fn scalar() -> Self {
        Self(0)
    }

    /// A single basis vector e_index
    pub fn vector(index: u8) -> Self {
        Self(1 << index)
    }

    // A bivector e_i ^ e_j
    pub fn bivector(i: u8, j: u8) -> Self {
        Self(1 << i | 1 << j)
    }

    // A trivector e_i ^ e_j ^ e_k
    pub fn trivector(i: u8, j: u8, k: u8) -> Self {
        Self(1 << i | 1 << j | 1 << k)
    }

    // A quadvector e_i ^ e_j ^ e_k ^ e_l
    pub fn quadvector(i: u8, j: u8, k: u8, l: u8) -> Self {
        Self(1 << i | 1 << j | 1 << k | 1 << l)
    }

    // A pentavector e_i ^ e_j ^ e_k ^ e_l ^ e_m
    pub fn pentavector(i: u8, j: u8, k: u8, l: u8, m: u8) -> Self {
        Self(1 << i | 1 << j | 1 << k | 1 << l | 1 << m)
    }

    // Construct a basis blade from a bit pattern.
    pub fn new(value: u8) -> Self {
        Self(value)
    }

    /// when taking a geometric product of two basis blades,
    /// the resulting blade will be the symmetric difference of the
    /// basis vectors contained in each blade
    pub fn symmetric_diff(&self, other: &Self) -> Self {
        let Self(x) = self;
        let Self(y) = other;
        Self(x ^ y)
    }

    /// When taking a geometric product of two basis blades, the sign
    /// of the result will determine on which basis vectors are in the
    /// intersection of the two basis blades. The sign will be handled
    /// by Multivector
    pub fn intersection(&self, other: &Self) -> Self {
        let Self(x) = self;
        let Self(y) = other;
        Self(x & y)
    }

    /// When taking the geometric product ab, how many anticommutative swaps
    /// are needed to sort the vectors in order?
    pub fn product_swap_count(a: &BasisBlade, b: &BasisBlade) -> u8 {
        let &Self(a_bits) = a;
        let &Self(b_bits) = b;

        // Let's say we have
        // xyzw xyzw
        // 0123 0123
        //
        // First we need to move the right x to the left, w, z, y = 3 places
        //
        // xxyzw yzw
        //  ^--- 123
        //
        // Second, we move the right y to the left, w, z = 2 places
        //
        // xxyyzw zw
        //    ^--
        //
        // Third, we move the right z one place to the left, w = 1 place
        //
        // xxyyzzw w
        //      ^-
        //
        // The last w just gets tacked on at the end, 0 places
        //
        // xxyyzzww
        //
        // now... some of these steps get skipped if there are vectors
        // absent. so check first before incrementing the step counter

        let a_stop = 8 - a_bits.leading_zeros();
        let b_stop = 8 - b_bits.leading_zeros();

        let mut swaps = 0;

        // Start with b's e0 vector and count up e1, e2, ... ek
        for i in 0..b_stop {
            // no vector, continue
            if (b_bits >> i) & 1 == 0 {
                continue;
            }

            // for a's vector, we want to count _down_ e_l, e_(l - 1), ... e_(i + 1)
            // stopping just short of the vector in the place specified
            for j in (i + 1..a_stop).rev() {
                // There wasn't a vector in this spot, nothing to commute
                if (a_bits >> j) & 1 == 0 {
                    continue;
                }

                // We commuted one vector to the left
                swaps += 1;
            }
        }

        swaps
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn swap_count_with_scalars_is_zero() {
        let scalar = BasisBlade::scalar();

        let result = BasisBlade::product_swap_count(&scalar, &scalar);

        assert_eq!(result, 0);
    }

    #[test]
    pub fn swap_count_x_x_is_zero() {
        let x = BasisBlade::vector(0);

        let result = BasisBlade::product_swap_count(&x, &x);

        assert_eq!(result, 0);
    }

    #[test]
    pub fn swap_count_x_y_is_zero() {
        let x = BasisBlade::vector(0);
        let y = BasisBlade::vector(1);

        let result = BasisBlade::product_swap_count(&x, &y);

        assert_eq!(result, 0);
    }

    #[test]
    pub fn swap_count_y_x_is_one() {
        let x = BasisBlade::vector(0);
        let y = BasisBlade::vector(1);

        let result = BasisBlade::product_swap_count(&y, &x);

        assert_eq!(result, 1);
    }

    #[test]
    pub fn swap_count_xy_xy_is_one() {
        let xy = BasisBlade::bivector(0, 1);

        let result = BasisBlade::product_swap_count(&xy, &xy);

        assert_eq!(result, 1);
    }

    #[test]
    pub fn swap_count_yz_yz_is_one() {
        let yz = BasisBlade::bivector(1, 2);

        let result = BasisBlade::product_swap_count(&yz, &yz);

        assert_eq!(result, 1);
    }

    #[test]
    pub fn swap_count_xyzw_xyzw_is_six() {
        let xyzw = BasisBlade::quadvector(0, 1, 2, 3);

        let result = BasisBlade::product_swap_count(&xyzw, &xyzw);

        assert_eq!(result, 6);
    }

    #[test]
    pub fn swap_count_xzw_y_is_two() {
        let xzw = BasisBlade::trivector(0, 2, 3);
        let y = BasisBlade::vector(1);

        let result = BasisBlade::product_swap_count(&xzw, &y);

        assert_eq!(result, 2);
    }

    #[test]
    pub fn swap_count_xy_zw_is_zero() {
        let xy = BasisBlade::bivector(0, 1);
        let zw = BasisBlade::bivector(2, 3);

        let result = BasisBlade::product_swap_count(&xy, &zw);

        assert_eq!(result, 0);
    }
}

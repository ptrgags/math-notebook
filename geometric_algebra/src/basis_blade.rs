use std::fmt::Debug;

/// A basis blade in GA is a wedge product of basis vectors. This struct
/// helps with the bookkeeping of the basis vectors, but not the coefficient
/// which will be handled by Multivector.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct BasisBlade(pub u8);

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

    fn get_blades_recursive(output: &mut Vec<BasisBlade>, prefix: u8, dimension: u8, grade: u8) {
        // No more choices to make, the prefix is the blade we want so add it to the output.
        if grade == 0 {
            output.push(BasisBlade(prefix));
            return;
        }

        // Always select vectors in increasing order of index
        // To do this, we select one of the available vectors, and
        // see what the remaining
        //
        // E.g. in 5d for a trivector (let's use basis x, y, z, p, m as in CGA)
        // the decision tree looks like this:
        //
        // x | y | z
        //   |   | p
        //   |   | m
        //   | z | p
        //   |   | m
        //   | p | m
        // y | z | p
        //   |   | m
        // z | p | m

        let choices = dimension - grade + 1;
        for i in 0..choices {
            let selected_vector = 1 << i;
            let remaining_dimensions = dimension - (i + 1);
            let mut sub_choices = Vec::new();
            Self::get_blades_recursive(&mut sub_choices, 0, remaining_dimensions, grade - 1);

            output.extend(sub_choices.into_iter().map(|BasisBlade(x)| {
                // the sub-choices are the high bits, so shift them up and
                // union with the selected vector to simulate the wedge product.
                //
                // example:
                // i = 1
                // selected_vector = 0b10 (y)
                // sub choice x = 0b11. We don't want xy, but rather 1st and
                // second vectors after y, i.e. z, w. So we get 0b1100 after
                // shifting
                //
                // finally, unioning them together we get 0b1110, i.e.
                // the trivector yzw
                BasisBlade(x << (i + 1) | selected_vector)
            }));
        }
    }

    fn get_blades_for_grade(dimension: u8, grade: u8) -> Vec<BasisBlade> {
        // 0D is scalars only. In any dimension, grade 0 is a single scalar.
        if dimension == 0 || grade == 0 {
            return vec![BasisBlade::scalar()];
        }

        // Shouldn't happen the way I iterate.
        if grade > dimension {
            panic!("grade bigger than dimension!");
        }

        let mut result = Vec::new();
        Self::get_blades_recursive(&mut result, 0, dimension, grade);

        result
    }

    pub fn get_all_blades(dimension: u8) -> Vec<BasisBlade> {
        let mut result = Vec::new();
        for grade in 0..=dimension {
            let k_blades = Self::get_blades_for_grade(dimension, grade);
            result.extend(k_blades.into_iter());
        }
        result
    }

    pub fn get_even_blades(dimension: u8) -> Vec<BasisBlade> {
        let mut result = Vec::new();
        for grade in 0..=dimension {
            if grade % 2 == 1 {
                continue;
            }

            let k_blades = Self::get_blades_for_grade(dimension, grade);
            result.extend(k_blades.into_iter());
        }
        result
    }

    pub fn get_odd_blades(dimension: u8) -> Vec<BasisBlade> {
        let mut result = Vec::new();
        for grade in 0..=dimension {
            if grade % 2 == 0 {
                continue;
            }

            let k_blades = Self::get_blades_for_grade(dimension, grade);
            result.extend(k_blades.into_iter());
        }
        result
    }
}

impl Debug for BasisBlade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BasisBlade({:#b})", self.0)
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

    #[test]
    pub fn get_all_blades_with_0d_returns_blades_in_correct_order() {
        let result = BasisBlade::get_all_blades(0);

        let expected = vec![
            // scalar
            BasisBlade(0b0),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    pub fn get_all_blades_with_1d_returns_blades_in_correct_order() {
        let result = BasisBlade::get_all_blades(1);

        let expected = vec![
            // scalar
            BasisBlade(0b0),
            // vectors
            BasisBlade(0b1),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    pub fn get_all_blades_with_2d_returns_blades_in_correct_order() {
        let result = BasisBlade::get_all_blades(2);

        let expected = vec![
            // scalar
            BasisBlade(0b00),
            // vectors
            BasisBlade(0b01),
            BasisBlade(0b10),
            // bivectors
            BasisBlade(0b11),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    pub fn get_all_blades_with_3d_returns_blades_in_correct_order() {
        let result = BasisBlade::get_all_blades(3);

        let expected = vec![
            // scalar
            BasisBlade(0b000),
            // vectors
            BasisBlade(0b001),
            BasisBlade(0b010),
            BasisBlade(0b100),
            // bivectors
            BasisBlade(0b011),
            BasisBlade(0b101),
            BasisBlade(0b110),
            // trivector
            BasisBlade(0b111),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    pub fn get_all_blades_with_4d_returns_blades_in_correct_order() {
        let result = BasisBlade::get_all_blades(4);

        let expected = vec![
            // scalar
            BasisBlade(0b0000),
            // vectors
            BasisBlade(0b0001),
            BasisBlade(0b0010),
            BasisBlade(0b0100),
            BasisBlade(0b1000),
            // bivectors
            BasisBlade(0b0011),
            BasisBlade(0b0101),
            BasisBlade(0b1001),
            BasisBlade(0b0110),
            BasisBlade(0b1010),
            BasisBlade(0b1100),
            // trivectors
            BasisBlade(0b0111),
            BasisBlade(0b1011),
            BasisBlade(0b1101),
            BasisBlade(0b1110),
            // Quadvector
            BasisBlade(0b1111),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    pub fn get_even_blades_returns_only_even_grades() {
        let result = BasisBlade::get_even_blades(4);

        let expected = vec![
            // scalar
            BasisBlade(0b0000),
            // no vectors
            // bivectors
            BasisBlade(0b0011),
            BasisBlade(0b0101),
            BasisBlade(0b1001),
            BasisBlade(0b0110),
            BasisBlade(0b1010),
            BasisBlade(0b1100),
            // no trivectors
            // quadvector
            BasisBlade(0b1111),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    pub fn get_odd_blades_returns_only_odd_grades() {
        let result = BasisBlade::get_odd_blades(4);

        let expected = vec![
            // no scalar
            // vectors
            BasisBlade(0b0001),
            BasisBlade(0b0010),
            BasisBlade(0b0100),
            BasisBlade(0b1000),
            // no bivectors
            // trivectors
            BasisBlade(0b0111),
            BasisBlade(0b1011),
            BasisBlade(0b1101),
            BasisBlade(0b1110),
            // no Quadvector
        ];

        assert_eq!(result, expected);
    }
}

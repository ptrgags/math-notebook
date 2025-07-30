/// A basis blade in GA is a wedge product of basis vectors. This struct
/// helps with the bookkeeping of the basis vectors, but not the coefficient
/// which will be handled by Multivector.
#[derive(Clone, Copy, Eq)]
pub struct BasisVector(u8);


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
    pub fn trivector(i: u8, j: u8, k: u8) -> self {
        Self(1 << i | 1 << j | 1 << k)
    }

    // A quadvector e_i ^ e_j ^ e_k ^ e_l
    pub fn quadvector(i: u8, j: u8, k: u8, l: u8) -> self {
        Self(1 << i | 1 << j | 1 << k | 1 << l)
    }

    // A pentavector e_i ^ e_j ^ e_k ^ e_l ^ e_m
    pub fn pentavector(i: u8, j: u8, k: u8, l: u8, m: u8) -> self {
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
}
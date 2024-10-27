use crate::{isogonal::Isogonal, Mobius};

/// This trait represents a graphics primitive that can be transformed
/// by a mobius transformation.
pub trait ConformalPrimitive {
    fn transform(&self, xform: Mobius) -> Self;
}

/// This trait represents a graphics primitive that can be transformed
/// by an Isogonal transformation. These tend to be low-level
/// primitives that are better for computation but not very human-friendly
pub trait IsogonalPrimitive {
    fn transform(&self, xform: Isogonal) -> Self;
}

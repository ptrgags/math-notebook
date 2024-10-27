pub mod cline;
pub mod collection;

use crate::{isogonal::Isogonal, Mobius};
pub use cline::*;
pub use collection::*;

pub trait Transformable<T> {
    fn transform(&self, xform: T) -> Self;
}

/// If you can transform using an isogonal transform, then you can trivially
/// transform with a Mobius transform (as Mobius maps are a subgroup of
/// the isogonal maps I use)
impl<T: Transformable<Isogonal>> Transformable<Mobius> for T {
    fn transform(&self, xform: Mobius) -> Self {
        self.transform(Isogonal::Conformal(xform))
    }
}

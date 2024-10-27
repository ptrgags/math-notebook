use std::fmt::Display;

use crate::isogonal::Isogonal;

use super::Transformable;

/// A generic collection of transformable primitives of the same type
pub struct Collection<T: Transformable<Isogonal>> {
    primitives: Vec<T>,
}

impl<T: Transformable<Isogonal>> Collection<T> {
    pub fn new(primitives: Vec<T>) -> Self {
        Self { primitives }
    }

    pub fn get_primitives(&self) -> &[T] {
        &self.primitives
    }
}

impl<T: Transformable<Isogonal>> Transformable<Isogonal> for Collection<T> {
    fn transform(&self, xform: Isogonal) -> Self {
        let primitives: Vec<T> = self.primitives.iter().map(|x| x.transform(xform)).collect();

        Self { primitives }
    }
}

impl<T: Display + Transformable<Isogonal>> Display for Collection<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for primitive in self.primitives.iter() {
            write!(f, "{}\n", primitive)?;
        }
        Ok(())
    }
}

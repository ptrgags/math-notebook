use std::{error::Error, fmt::Display};

use crate::{
    cline_arc::ClineArc,
    isogonal::Isogonal,
    rendering::{RenderPrimitive, Renderable},
};

use super::{Cline, Transformable};

/// A generic collection of transformable primitives of the same type
#[derive(Clone)]
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

impl<T: Renderable + Transformable<Isogonal>> Renderable for Collection<T> {
    fn bake_geometry(&self) -> Result<Vec<RenderPrimitive>, Box<dyn Error>> {
        let mut errors = vec![];
        let baked: Vec<_> = self
            .primitives
            .iter()
            .map(|x| x.bake_geometry())
            .filter_map(|x| x.map_err(|x| errors.push(x)).ok())
            .flatten()
            .collect();

        if !errors.is_empty() {
            println!("Warning: Errors detected when baking geometry. Skipping these primitives");
            for error in errors {
                println!("{}", error)
            }
        }

        Ok(baked)
    }
}

pub type ClineTile = Collection<Cline>;
pub type ClineArcTile = Collection<ClineArc>;

use std::{error::Error, fmt::Display};

use rendering::{RenderPrimitive, Renderable};

use crate::{cline_arc::ClineArc, isogonal::Isogonal};

use super::{Cline, Transformable};

/// A generic collection of transformable primitives of the same type
#[derive(Clone)]
pub struct Collection<T> {
    primitives: Vec<T>,
}

impl<T> Collection<T> {
    pub fn new(primitives: Vec<T>) -> Self {
        Self { primitives }
    }

    pub fn get_primitives(&self) -> &[T] {
        &self.primitives
    }

    pub fn union(primitives: Vec<Self>) -> Self {
        let all_primitives: Vec<T> = primitives.into_iter().flat_map(|x| x.primitives).collect();
        Self {
            primitives: all_primitives,
        }
    }
}

impl<T: Transformable<Isogonal>> Transformable<Isogonal> for Collection<T> {
    fn transform(&self, xform: Isogonal) -> Self {
        let primitives: Vec<T> = self.primitives.iter().map(|x| x.transform(xform)).collect();

        Self { primitives }
    }
}

impl<T: Display> Display for Collection<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for primitive in self.primitives.iter() {
            writeln!(f, "{}", primitive)?;
        }
        Ok(())
    }
}

impl<T: Renderable> Renderable for Collection<T> {
    fn render(&self) -> Result<RenderPrimitive, Box<dyn Error>> {
        let mut errors = vec![];
        let baked: Vec<RenderPrimitive> = self
            .primitives
            .iter()
            .map(|x| x.render())
            .filter_map(|x| x.map_err(|x| errors.push(x)).ok())
            .collect();

        if !errors.is_empty() {
            println!("Warning: Errors detected when baking geometry. Skipping these primitives");
            for error in errors {
                println!("{}", error)
            }
        }

        Ok(RenderPrimitive::group(baked))
    }
}

pub type ClineTile = Collection<Cline>;
pub type ClineArcTile = Collection<ClineArc>;

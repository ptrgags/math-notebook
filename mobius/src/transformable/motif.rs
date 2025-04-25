use std::{fmt::Display, ops::Mul};

use abstraction::semigroup::Semigroup;
use rendering::{style::Style, RenderPrimitive, Renderable};

use crate::isogonal::Isogonal;

use super::Transformable;

#[derive(Clone, PartialEq)]
pub struct Motif<T> {
    parts: Vec<(T, usize)>,
}

impl<T> Motif<T> {
    pub fn new(parts: Vec<(T, usize)>) -> Self {
        Self { parts }
    }

    pub fn iter(&self) -> impl Iterator<Item = &(T, usize)> {
        self.parts.iter()
    }
}

impl<T: Clone> Mul for Motif<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut parts = self.parts.clone();
        parts.extend(rhs.parts.clone());
        Self { parts }
    }
}

impl<T: Clone> Semigroup for Motif<T> {
    fn sconcat(values: &[Self]) -> Self
    where
        Self: Sized,
    {
        let all_parts: Vec<(T, usize)> = values.iter().cloned().flat_map(|x| x.parts).collect();
        Self { parts: all_parts }
    }
}

impl<T: Renderable> Motif<T> {
    pub fn render_group(&self, styles: &[Style]) -> RenderPrimitive {
        let primitives: Vec<RenderPrimitive> = self
            .parts
            .iter()
            .map(|(part, style_index)| {
                let primitive = part.render().unwrap();
                let style = styles[*style_index];

                RenderPrimitive::Group(vec![primitive], style)
            })
            .collect();
        RenderPrimitive::group(primitives)
    }
}

impl<T: Transformable<Isogonal>> Transformable<Isogonal> for Motif<T> {
    fn transform(&self, xform: Isogonal) -> Self {
        let parts = self
            .parts
            .iter()
            .map(|(tile, id)| (tile.transform(xform), *id))
            .collect();

        Self { parts }
    }
}

impl<T: Transformable<Isogonal> + Display> Display for Motif<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (tile, id) in self.iter() {
            write!(f, "{}\n{}\n", tile, id)?;
        }
        Ok(())
    }
}

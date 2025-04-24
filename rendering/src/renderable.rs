use std::error::Error;

use crate::primitive::RenderPrimitive;

pub trait Renderable {
    fn bake_geometry(&self) -> Result<Vec<RenderPrimitive>, Box<dyn Error>>;
}

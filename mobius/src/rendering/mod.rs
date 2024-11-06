pub mod render_primitive;
pub mod style;

use std::error::Error;

pub use render_primitive::*;
pub use style::*;

pub trait Renderable {
    fn bake_geometry(&self) -> Result<Vec<RenderPrimitive>, Box<dyn Error>>;
}

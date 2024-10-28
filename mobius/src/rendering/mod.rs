pub mod render_primitive;
pub mod style;

pub use render_primitive::*;
pub use style::*;

pub trait Renderable {
    fn bake_geometry(&self) -> Vec<RenderPrimitive>;
}
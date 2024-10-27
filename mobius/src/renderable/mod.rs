pub mod render_primitive;

pub use render_primitive::*;

pub trait Renderable {
    fn bake_geometry(&self) -> Vec<RenderPrimitive>;
}

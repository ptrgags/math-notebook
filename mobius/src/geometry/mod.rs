pub mod arc_angles;
pub mod circle;
pub mod circular_arc;
pub mod double_ray;
pub mod line;
pub mod line_segment;
pub mod ray;

use crate::Complex;

pub use arc_angles::*;
pub use circle::*;
pub use circular_arc::*;
pub use double_ray::*;
pub use line::*;
pub use line_segment::*;
pub use ray::*;

/// Human-understandable geometry objects
pub trait Geometry {}

/// Directed edge. If a geometry type defines this,
/// then it can be used to make a polygon
pub trait DirectedEdge {
    fn start(&self) -> Complex;
    fn end(&self) -> Complex;
}

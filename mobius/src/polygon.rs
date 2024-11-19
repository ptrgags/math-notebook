use thiserror::Error;

use crate::{
    cline_arc::{ClineArc, ClineArcGeometry},
    geometry::DirectedEdge,
    rendering::{PathCommand, RenderPrimitive, Renderable},
};

#[derive(Debug, Error)]
pub enum PolygonError {
    #[error("polygons must have at least 2 edges")]
    TooFewEdges,
    #[error("polygon edges must be connected")]
    Discontinuity,
    #[error("can't render polygon with infinite edge")]
    InfiniteEdge,
}

#[derive(Debug)]
pub struct Polygon {
    edges: Vec<ClineArc>,
}

impl Polygon {
    pub fn new(edges: Vec<ClineArc>) -> Result<Self, PolygonError> {
        let n = edges.len();
        if n < 2 {
            return Err(PolygonError::TooFewEdges);
        }

        for (i, edge) in edges.iter().enumerate() {
            if edge.end() != edges[(i + 1) % n].start() {
                return Err(PolygonError::Discontinuity);
            }
        }

        Ok(Self { edges })
    }
}

impl Renderable for Polygon {
    fn bake_geometry(&self) -> Result<Vec<RenderPrimitive>, Box<dyn std::error::Error>> {
        let start = self.edges[0].start();
        let mut commands = vec![PathCommand::MoveTo(start)];

        for edge in self.edges.iter() {
            match edge.classify()? {
                ClineArcGeometry::CircularArc(circular_arc) => {
                    commands.push(PathCommand::ArcTo(circular_arc))
                }
                ClineArcGeometry::LineSegment(line_segment) => {
                    commands.push(PathCommand::LineTo(line_segment.end()))
                }
                _ => return Err(PolygonError::InfiniteEdge.into()),
            }
        }

        Ok(vec![RenderPrimitive::Polygon(commands)])
    }
}

#[cfg(test)]
mod test {
    use std::{
        error::Error,
        f64::consts::{PI, TAU},
    };

    use crate::{
        geometry::{ArcAngles, Circle, CircularArc, LineSegment},
        Complex,
    };

    use super::*;

    type Res = Result<(), Box<dyn Error>>;

    #[test]
    pub fn new_with_empty_vec_returns_error() {
        let result = Polygon::new(vec![]);

        assert!(matches!(result, Err(PolygonError::TooFewEdges)))
    }

    #[test]
    pub fn new_with_one_edge_returns_error() {
        let segment: ClineArc = LineSegment::new(Complex::Zero, Complex::ONE).into();

        let result = Polygon::new(vec![segment]);

        assert!(matches!(result, Err(PolygonError::TooFewEdges)))
    }

    #[test]
    pub fn new_with_digon_constructs() -> Res {
        let circle = Circle::unit_circle();
        let upper = CircularArc::new(circle, ArcAngles::new(0.0, PI)?);
        let lower = CircularArc::new(circle, ArcAngles::new(PI, TAU)?);

        let result = Polygon::new(vec![upper.into(), lower.into()]);

        assert!(matches!(result, Ok(_)));

        Ok(())
    }
}

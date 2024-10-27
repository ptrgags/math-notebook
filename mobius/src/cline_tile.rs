use std::fmt::Display;

use crate::{
    cline_arc::ClineArc,
    isogonal::Isogonal,
    renderable::{RenderPrimitive, Renderable},
    transformable::{Cline, Transformable},
};

#[derive(Clone)]
pub struct ClineTile {
    clines: Vec<Cline>,
}

impl ClineTile {
    pub fn new(clines: Vec<Cline>) -> Self {
        Self { clines }
    }

    pub fn get_clines(&self) -> &[Cline] {
        &self.clines
    }
}

impl Transformable<Isogonal> for ClineTile {
    fn transform(&self, xform: Isogonal) -> Self {
        let clines: Vec<Cline> = self.clines.iter().map(|x| x.transform(xform)).collect();

        Self { clines }
    }
}

impl Renderable for ClineTile {
    fn bake_geometry(&self) -> Vec<RenderPrimitive> {
        self.clines.iter().flat_map(|x| x.bake_geometry()).collect()
    }
}

impl Display for ClineTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for cline in self.clines.iter() {
            write!(f, "{}\n", cline.classify())?;
        }

        Ok(())
    }
}

pub struct ClineArcTile {
    arcs: Vec<ClineArc>,
}

impl ClineArcTile {
    pub fn new(arcs: Vec<ClineArc>) -> Self {
        Self { arcs }
    }

    pub fn get_arcs(&self) -> &[ClineArc] {
        &self.arcs
    }
}

impl Transformable<Isogonal> for ClineArcTile {
    fn transform(&self, xform: Isogonal) -> Self {
        let arcs = self.arcs.iter().map(|x| x.transform(xform)).collect();
        Self { arcs }
    }
}

impl Renderable for ClineArcTile {
    fn bake_geometry(&self) -> Vec<RenderPrimitive> {
        self.arcs.iter().flat_map(|x| x.bake_geometry()).collect()
    }
}

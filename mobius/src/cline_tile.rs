use std::fmt::Display;

use crate::{cline::Cline, cline_arc::ClineArc, Mobius};

#[derive(Clone)]
pub struct ClineTile {
    clines: Vec<Cline>,
}

impl ClineTile {
    pub fn new(clines: Vec<Cline>) -> Self {
        Self { clines }
    }

    pub fn transform(&self, xform: Mobius) -> Self {
        let transformed = self
            .clines
            .iter()
            .map(|x| Cline::transform(xform, *x))
            .collect();

        Self {
            clines: transformed,
        }
    }

    pub fn get_clines(&self) -> &[Cline] {
        &self.clines
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

    pub fn transform(&self, xform: Mobius) -> Self {
        let transformed: Vec<ClineArc> = self
            .arcs
            .iter()
            .map(|x| ClineArc::transform(xform, *x))
            .collect();

        Self { arcs: transformed }
    }

    pub fn get_arcs(&self) -> &[ClineArc] {
        &self.arcs
    }
}

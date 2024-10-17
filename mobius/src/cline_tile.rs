use std::fmt::Display;

use crate::{cline::Cline, Mobius};

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

#[cfg(test)]
mod test {
    use super::*;
}

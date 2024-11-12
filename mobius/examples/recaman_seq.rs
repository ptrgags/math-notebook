use std::{collections::HashSet, error::Error, f64::consts::PI};

use abstraction::Semigroup;
use mobius::{
    cline_arc::ClineArc,
    geometry::ArcDirection,
    orthogonal_arcs::integer_arc,
    rendering::Style,
    rotation, scale,
    svg_plot::{render_views, style_geometry, View},
    transformable::{ClineArcTile, Transformable},
    translation, Complex, Mobius,
};

/// Recamán's sequence.
/// a[0] = 0
/// a[n] = a[n - 1] - n if possible and not already visited
/// a[n] = a[n - 1] + n otherwise
///
/// See the [Numberphile video](https://youtu.be/FGC5TdIiT9U?si=RTJw6SnZMGj739eA)
#[derive(Default)]
pub struct RecamanSequence {
    index: usize,
    current: usize,
    visited: HashSet<usize>,
}

impl RecamanSequence {
    pub fn new() -> Self {
        Self {
            index: 0,
            current: 0,
            visited: HashSet::new(),
        }
    }
}

impl Iterator for RecamanSequence {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let term = self.current;
        self.visited.insert(self.current);

        self.index += 1;
        // If there's an empty spot n steps to the left, go there,
        // else go to the right n steps.
        if self.current > self.index && !self.visited.contains(&(self.current - self.index)) {
            self.current -= self.index;
        } else {
            self.current += self.index;
        }

        Some(term)
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let max_terms = 100;
    let seq = RecamanSequence::new();
    let terms: Vec<usize> = seq.take(max_terms).collect();

    let &max_value = terms.iter().reduce(|acc, x| acc.max(x)).unwrap_or(&0);
    let radius = max_value as f64;
    let center = 0.5 * radius;

    let mut arcs = Vec::new();
    for i in 0..(terms.len() - 1) {
        let a = terms[i];
        let b = terms[i + 1];

        let above = i % 2 == 0;
        let direction = match (above, b > a) {
            (true, true) => ArcDirection::Clockwise,
            (true, false) => ArcDirection::Counterclockwise,
            (false, true) => ArcDirection::Counterclockwise,
            (false, false) => ArcDirection::Clockwise,
        };

        let arc = integer_arc(a as i64, b as i64, direction)?;
        arcs.push(ClineArc::from(arc))
    }

    let tile = ClineArcTile::new(arcs);
    let rot90 = rotation(PI / 2.0)?;
    let zoom_out = scale(1.0 / radius)?;
    let translate_center = translation(Complex::new(0.0, -center))?;
    let in_view = tile.transform(translate_center * rot90);

    let style = Style::stroke(255, 255, 255).with_width(0.125);
    render_views(
        "output",
        "recaman",
        &[View("", 0.0, 0.0, radius / 2.5)],
        style_geometry(style, &in_view),
    )?;

    Ok(())
}

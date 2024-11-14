use std::{error::Error, f64::consts::PI};

use mobius::{
    cline_arc::ClineArc,
    integer_arcs::{integer_arc_by_direction, integer_arc_by_hemisphere, Hemisphere},
    rendering::Style,
    rotation,
    svg_plot::{render_views, style_geometry, View},
    transformable::{ClineArcTile, Transformable},
    translation, Complex,
};

#[derive(Debug, thiserror::Error)]
enum BracketError {
    #[error("Characters must be [ or ], got {0}")]
    InvalidCharacter(char),
    #[error("Unbalanced brackets: {0}")]
    Unbalanced(String),
}

#[derive(PartialEq, Clone, Copy)]
enum Bracket {
    Left,
    Right,
}

struct BalancedBrackets {
    brackets: Vec<Bracket>,
}

impl BalancedBrackets {
    pub fn new(bracket_string: &str) -> Result<Self, BracketError> {
        let mut depth: i32 = 0;
        let mut brackets = Vec::new();
        for c in bracket_string.chars() {
            match c {
                '[' => {
                    brackets.push(Bracket::Left);
                    depth += 1;
                }
                ']' => {
                    brackets.push(Bracket::Right);
                    depth -= 1;
                }
                x => return Err(BracketError::InvalidCharacter(x)),
            }
            if depth < 0 {
                return Err(BracketError::Unbalanced(String::from(bracket_string)));
            }
        }

        if depth != 0 {
            return Err(BracketError::Unbalanced(String::from(bracket_string)));
        }

        Ok(Self { brackets })
    }

    pub fn len(&self) -> usize {
        self.brackets.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (i64, i64)> + '_ {
        BracketIterator::new(&self.brackets)
    }
}

struct BracketIterator<'a> {
    brackets: &'a [Bracket],
    index: usize,
    stack: Vec<i64>,
}

impl<'a> BracketIterator<'a> {
    pub fn new(brackets: &'a [Bracket]) -> Self {
        Self {
            brackets,
            index: 0,
            stack: Vec::new(),
        }
    }
}

impl<'a> Iterator for BracketIterator<'a> {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.brackets.len() {
            // Increment the index whether we short circuit or not.
            let current = self.index;
            self.index += 1;

            match self.brackets[current] {
                Bracket::Left => self.stack.push(current as i64),
                Bracket::Right => {
                    let a = self.stack.pop().unwrap();
                    let b = current as i64;
                    return Some((a, b));
                }
            }
        }

        None
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let north_brackets = BalancedBrackets::new("[][[]][][][[[]][]]")?;
    let mut arcs: Vec<ClineArc> = Vec::new();
    north_brackets.iter().for_each(|(a, b)| {
        let arc = integer_arc_by_hemisphere(a, b, Hemisphere::North).unwrap();
        arcs.push(arc.into());
    });

    let south_brackets = BalancedBrackets::new("[[][][]][[[]][]][]")?;
    south_brackets.iter().for_each(|(a, b)| {
        let arc = integer_arc_by_hemisphere(a, b, Hemisphere::South).unwrap();
        arcs.push(arc.into());
    });

    assert_eq!(north_brackets.len(), south_brackets.len());

    let tile = ClineArcTile::new(arcs);
    let rot90 = rotation(PI / 2.0)?;
    let radius = 0.5 * (north_brackets.len() as f64);
    let translate_center = translation(Complex::new(0.0, -radius)).unwrap();
    let in_view = tile.transform(translate_center * rot90);

    let style = Style::stroke(255, 255, 255).with_width(0.125);
    render_views(
        "output",
        "bracket_test",
        &[View("", 0.0, 0.0, radius)],
        style_geometry(style, &in_view),
    )?;

    Ok(())
}

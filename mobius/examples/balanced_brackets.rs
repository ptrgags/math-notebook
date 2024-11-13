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

    pub fn iter(&self) -> impl Iterator<Item = &Bracket> {
        self.brackets.iter()
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let north_brackets = BalancedBrackets::new("[][[]][][][[[]][]]")?;
    let mut stack: Vec<usize> = Vec::new();
    let mut arcs: Vec<ClineArc> = Vec::new();
    north_brackets.iter().enumerate().for_each(|(i, &b)| {
        if b == Bracket::Left {
            stack.push(i);
        } else {
            let a = stack.pop().unwrap();
            let b = i;

            let arc = integer_arc_by_hemisphere(a as i64, b as i64, Hemisphere::North).unwrap();
            arcs.push(arc.into());
        }
    });

    let south_brackets = BalancedBrackets::new("[[][][]][[[]][]][]")?;
    let mut stack: Vec<usize> = Vec::new();
    south_brackets.iter().enumerate().for_each(|(i, &b)| {
        if b == Bracket::Left {
            stack.push(i);
        } else {
            let a = stack.pop().unwrap();
            let b = i;

            let arc = integer_arc_by_hemisphere(a as i64, b as i64, Hemisphere::South).unwrap();
            arcs.push(arc.into());
        }
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

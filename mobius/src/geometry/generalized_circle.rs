use std::fmt::Display;

use super::{Circle, Line};

/// A Generalized Circle is either a circle with finite radius, or
/// an infinite circle through infinity (a.k.a. a line)
#[derive(PartialEq, Debug)]
pub enum GeneralizedCircle {
    Circle(Circle),
    Line(Line),
}

impl Display for GeneralizedCircle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeneralizedCircle::Circle(circle) => circle.fmt(f),
            GeneralizedCircle::Line(line) => line.fmt(f),
        }
    }
}

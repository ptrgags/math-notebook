use derive_more::derive::Display;

use super::point::Point;

#[derive(Display)]
#[display("Line")]
pub struct Line {}

impl Line {
    pub fn new(x: f64, y: f64, d: f64) -> Self {
        todo!();
    }

    pub fn meet(&self, line: &Line) -> Point {
        todo!();
    }
}

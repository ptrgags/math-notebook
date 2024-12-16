use derive_more::derive::Display;

use crate::cga_internals::bivector::Bivector;

#[derive(Debug, Display, PartialEq)]
#[display("Point")]
pub struct Point {}

impl Point {
    pub const fn origin() -> Self {
        todo!();
    }
}

impl From<Bivector> for Point {
    fn from(value: Bivector) -> Self {
        todo!()
    }
}

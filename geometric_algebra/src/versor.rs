use crate::{
    bivector::Bivector, pseudoscalar::Pseudoscalar, quadvector::Quadvector, trivector::Trivector,
    vector::Vector,
};

pub struct OddVersor {
    vec: Option<Vector>,
    trivec: Option<Trivector>,
    pseudoscalar: Option<Pseudoscalar>,
}

pub struct EvenVersor {
    scalar: Option<Vector>,
    bivec: Option<Bivector>,
    quadvec: Option<Quadvector>,
}

pub enum Versor {
    Odd(OddVersor),
    Even(EvenVersor),
}

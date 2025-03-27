use crate::{vector::Vector, versor::Versor};

pub struct Xform {}

impl Xform {
    pub fn identity() -> Versor {
        Versor::identity()
    }

    pub fn reflect_x() -> Versor {
        Versor::from(Vector::x())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn reflect_x_is_an_involution() {
        let refl = Xform::reflect_x();

        let result = refl * refl;

        let expected = Xform::identity();
        assert_eq!(result, expected);
    }
}

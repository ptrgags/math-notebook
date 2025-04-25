use crate::{isogonal::Isogonal, point_reflection};

pub fn reflect_y() -> Isogonal {
    Isogonal::AntiConformal(point_reflection())
}

#[cfg(test)]
mod test {
    use abstraction::monoid::Monoid;

    use crate::Complex;

    use super::*;

    #[test]
    pub fn reflect_y_is_involution() {
        let reflect = reflect_y();

        let result = reflect * reflect;

        let expected = Isogonal::identity();
        assert_eq!(result, expected);
    }

    #[test]
    pub fn reflect_y_mirrors_point() {
        let reflect = reflect_y();
        let point = Complex::new(1.0, 2.0);

        let result = reflect * point;

        let expected = Complex::new(-1.0, 2.0);
        assert_eq!(result, expected);
    }
}

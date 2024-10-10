use crate::{Complex, Mobius};

// Complex inversion nu(z) = 1/z, implemented as
// (0z + i) / (iz + 0) to have determinant 1
pub fn inversion() -> Mobius {
    Mobius::new(
        Complex::Zero,
        Complex::I,
        Complex::I,
        Complex::Zero
    ).unwrap()
}

pub fn translation(displacement: Complex) -> Result<Mobius, String> {
    match displacement {
        Complex::Infinity => Err(String::from("displacement must be finite")),
        d => Mobius::new(
            Complex::ONE, d, Complex::Zero,  Complex::ONE
        )
    }
}

pub fn rotation(theta: f64) -> Result<Mobius, String> {
    if !theta.is_finite() {
        return Err(String::from("theta must be finite"))
    }

    let rotor = Complex::from_polar(1.0, 0.5 * theta);
    Mobius::new(
        rotor,
        Complex::Zero,
        Complex::Zero,
        rotor.inverse()
    )
}

pub fn scale(k: f64) -> Result<Mobius, String> {
    if k == 0.0 || !k.is_finite() {
        return Err(String::from("k must be finite and nonzero"))
    }

    let sqrt_k = k.sqrt();
    let inv_sqrt_k = 1.0 / sqrt_k;

    Mobius::new(
        Complex::new(sqrt_k, 0.0),
        Complex::Zero,
        Complex::Zero,
        Complex::new(inv_sqrt_k, 0.0)
    )
}

mod test {
    
    use core::f64;

    use crate::mobius::MobiusType;
    use super::*;


    #[test]
    pub fn inversion_has_determinant_one() {
        let result = inversion().det();

        assert_eq!(result, Complex::ONE);
    }

    #[test]
    pub fn inversion_is_an_ellptic_transform() {
        let invert = inversion();

        let xform_type = invert.classify();

        assert_eq!(xform_type, MobiusType::Elliptic)
    }

    #[test]
    pub fn inversion_is_an_involution() {
        let invert = inversion();
        let inv_sqr = invert * invert;

        assert_eq!(inv_sqr, Mobius::IDENTITY)
    }

    #[test]
    pub fn translation_has_determinant_one() {
        let offset = Complex::new(3.0, 4.0);
        let translate = translation(offset).unwrap();

        let result = translate.det();

        assert_eq!(result, Complex::ONE);
    }

    #[test]
    pub fn translate_is_a_parabolic_transform() -> Result<(), String> {
        let translate = translation(Complex::new(1.0, 2.0))?;

        let xform_type = translate.classify();

        assert_eq!(xform_type, MobiusType::Parabolic);
        Ok(())
    }

    #[test]
    pub fn rotation_has_determinant_one() {
        let theta = f64::consts::FRAC_PI_6;
        let rotate = rotation(theta).unwrap();

        let result = rotate.det();

        assert_eq!(result, Complex::ONE);
    }

    #[test]
    pub fn rotation_is_an_elliptic_transform() -> Result<(), String> {
        let theta = f64::consts::FRAC_PI_4;
        let rotate = rotation(theta)?;

        let xform_type = rotate.classify();

        assert_eq!(xform_type, MobiusType::Elliptic);
        Ok(())
    }

    #[test]
    pub fn rotation_of_zero_is_identity() {
        let rot_zero = rotation(0.0).unwrap();
        
        assert_eq!(rot_zero, Mobius::IDENTITY);
    }

    #[test]
    pub fn scale_is_a_hyperbolic_transform() -> Result<(), String> {
        let theta = f64::consts::FRAC_PI_4;
        let scale = scale(theta)?;

        let xform_type = scale.classify();

        assert_eq!(xform_type, MobiusType::Hyperbolic);
        Ok(())
    }

    
}
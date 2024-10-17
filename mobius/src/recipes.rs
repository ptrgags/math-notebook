use crate::{nearly::is_nearly, Complex, Mobius};

// Complex inversion nu(z) = 1/z, implemented as
// (0z + i) / (iz + 0) to have determinant 1
pub fn inversion() -> Mobius {
    Mobius::new(Complex::Zero, Complex::I, Complex::I, Complex::Zero).unwrap()
}

pub fn translation(displacement: Complex) -> Result<Mobius, String> {
    match displacement {
        Complex::Infinity => Err(String::from("displacement must be finite")),
        d => Mobius::new(Complex::ONE, d, Complex::Zero, Complex::ONE),
    }
}

pub fn rotation(theta: f64) -> Result<Mobius, String> {
    if !theta.is_finite() {
        return Err(String::from("theta must be finite"));
    }

    let rotor = Complex::from_polar(1.0, 0.5 * theta);
    Mobius::new(rotor, Complex::Zero, Complex::Zero, rotor.inverse())
}

pub fn scale(k: f64) -> Result<Mobius, String> {
    if k == 0.0 || !k.is_finite() {
        return Err(String::from("k must be finite and nonzero"));
    }

    let sqrt_k = k.sqrt();
    let inv_sqrt_k = 1.0 / sqrt_k;

    Mobius::new(
        Complex::new(sqrt_k, 0.0),
        Complex::Zero,
        Complex::Zero,
        Complex::new(inv_sqrt_k, 0.0),
    )
}

/// Compute a Mobius transform that fixes the upper half plane.
/// It also separately fixes the extended real line, as it does
/// the lower half plane.
///
/// This is simply the group of mobius transformations with real
/// parameters.
pub fn upper_half_plane(a: f64, b: f64, c: f64, d: f64) -> Result<Mobius, String> {
    Mobius::new(a.into(), b.into(), c.into(), d.into())
}

/// The Cayley map K(z) = (z - i) / (z + i) is a 3-fold rotation
/// of the Riemann sphere with axis through (+1, +1, +1). It permutes
/// the corners of the Riemann sphere (0 -1 i)(inf 1 -i)
///
/// This implementation normalizes it to have determinant 1
pub fn cayley_map() -> Mobius {
    // This transform is expressed as
    //
    // [1 -i]
    // [1  i]
    //
    // but this has determinant i - (-i) = 2i
    //
    // so we must divide by sqrt(2i).
    // This has angle pi/4 and radius sqrt(2)
    // which is sqrt(2) (sqrt(2) / 2 + i * sqrt(2) / 2)
    // which is (1 + i);
    let divisor = Complex::new(1.0, 1.0);

    Mobius::new(
        Complex::ONE / divisor,
        -Complex::I / divisor,
        Complex::ONE / divisor,
        Complex::I / divisor,
    )
    .unwrap()
}

/// Create a map that preserves the unit circle
pub fn unit_circle_map(u: Complex, v: Complex) -> Result<Mobius, String> {
    let norm = u.norm() - v.norm();
    if !is_nearly(norm, 1.0) {
        return Err(String::from("norm(u) - norm(v) must equal 1"));
    }

    Mobius::new(u, v, u.conj(), v.conj())
}

pub fn special_stretch_map(u: f64) -> Result<Mobius, String> {
    if u <= 1.0 {
        return Err(String::from("u must be greater than 1.0"));
    }
    let v = (u * u - 1.0).sqrt();

    unit_circle_map(u.into(), v.into())
}

#[cfg(test)]
mod test {

    use core::f64;

    use test_case::test_case;

    use crate::{mobius::MobiusType, nearly::is_nearly};

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

    #[test]
    pub fn upper_half_plane_fixes_upper_half_plane() -> Result<(), String> {
        let upper_point = Complex::new(-2.0, 3.0);
        let xform = upper_half_plane(2.0, 0.0, 0.0, 0.5)?;

        let result = xform * upper_point;

        assert!(result.imag() > 0.0);
        Ok(())
    }

    #[test]
    pub fn upper_half_plane_fixes_real_line() -> Result<(), String> {
        let real_point: Complex = (15.0).into();
        let xform = upper_half_plane(2.0, 0.0, 0.0, 0.5)?;

        let result = xform * real_point;

        assert!(is_nearly(result.imag(), 0.0));
        Ok(())
    }

    #[test]
    pub fn upper_half_plane_fixes_lower_half_plane() -> Result<(), String> {
        let upper_point = Complex::new(-5.0, -1.5);
        let xform = upper_half_plane(2.0, 0.0, 0.0, 0.5)?;

        let result = xform * upper_point;

        assert!(result.imag() < 0.0);
        Ok(())
    }

    #[test]
    pub fn cayley_map_has_det_one() {
        let k = cayley_map();

        let result = k.det();

        assert_eq!(result, Complex::ONE);
    }

    #[test]
    pub fn cayley_map_has_order_3() {
        let k = cayley_map();

        let k_cubed = k * k * k;

        assert_eq!(k_cubed, Mobius::IDENTITY);
    }

    #[test]
    pub fn cayley_map_permutes_sphere_corners() {
        let zero = Complex::Zero;
        let inf = Complex::Infinity;
        let k = cayley_map();
        let k_sqr = k * k;

        let k_zero = k * zero;
        let k2_zero = k_sqr * zero;
        let k_inf = k * inf;
        let k2_inf = k_sqr * inf;

        // Expected permutation is
        // (0 -1 i)(inf 1 -i)
        assert_eq!(k_zero, -Complex::ONE);
        assert_eq!(k2_zero, Complex::I);
        assert_eq!(k_inf, Complex::ONE);
        assert_eq!(k2_inf, -Complex::I);
    }

    #[test]
    pub fn cayley_map_maps_real_line_to_unit_circle() {
        let real_point: Complex = (45.0).into();
        let k = cayley_map();

        let result = k * real_point;

        assert!(is_nearly(result.mag(), 1.0));
    }

    #[test_case(Complex::new(3.0, 4.0); "real = positive, outside unit circle")]
    #[test_case(Complex::new(0.25, 0.5); "real = positive, inside unit circle")]
    #[test_case(Complex::new(-3.0, 4.0); "real = negative, outside unit circle")]
    #[test_case(Complex::new(-0.25, 0.5); "real = negative, inside unit circle")]
    pub fn cayley_map_maps_upper_half_plane_to_unit_disk(z: Complex) {
        let k = cayley_map();

        let result = k * z;

        assert!(result.mag() < 1.0);
    }
}

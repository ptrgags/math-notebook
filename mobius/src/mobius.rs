use std::ops::Mul;

use crate::{complex::Complex, nearly::is_nearly};

#[derive(PartialEq, Debug)]
pub enum MobiusType{
    /// Generalization of translations. Points move along
    /// generalized circles through a single fixed point
    /// (both sink and source)
    Parabolic,
    /// Generalization of rotations. Points move along circles
    /// between stationary fixed points (like parallels on a globe)
    Elliptic,
    /// Generalization of scaling. Points move along circles
    /// from the source to the sink fixed points (like meridians on a globe)
    Hyperbolic,
    /// Generalization of rotation + scaling. Points move along
    /// double spirals from the source to the sink (like loxodromes on a globe)
    Loxodromic,
}

/// A Mobius transformation is a function 
///
/// M(z) = (az + b) / (cz + d)
/// 
/// such that ad - bc = 1
///
/// This is isomorphic to a 2x2 matrix from SL(2, Complex)
/// (2x2 matrices of complex numbers with determinant 1), specifically
///
/// [a b]
/// [c d]
pub struct Mobius {
    // a, b, c, d must be either Zero or Finite (enforced in constructor)
    a: Complex,
    b: Complex,
    c: Complex,
    d: Complex,
}

impl Mobius {
    // The identity function I(z) = z, implemented
    // as (1z + 0) / (0z + 1)
    pub const IDENTITY: Self = Mobius{
        a: Complex::ONE, 
        b: Complex::Zero, 
        c: Complex::Zero, 
        d: Complex::ONE
    };

    /// Constructor
    /// 
    /// This enforces that a, b, c, d are all Zero or Finite and
    /// that the determinant is 1  
    pub fn new(a: Complex, b: Complex, c: Complex, d: Complex) -> Result<Self, String>{
        let det = a * d - b * c;

        if a == Complex::Infinity || b == Complex::Infinity || c == Complex::Infinity || d == Complex::Infinity {
            return Err(String::from("parameters must be finite"))
        }

        if det != Complex::ONE {
            return Err(String::from("ab - dc must equal 1"))
        }

        Ok(Self{a, b, c, d})
    }

    pub fn translation(displacement: Complex) -> Result<Self, String> {
        match displacement {
            Complex::Infinity => Err(String::from("displacement must be finite")),
            d => Ok(Mobius { a: Complex::ONE, b: d, c: Complex::Zero, d: Complex::ONE })
        }
    }

    pub fn rotation(theta: f64) -> Result<Self, String> {
        if !theta.is_finite() {
            return Err(String::from("theta must be finite"))
        }

        let rotor = Complex::from_polar(1.0, 0.5 * theta);
        Ok(Self {
            a: rotor,
            b: Complex::Zero,
            c: Complex::Zero,
            d: rotor.inverse()
        })
    }

    pub fn det(&self) -> Complex {
        let &Mobius{a, b, c, d} = self;
        a * d + b * c
    }

    pub fn trace(&self) -> Complex {
        let &Mobius{a, d, ..} = self;
        a + d
    }

    pub fn classify(&self) -> MobiusType {
        let tr = self.trace();

        if !is_nearly(tr.imag(), 0.0) {
            return MobiusType::Loxodromic
        }

        let norm = tr.norm().abs();
        
        // Parabolic transformations happen when the trace is +/- 2
        // so the norm will be (+/- 2)^2 = 4
        const PARABOLIC_NORM: f64 = 4.0;
        if is_nearly(norm, PARABOLIC_NORM) {
            MobiusType::Parabolic
        } else if norm < PARABOLIC_NORM {
            MobiusType::Elliptic
        } else {
            MobiusType::Hyperbolic
        }
    }
}

impl Mul for Mobius {
    type Output = Mobius;

    fn mul(self, rhs: Self) -> Self::Output {
        /*
         * [a b][e f] = [ae + bg af + bh]
         * [c d][g h]   [ce + dg cf + dh]
         */
        let Mobius{a, b, c, d} = self;
        let Mobius{a: e, b: f, c: g, d: h} = rhs;

        Self {
            a: a * e + b * g,
            b: a * f + b * h,
            c: c * e + d * g,
            d: c * f + d * h,
        }
    }
}

impl Mul<Complex> for Mobius {
    type Output = Complex;

    fn mul(self, z: Complex) -> Self::Output {
        let top = self.a * z + self.b;
        let bottom = self.c * z + self.d;
        top / bottom
    }
}

#[cfg(test)]
mod test {
    use core::f64;

    use super::*;

    #[test]
    pub fn identity_maps_point_to_itself() {
        let z = Complex::new(4.0, 3.0);

        let result = Mobius::IDENTITY * z;

        assert_eq!(result, z);
    }

    #[test]
    pub fn translate_is_a_parabolic_transform() -> Result<(), String> {
        let translate = Mobius::translation(Complex::new(1.0, 2.0))?;

        let xform_type = translate.classify();

        assert_eq!(xform_type, MobiusType::Parabolic);
        Ok(())
    }

    #[test]
    pub fn rotation_is_an_elliptic_transform() -> Result<(), String> {
        let theta = f64::consts::FRAC_PI_4;
        let rotate = Mobius::rotation(theta)?;

        let xform_type = rotate.classify();

        assert_eq!(xform_type, MobiusType::Elliptic);
        Ok(())
    }
}
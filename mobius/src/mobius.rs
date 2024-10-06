use std::ops::Mul;

use crate::{complex::Complex, nearly::is_nearly};

#[derive(PartialEq, Debug)]
pub enum MobiusType {
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

pub enum FixedPoints {
    Single(Complex),
    Pair(Complex, Complex),
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
#[derive(Clone, Copy)]
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

    // Complex inversion nu(z) = 1/z, implemented as
    // (0z + i) / (iz + 0) to have determinant 1
    pub const INVERSION: Self = Mobius {
        a: Complex::Zero,
        b: Complex::I,
        c: Complex::I,
        d: Complex::Zero,
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
        a * d - b * c
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

    /// Since we assume det 1, the inverse transformation
    /// is a simplified matrix inverse
    /// 
    /// [a b]^(-1) = [ d -b]
    /// [c d]        [-c  a]
    pub fn inverse(&self) -> Self {
        let &Self{a, b, c, d} = self;

        Self{a: d, b: -b, c: -c, d: a}
    }

    pub fn apply(&self, z: Complex) -> Complex {
        let &Mobius{a, b, c, d} = self;

        match z {
            Complex::Zero => b / d,
            // if c is zero, then we really have (az + b) / d, so the
            // value will be infinity
            Complex::Infinity if c == Complex::Zero => Complex::Infinity,
            Complex::Infinity => a / c,
            point @ Complex::Finite(_, _) => (a * point + b) / (c * point + d)
        }
    }

    pub fn fixed_points(&self) -> FixedPoints {
        let &Self{a, b, c, d} = self;

        // When c is 0, the equation reduces to 
        //
        // (a/d)z + b/d = z.
        //
        // also d != 0 because we constrain parameters so ad - bc = 1
        //
        // this always has one fixed point z = inf.
        // Let's solve for the other one
        //
        // az + b = dz
        // (a - d)z + b = 0
        // z = -b / (a - d)
        //
        // Note that when a = d, we get z = inf a second time.
        // This corresponds to a parabolic transformation,
        // specifically
        // z + b/d which is a basic translation!
        if c == Complex::Zero && a == d {
            return FixedPoints::Single(Complex::Infinity)
        }

        if c == Complex::Zero {
            let k = a / d;
            let fixed_point = -b / (a - d);

            return FixedPoints::Pair(fixed_point, Complex::Infinity)
        }
        
        let trace = self.trace();
        let discriminant = trace * trace - Complex::Finite(4.0, 0.0);
        let denominator = Complex::Finite(2.0, 0.0) * c;
        let midpoint = (a - d) / denominator;
        
        if discriminant == Complex::Zero {
            FixedPoints::Single(midpoint)
        } else {
            let offset = discriminant.sqrt() / denominator;
            FixedPoints::Pair(midpoint - offset, midpoint + offset)
        }
    }
    
    /// The "difference" between left and right transformations.
    /// This is kind of like a "ratio" of the two transformations
    /// left * right^-1.
    pub fn difference(left: Self, right: Self) -> Self {
        left * right.inverse()
    }

    /// The sandwich product
    /// a 🥪 b = aba^(-1)
    /// also known as "conjugation". The resulting transformation
    /// does the same thing as b, but adjusted from the perspective of
    /// applying a
    pub fn sandwich(bread: Self, filling: Self) -> Self {
        bread * filling * bread.inverse()
    }

    /// Commutator product
    /// [a, b] = aba^(-1)b^(-1)
    /// which is equal to difference(ab, ba)
    pub fn commutator(left: Self, right: Self) -> Self {
        left * right * left.inverse() * right.inverse()
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

    use test_case::test_case;

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

    #[test]
    pub fn inversion_is_an_ellptic_transform() {
        let inversion = Mobius::INVERSION;

        let xform_type = inversion.classify();

        assert_eq!(xform_type, MobiusType::Elliptic)
    }

    #[test_case(Mobius::IDENTITY; "identity")]
    #[test_case(Mobius::INVERSION; "complex inversion")]
    #[test_case(Mobius::translation(Complex::new(3.0, 4.0)).unwrap(); "translation")]
    #[test_case(Mobius::rotation(f64::consts::FRAC_PI_6).unwrap(); "rotation")]
    pub fn specialized_constructors_have_determinant_one(mobius: Mobius) {
        let result = mobius.det();

        assert_eq!(result, Complex::ONE)
    }
}
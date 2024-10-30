use std::{fmt::Display, ops::Mul};

use abstraction::{Group, Semigroup};

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

#[derive(Debug)]
pub enum FixedPoints {
    Single(Complex),
    Pair(Complex, Complex),
}

impl Display for FixedPoints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single(z) => write!(f, "Single({})", z),
            Self::Pair(a, b) => write!(f, "Pair({}, {})", a, b),
        }
    }
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
#[derive(Clone, Copy, Debug)]
pub struct Mobius {
    // a, b, c, d must be either Zero or Finite (enforced in constructor)
    pub a: Complex,
    pub b: Complex,
    pub c: Complex,
    pub d: Complex,
}

impl Mobius {
    /// Constructor
    ///
    /// This enforces that a, b, c, d are all Zero or Finite and
    /// that the determinant is 1  
    pub fn new(a: Complex, b: Complex, c: Complex, d: Complex) -> Result<Self, String> {
        if a == Complex::Infinity
            || b == Complex::Infinity
            || c == Complex::Infinity
            || d == Complex::Infinity
        {
            return Err(String::from("parameters must be finite"));
        }

        let det = a * d - b * c;
        if det != Complex::ONE {
            return Err(String::from("ab - dc must equal 1"));
        }

        Ok(Self { a, b, c, d })
    }

    pub fn from_unnormalized(
        a: Complex,
        b: Complex,
        c: Complex,
        d: Complex,
    ) -> Result<Self, String> {
        if a == Complex::Infinity
            || b == Complex::Infinity
            || c == Complex::Infinity
            || d == Complex::Infinity
        {
            return Err(String::from("parameters must be finite"));
        }

        let det = a * d - b * c;
        let normalize = det.sqrt().inverse();
        if det == Complex::ONE {
            Ok(Self { a, b, c, d })
        } else {
            Ok(Self {
                a: normalize * a,
                b: normalize * b,
                c: normalize * c,
                d: normalize * d,
            })
        }
    }

    /// Compute the determinant, ad - bc
    pub fn det(&self) -> Complex {
        let &Mobius { a, b, c, d } = self;
        a * d - b * c
    }

    /// Compute the trace, a + d
    pub fn trace(&self) -> Complex {
        let &Mobius { a, d, .. } = self;
        a + d
    }

    /// Classify the Mobius transformation as
    /// parabolic, elliptic, hyperbolic, or loxodromic
    /// depending on the trace
    pub fn classify(&self) -> MobiusType {
        let tr = self.trace();

        if !is_nearly(tr.imag(), 0.0) {
            return MobiusType::Loxodromic;
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
        let &Self { a, b, c, d } = self;

        Self {
            a: d,
            b: -b,
            c: -c,
            d: a,
        }
    }

    /// Compute the distance
    /// |M(z) - M(w)| = |z - w| / (|cz + d||cw + d|)
    pub fn distance(&self, z: Complex, w: Complex) -> f64 {
        let &Self { c, d, .. } = self;
        let numerator = (z - w).mag();
        let denominator_z = (c * z + d).mag();
        let denominator_w = (c * w + d).mag();
        let denominator = denominator_z * denominator_w;

        numerator / denominator
    }

    pub fn fixed_points(&self) -> FixedPoints {
        let &Self { a, b, c, d } = self;

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
            return FixedPoints::Single(Complex::Infinity);
        }

        if c == Complex::Zero {
            let fixed_point = -b / (a - d);
            return FixedPoints::Pair(fixed_point, Complex::Infinity);
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

    // TODO: solve for the parameter k (loxodromic, elliptic, hyperbolic)
    // or the displacement d (parabolic)
    // this involves:
    // 1. Finding the fixed points
    // 2. Compute a transform S such that S(P) = inf
    // 3. If there was only 1 fixed point, S 🥪 T = translation, so just extract
    //      the translation amount
    // 4. Otherwise, recompute S so that S(Q) = 0

    /// Take the complex conjugate of each entry. This is used for
    /// anticonformal mappings, see isogonal.rs
    pub fn complex_conjugate(&self) -> Self {
        Self {
            a: self.a.conj(),
            b: self.b.conj(),
            c: self.c.conj(),
            d: self.d.conj(),
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
        let Mobius { a, b, c, d } = self;
        let Mobius {
            a: e,
            b: f,
            c: g,
            d: h,
        } = rhs;

        let new_a = a * e + b * g;
        let new_b = a * f + b * h;
        let new_c = c * e + d * g;
        let new_d = c * f + d * h;

        let det = new_a * new_d - new_b * new_c;
        if det == Complex::ONE {
            Self {
                a: new_a,
                b: new_b,
                c: new_c,
                d: new_d,
            }
        } else {
            let sqrt_det = det.sqrt();
            Self {
                a: new_a / sqrt_det,
                b: new_b / sqrt_det,
                c: new_c / sqrt_det,
                d: new_d / sqrt_det,
            }
        }
    }
}

impl Mul<Complex> for Mobius {
    type Output = Complex;

    fn mul(self, z: Complex) -> Self::Output {
        let Self { a, b, c, d } = self;

        match z {
            Complex::Zero => b / d,
            // if c is zero, then we really have (az + b) / d, so the
            // value will be infinity
            Complex::Infinity if c == Complex::Zero => Complex::Infinity,
            Complex::Infinity => a / c,
            point @ Complex::Finite(_, _) => (a * point + b) / (c * point + d),
        }
    }
}

impl Semigroup for Mobius {
    // The identity function I(z) = z, implemented
    // as (1z + 0) / (0z + 1)
    fn identity() -> Self {
        Self {
            a: Complex::ONE,
            b: Complex::Zero,
            c: Complex::Zero,
            d: Complex::ONE,
        }
    }
}

impl Group for Mobius {
    fn inverse(&self) -> Self {
        self.inverse()
    }
}

impl PartialEq for Mobius {
    fn eq(&self, other: &Self) -> bool {
        // Subtlety that Indra's Pearls doesn't explain!
        //
        // Since a scalar multiple of a mobius transform is the same transformation,
        // Notice that
        //
        // det (kM) = (ka)(kd) - (kb)(kc) = k^2 (ad - bc) = k^2 det M
        //
        // So if k^2 = 1 (k = {-1, 1}), then we don't change the determinant even
        // though we scaled the coefficients.
        //
        // Therefore, our equality function is M1 == M2 || M1 == -M2 in terms of
        // the matrix coefficients.
        (self.a == other.a && self.b == other.b && self.c == other.c && self.d == other.d)
            || (self.a == -other.a
                && self.b == -other.b
                && self.c == -other.c
                && self.d == -other.d)
    }
}

impl Display for Mobius {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let &Mobius { a, b, c, d } = self;
        write!(f, "[{} {}]\n[{} {}]", a, b, c, d)
    }
}

#[cfg(test)]
mod test {
    use abstraction::{test_associativity, test_group, test_identity};

    use super::*;

    #[test]
    pub fn new_returns_error_for_infinite_entry() {
        let result = Mobius::new(
            Complex::Infinity,
            Complex::Zero,
            Complex::Zero,
            Complex::ONE,
        );

        assert!(result.is_err_and(|e| e.contains("must be finite")))
    }

    #[test]
    pub fn new_returns_error_for_unnormalized_input() {
        let result = Mobius::new((2.0).into(), Complex::Zero, Complex::Zero, (1.0).into());

        assert!(result.is_err_and(|e| e.contains("ab - dc must equal 1")))
    }

    #[test]
    pub fn new_returns_ok_for_valid_input() {
        let result = Mobius::new(Complex::ONE, Complex::Zero, Complex::Zero, Complex::ONE);

        assert!(result.is_ok_and(|x| x == Mobius::identity()))
    }

    test_identity!(
        Mobius,
        [
            (
                scale_transform,
                Mobius {
                    a: (2.0).into(),
                    b: Complex::Zero,
                    c: Complex::Zero,
                    d: (0.5).into()
                }
            ),
            (
                translate_transform,
                Mobius {
                    a: Complex::ONE,
                    b: Complex::new(3.0, 4.0),
                    c: Complex::Zero,
                    d: Complex::ONE,
                }
            )
        ]
    );

    test_associativity!(
        Mobius,
        [(
            three_arbitrary_xforms,
            Mobius {
                a: Complex::ONE,
                b: Complex::Zero,
                c: Complex::new(3.0, 4.0),
                d: Complex::ONE,
            },
            Mobius {
                a: (2.0).into(),
                b: Complex::Zero,
                c: Complex::Zero,
                d: (0.5).into(),
            },
            Mobius {
                a: Complex::ONE,
                b: Complex::new(3.0, 4.0),
                c: Complex::Zero,
                d: Complex::ONE,
            }
        )]
    );

    test_group!(
        Mobius,
        [
            (
                parabolic_and_scale,
                Mobius {
                    a: Complex::ONE,
                    b: Complex::Zero,
                    c: Complex::new(3.0, 4.0),
                    d: Complex::ONE,
                },
                Mobius {
                    a: (2.0).into(),
                    b: Complex::Zero,
                    c: Complex::Zero,
                    d: (0.5).into(),
                }
            ),
            (
                inverse_and_translation,
                Mobius {
                    a: Complex::Zero,
                    b: Complex::I,
                    c: Complex::I,
                    d: Complex::Zero,
                },
                Mobius {
                    a: Complex::ONE,
                    b: (5.0).into(),
                    c: Complex::Zero,
                    d: Complex::ONE,
                }
            )
        ]
    );

    #[test]
    pub fn identity_maps_point_to_itself() {
        let z = Complex::new(4.0, 3.0);

        let result = Mobius::identity() * z;

        assert_eq!(result, z);
    }
}

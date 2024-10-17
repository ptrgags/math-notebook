use std::fmt::Display;

use crate::{Complex, Mobius};

// Simpler data structure for representing clines in human-understandable
// format.
#[derive(PartialEq, Debug)]
pub enum GeneralizedCircle {
    Circle { center: Complex, radius: f64 },
    Line { unit_normal: Complex, distance: f64 },
}

impl Display for GeneralizedCircle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeneralizedCircle::Circle { center, radius } => {
                write!(f, "Circle({}, {:.3})", center, radius)
            }
            GeneralizedCircle::Line {
                unit_normal,
                distance,
            } => write!(f, "Line({}, {:.3})", unit_normal, distance),
        }
    }
}

/// Generalized circle/line, sometimes called a "cline"
/// See https://en.wikipedia.org/wiki/Generalised_circle
///
/// It's represented as the implicit equation
/// A z*z.conj() + B z + C z.conj() + D = 0
///
/// And the coefficients can be formed into the matrix:
/// [A B]
/// [C D]
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Cline {
    // The z*z.conj() component. After normalization, this is
    // 1 for circles and 0 for lines
    a: Complex,
    // The z component
    b: Complex,
    // the z.conj() component.
    c: Complex,
    // The 1 component.
    d: Complex,
}

impl Cline {
    pub fn circle(center: Complex, radius: f64) -> Self {
        let d = center.norm() - radius * radius;

        Self {
            a: Complex::ONE,
            b: -center.conj(),
            c: -center,
            d: d.into(),
        }
    }

    pub fn line(normal: Complex, distance: f64) -> Result<Self, String> {
        match normal.normalize() {
            // A line can be expressed as the implicit equation
            // dot(n, z) = d
            // Re(n.conj() * z) = d
            // 1/2 (n.conj() * z + n * z.conj()) = d
            // n.conj() * z + n * z.conj() = 2 d
            //
            // so we have:
            // A = 0
            // B = n.conj()
            // C = n
            // D = -2d
            Some(unit_normal) => Ok(Self {
                a: Complex::Zero,
                b: unit_normal.conj(),
                c: unit_normal,
                d: (-2.0 * distance).into(),
            }),
            None => Err(String::from("normal must not be zero or infinity")),
        }
    }

    pub fn classify(&self) -> GeneralizedCircle {
        let &Cline { a, b, c, d } = self;

        if a == Complex::Zero {
            // Line n.conj() z + n * z.conj() - 2d = 0
            let unit_normal = c;
            let distance = d / (-2.0).into();

            GeneralizedCircle::Line {
                unit_normal,
                distance: distance.real(),
            }
        } else {
            // Circle z * z.conj() -center.conj() * z - center * z.conj() + (center.norm() - r^2) = 0
            let center = -c;

            // D = center.norm() - r^2
            // center.norm() - D = r^2
            let radius = (center.norm() - d.real()).sqrt();

            GeneralizedCircle::Circle { center, radius }
        }
    }

    pub fn transform(xform: Mobius, cline: Cline) -> Self {
        // According to the Wikipedia article, the implicit equation
        // can be written 0 = z^T C conj(z)
        //
        // If the transform is M, we want to apply M^(-1) to z, which gives
        //
        // (M^-1 z)^T C conj(M^-1 z)
        // z^T (M^-T C conj(M^(-1))) conj(z)
        //
        // The inner matrix product (M^-T C conj(M^(-1))) is the transformed
        // cline. Let's expand this for computing it
        //
        //     M^-T   C     conj(M^-1)
        //   [ d -c][A B][ conj(a) -conj(b)]
        //   [-b  a][C D][-conj(c)  conj(d)]

        let Mobius {
            a: ma,
            b: mb,
            c: mc,
            d: md,
        } = xform;
        let Cline {
            a: ca,
            b: cb,
            c: cc,
            d: cd,
        } = cline;

        // First compute the product of left two matrices L = M^-T C
        let l_00 = md * ca - mc * cc;
        let l_01 = md * cb - mc * cd;
        let l_10 = -mb * ca + ma * cc;
        let l_11 = -mb * cb + ma * cd;

        // Now compute L * conj(M^-1)
        let a = l_00 * ma.conj() + l_01 * mc.conj();
        let b = l_00 * -mb.conj() + l_01 * md.conj();

        let c = l_10 * ma.conj() + l_11 * mc.conj();
        let d = l_10 * -mb.conj() + l_11 * md.conj();

        if a != Complex::Zero {
            // For a circle, we want A = 1, so divide everything by A
            // to normalize it.
            Self {
                a: Complex::ONE,
                b: b / a,
                c: c / a,
                d: d / a,
            }
        } else {
            // So we have Bz + C z.conj() + D = 0
            // A line has the equation
            // n.conj() z + n * z.conj() - 2d = 0
            // We want n to be normalized, so divide the whole equation
            // by the magnitude of c
            let length = c.mag().into();
            Self {
                a: Complex::Zero,
                b: b / length,
                c: c / length,
                d: d / length,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn classify_identifies_unit_circle() {
        let unit_circle = Cline::circle(Complex::Zero, 1.0);

        let result = unit_circle.classify();

        assert_eq!(
            result,
            GeneralizedCircle::Circle {
                center: Complex::Zero,
                radius: 1.0
            }
        )
    }

    #[test]
    pub fn classify_identifies_real_axis() {
        let real_axis = Cline::line(Complex::I, 0.0).unwrap();

        let result = real_axis.classify();

        assert_eq!(
            result,
            GeneralizedCircle::Line {
                unit_normal: Complex::I,
                distance: 0.0
            }
        )
    }

    #[test]
    pub fn classify_identifies_imaginary_axis() {
        let imag_axis = Cline::line(Complex::ONE, 0.0).unwrap();

        let result = imag_axis.classify();

        assert_eq!(
            result,
            GeneralizedCircle::Line {
                unit_normal: Complex::ONE,
                distance: 0.0
            }
        )
    }
}

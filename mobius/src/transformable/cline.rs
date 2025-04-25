use std::{error::Error, fmt::Display};

use rendering::{RenderPrimitive, Renderable};

use crate::{
    complex_error::ComplexError,
    geometry::{Circle, GeneralizedCircle, Line},
    isogonal::Isogonal,
    unit_complex::UnitComplex,
    Complex, Mobius,
};

use super::Transformable;

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
    pub fn unit_circle() -> Self {
        Circle::unit_circle().into()
    }

    pub fn real_axis() -> Self {
        Line::real_axis().into()
    }

    pub fn imag_axis() -> Self {
        Line::imag_axis().into()
    }

    pub fn classify(&self) -> Result<GeneralizedCircle, ComplexError> {
        let &Cline { a, b: _, c, d } = self;

        let gen_circle = if a == Complex::Zero {
            // Line n.conj() z + n * z.conj() - 2d = 0
            let unit_normal = UnitComplex::normalize(c)?;
            let distance = d / (-2.0).into();

            GeneralizedCircle::Line(Line {
                unit_normal,
                distance: distance.real(),
            })
        } else {
            // Circle z * z.conj() -center.conj() * z - center * z.conj() + (center.norm() - r^2) = 0
            let center = -c;

            // D = center.norm() - r^2
            // center.norm() - D = r^2
            let radius = (center.norm() - d.real()).sqrt();

            GeneralizedCircle::Circle(Circle { center, radius })
        };

        Ok(gen_circle)
    }

    pub fn complex_conjugate(&self) -> Self {
        // computing the complex conjugate of the matrix is just the transpose!
        //
        // conj(M) = conj([A        B]) = [A B.conj()] = M^T
        //                [B.conj() D]    [B        D]
        //
        // (this is because A and D are real, and B and C are complex conjugates)
        Self {
            a: self.a,
            b: self.c,
            c: self.b,
            d: self.d,
        }
    }

    fn transform(xform: Mobius, cline: Cline) -> Self {
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
        let a = l_00 * md.conj() + l_01 * -mc.conj();
        let b = l_00 * -mb.conj() + l_01 * ma.conj();

        let c = l_10 * md.conj() + l_11 * -mc.conj();
        let d = l_10 * -mb.conj() + l_11 * ma.conj();

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

impl From<Circle> for Cline {
    fn from(value: Circle) -> Self {
        let Circle { center, radius } = value;
        let d = center.norm() - radius * radius;

        Self {
            a: Complex::ONE,
            b: -center.conj(),
            c: -center,
            d: d.into(),
        }
    }
}

impl From<Line> for Cline {
    fn from(value: Line) -> Self {
        let Line {
            unit_normal,
            distance,
        } = value;

        let &n = unit_normal.get();

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
        Self {
            a: Complex::Zero,
            b: n.conj(),
            c: n,
            d: (-2.0 * distance).into(),
        }
    }
}

impl Transformable<Isogonal> for Cline {
    fn transform(&self, xform: Isogonal) -> Self {
        match xform {
            Isogonal::Conformal(mobius) => Self::transform(mobius, *self),
            // If you swap z and z.conj() in the implicit equation for the cline,
            // the only thing that changes is B and C swap (i.e. transpose the matrix).
            // Since B and C are complex conjugates, and A and D are real, the transpose
            // is equal to the complex conjugate of the matrix
            //
            Isogonal::AntiConformal(mobius) => {
                Self::transform(mobius.complex_conjugate(), *self).complex_conjugate()
            }
        }
    }
}

impl Renderable for Cline {
    fn render(&self) -> Result<RenderPrimitive, Box<dyn Error>> {
        let primitive = match self.classify()? {
            GeneralizedCircle::Circle(circle) => circle.render()?,
            GeneralizedCircle::Line(line) => line.render()?,
        };

        Ok(primitive)
    }
}

impl Display for Cline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let &Self { a, b, c, d } = self;
        write!(f, "[{} {}]\n[{} {}]", a, b, c, d)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn classify_identifies_unit_circle() -> Result<(), ComplexError> {
        let unit_circle = Cline::unit_circle();

        let result = unit_circle.classify()?;

        assert_eq!(
            result,
            GeneralizedCircle::Circle(Circle {
                center: Complex::Zero,
                radius: 1.0
            })
        );

        Ok(())
    }

    #[test]
    pub fn classify_identifies_real_axis() -> Result<(), ComplexError> {
        let real_axis = Cline::real_axis();

        let result = real_axis.classify()?;

        assert_eq!(
            result,
            GeneralizedCircle::Line(Line {
                unit_normal: UnitComplex::I,
                distance: 0.0
            })
        );

        Ok(())
    }

    #[test]
    pub fn classify_identifies_imaginary_axis() -> Result<(), ComplexError> {
        let imag_axis = Cline::imag_axis();

        let result = imag_axis.classify()?;

        assert_eq!(
            result,
            GeneralizedCircle::Line(Line {
                unit_normal: UnitComplex::ONE,
                distance: 0.0
            })
        );

        Ok(())
    }
}

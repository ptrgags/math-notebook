use std::{fmt::Display, ops::Mul};

use abstraction::{group::Group, monoid::Monoid, semigroup::Semigroup};

use crate::{Complex, Mobius};

/// An isogonal (angle-preserving but not necessarily orientation preserving)
/// map realized as either a Mobius transformation M or a mirror (complex conjugation)
/// followed by a Mobius transformation, M * conj
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Isogonal {
    /// Mobius transformations are conformal (orientation-preserving)
    Conformal(Mobius),
    /// M * conj. The complex conjugate is an anti-conformal
    /// mappinig, means it preserves angle magnitudes, but flips the
    /// direction.
    AntiConformal(Mobius),
}

impl Isogonal {
    /// Get the transform I * conj(z) = z.conj()
    pub fn conj() -> Self {
        Self::AntiConformal(Mobius::identity())
    }
}

impl From<Mobius> for Isogonal {
    fn from(value: Mobius) -> Self {
        Self::Conformal(value)
    }
}

impl Mul for Isogonal {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Conformal(a), Self::Conformal(b)) => Self::Conformal(a * b),
            // a * (b * conj) = a * b * conj
            (Self::Conformal(a), Self::AntiConformal(b)) => Self::AntiConformal(a * b),
            // (a * conj) * b = a * conj(b) * conj
            (Self::AntiConformal(a), Self::Conformal(b)) => {
                Self::AntiConformal(a * b.complex_conjugate())
            }
            // (a * conj) * (b * conj) = (a * conj(b) * conj * conj) = (a * conj(b))
            (Self::AntiConformal(a), Self::AntiConformal(b)) => {
                Self::Conformal(a * b.complex_conjugate())
            }
        }
    }
}

impl Mul<Complex> for Isogonal {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output {
        match self {
            Self::Conformal(m) => m * rhs,
            // The transformation is m * conj, so conjugate first then
            // apply the matrix
            Self::AntiConformal(m) => m * rhs.conj(),
        }
    }
}

impl Semigroup for Isogonal {}
impl Monoid for Isogonal {
    fn identity() -> Self {
        Self::Conformal(Mobius::identity())
    }
}

impl Group for Isogonal {
    fn inverse(&self) -> Self {
        match self {
            Self::Conformal(m) => Self::Conformal(m.inverse()),
            // (M conj)^-1 = conj^-1 M^-1 = conj(M)^-1 conj
            Self::AntiConformal(m) => Self::AntiConformal(m.complex_conjugate().inverse()),
        }
    }
}

impl Display for Isogonal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Conformal(m) => write!(f, "Conformal{}", m),
            Self::AntiConformal(m) => write!(f, "AntiConformal{}", m),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Complex;
    use abstraction::{test_associativity, test_group, test_identity};

    test_identity!(
        Isogonal,
        [
            (
                conformal_map,
                Isogonal::from(Mobius {
                    a: (2.0).into(),
                    b: Complex::Zero,
                    c: Complex::Zero,
                    d: (0.5).into()
                })
            ),
            (
                anticonformal_map,
                Isogonal::AntiConformal(Mobius {
                    a: Complex::ONE,
                    b: Complex::Zero,
                    c: Complex::new(3.0, 4.0),
                    d: Complex::ONE,
                },)
            )
        ]
    );

    test_associativity!(
        Isogonal,
        [(
            three_arbitrary_xforms,
            Isogonal::Conformal(Mobius {
                a: Complex::ONE,
                b: Complex::Zero,
                c: Complex::new(3.0, 4.0),
                d: Complex::ONE,
            }),
            Isogonal::from(Mobius {
                a: (2.0).into(),
                b: Complex::Zero,
                c: Complex::Zero,
                d: (0.5).into(),
            }),
            Isogonal::AntiConformal(Mobius {
                a: Complex::ONE,
                b: Complex::new(3.0, 4.0),
                c: Complex::Zero,
                d: Complex::ONE,
            })
        )]
    );

    test_group!(
        Isogonal,
        [
            (
                conformal_conformal,
                Mobius {
                    a: Complex::ONE,
                    b: Complex::Zero,
                    c: Complex::new(3.0, 4.0),
                    d: Complex::ONE,
                }
                .into(),
                Mobius {
                    a: (2.0).into(),
                    b: Complex::Zero,
                    c: Complex::Zero,
                    d: (0.5).into(),
                }
                .into()
            ),
            (
                conformal_anticonformal,
                Mobius {
                    a: Complex::ONE,
                    b: Complex::Zero,
                    c: Complex::new(3.0, 4.0),
                    d: Complex::ONE,
                }
                .into(),
                Isogonal::AntiConformal(Mobius {
                    a: (2.0).into(),
                    b: Complex::Zero,
                    c: Complex::Zero,
                    d: (0.5).into(),
                })
            ),
            (
                anticonformal_conformal,
                Isogonal::AntiConformal(Mobius {
                    a: Complex::Zero,
                    b: Complex::I,
                    c: Complex::I,
                    d: Complex::Zero,
                }),
                Mobius {
                    a: Complex::ONE,
                    b: (5.0).into(),
                    c: Complex::Zero,
                    d: Complex::ONE,
                }
                .into()
            ),
            (
                anticonformal_anticonformal,
                Isogonal::AntiConformal(Mobius {
                    a: Complex::Zero,
                    b: Complex::I,
                    c: Complex::I,
                    d: Complex::Zero,
                }),
                Isogonal::AntiConformal(Mobius {
                    a: (2.0).into(),
                    b: Complex::Zero,
                    c: Complex::Zero,
                    d: (0.5).into(),
                })
            )
        ]
    );
}

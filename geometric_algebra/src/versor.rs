use std::ops::Mul;

use crate::{
    bivector::Bivector, pseudoscalar::Pseudoscalar, quadvector::Quadvector, scalar::Scalar,
    trivector::Trivector, vector::Vector,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct EvenVersor {
    scalar: Option<Scalar>,
    bivec: Option<Bivector>,
    quadvec: Option<Quadvector>,
}

impl EvenVersor {
    pub const fn one() -> Self {
        EvenVersor {
            scalar: Some(Scalar::one()),
            bivec: None,
            quadvec: None,
        }
    }

    pub fn reverse(self) -> Self {
        let Self {
            scalar,
            bivec,
            quadvec,
        } = self;

        Self {
            // Scalars are their own reverse
            scalar,
            // Bivectors are negated, as yx = -xy
            bivec: bivec.map(|x| -x),
            // pzyx = -zyxp = -yxzp = xyzp so no change
            quadvec,
        }
    }
}

impl Mul for EvenVersor {
    type Output = EvenVersor;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Mul<OddVersor> for EvenVersor {
    type Output = OddVersor;

    fn mul(self, rhs: OddVersor) -> Self::Output {
        todo!()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct OddVersor {
    vec: Option<Vector>,
    trivec: Option<Trivector>,
    pseudoscalar: Option<Pseudoscalar>,
}

impl OddVersor {
    pub fn reverse(self) -> Self {
        let Self {
            vec,
            trivec,
            pseudoscalar,
        } = self;

        Self {
            // vectors are their own inverse, no change!
            vec,
            // zyx = yxz = -xyz so we need to flip the sign
            trivec: trivec.map(|t| -t),
            // npzyx = pzyxn = -zyxpn = -yxzpn = xyzpn so no change!
            pseudoscalar,
        }
    }
}

impl From<Vector> for OddVersor {
    fn from(value: Vector) -> Self {
        Self {
            vec: Some(value),
            trivec: None,
            pseudoscalar: None,
        }
    }
}

fn maybe_mul<A: Mul<B>, B>(a: Option<A>, b: Option<B>) -> Option<<A as Mul<B>>::Output> {
    match (a, b) {
        (None, None) => None,
        (None, Some(_)) => None,
        (Some(_), None) => None,
        (Some(a), Some(b)) => Some(a * b),
    }
}

impl Mul for OddVersor {
    type Output = EvenVersor;

    fn mul(self, rhs: Self) -> Self::Output {
        let OddVersor {
            vec: v1,
            trivec: t1,
            pseudoscalar: p1,
        } = self;
        let OddVersor {
            vec: v2,
            trivec: t2,
            pseudoscalar: p2,
        } = rhs;

        let (v_dot_v, v_wedge_v) = maybe_mul(v1, v2).unwrap_or_default();
        let (v_dot_t, v_wedge_t) = maybe_mul(v1, t2).unwrap_or_default();
        let vp = maybe_mul(v1, p2).unwrap_or_default();
        let (t_dot_v, t_wedge_v) = maybe_mul(t1, v2).unwrap_or_default();
        let (tt_s, tt_bi, tt_quad) = maybe_mul(t1, t2).unwrap_or_default();
        let tp = maybe_mul(t1, p2).unwrap_or_default();
        let pv = maybe_mul(p1, v2).unwrap_or_default();
        let pt = maybe_mul(p1, t2).unwrap_or_default();
        let pp = maybe_mul(p1, p2).unwrap_or_default();

        let scalar_part = v_dot_v + tt_s + pp;
        let bivec_part = v_wedge_v + v_dot_t + t_dot_v + tt_bi + tp + pt;
        let quadvec_part = v_wedge_t + vp + t_wedge_v + tt_quad + pv;

        EvenVersor {
            scalar: scalar_part.nonzero(),
            bivec: bivec_part.nonzero(),
            quadvec: quadvec_part.nonzero(),
        }
    }
}

impl Mul<EvenVersor> for OddVersor {
    type Output = OddVersor;

    fn mul(self, rhs: EvenVersor) -> Self::Output {
        (rhs.reverse() * self.reverse()).reverse()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Versor {
    Even(EvenVersor),
    Odd(OddVersor),
}

impl Versor {
    pub const fn identity() -> Self {
        Self::Even(EvenVersor::one())
    }
}

impl From<Vector> for Versor {
    fn from(value: Vector) -> Self {
        Self::Odd(OddVersor::from(value))
    }
}

impl Mul for Versor {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        use Versor::*;
        match (self, rhs) {
            (Even(a), Even(b)) => Even(a * b),
            (Even(a), Odd(b)) => Odd(a * b),
            (Odd(a), Even(b)) => Odd(a * b),
            (Odd(a), Odd(b)) => Even(a * b),
        }
    }
}

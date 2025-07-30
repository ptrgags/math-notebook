use std::{collections::HashMap, ops::Add};

use crate::{basis_blade::BasisBlade, field::Field};

#[derive(Debug, Clone)]
pub struct Multivector<F> {
    terms: HashMap<BasisBlade, F>,
}

impl<F: Field> Multivector<F> {
    /// the zero multivector. This is equivalent to the
    /// zero vector, bivector, etc.
    pub fn zero() -> Self {
        Self::from(F::zero())
    }

    /// The multiplicative identity, 1.
    pub fn identity() -> Self {
        Self::from(F::one())
    }
}

impl<F: Field> From<F> for Multivector<F> {
    fn from(value: F) -> Self {
        let mut terms = HashMap::new();
        terms.insert(BasisBlade::scalar(), value);

        Self { terms }
    }
}

impl<F: Field> From<BasisBlade> for Multivector<F> {
    fn from(value: BasisBlade) -> Self {
        let mut terms = HashMap::new();
        terms.insert(value, F::one());

        Self { terms }
    }
}

impl<F: Field> Add for Multivector<F> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut terms = self.terms.clone();

        for (blade, coeff) in rhs.terms.iter() {
            terms
                .entry(*blade)
                .and_modify(|e| *e = *e + *coeff)
                .or_insert(*coeff);
        }

        Self { terms }
    }
}

impl<F: Field> PartialEq for Multivector<F> {
    fn eq(&self, other: &Self) -> bool {
        if self.terms.len() != other.terms.len() {
            return false;
        }

        for (blade, coeff) in self.terms.iter() {
            if other.terms[blade] != *coeff {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use crate::real::Real;

    use super::*;

    #[test]
    pub fn test_zero_is_additive_identity() {
        let x = Multivector::from(Real::from(3.0));
        let zero = Multivector::zero();

        assert_eq!(zero.clone() + x.clone(), x);
        assert_eq!(x.clone() + zero, x);
    }
}

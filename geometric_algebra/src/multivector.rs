use std::collections::HashMap;

use crate::{basis_blade::BasisBlade, field::Field};

pub struct Multivector<F: Field> {
    terms: HashMap<BasisBlade, F>,
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

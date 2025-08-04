use std::{fmt::Display, ops::Mul};

use crate::var_power::VariablePower;

/// A polynomial with a single term and coefficient of 1
/// e.g. x^2y^3
/// Variables are stored sorted by label, then by power
///
/// To create a Monomial, create individual variables, then multiply them
/// together.
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Monomial {
    variables: Vec<VariablePower>,
}

impl Monomial {
    pub fn one() -> Self {
        // any variable raised to the 0 power reduces to 1, here represented
        // as an empty vector
        Self { variables: vec![] }
    }

    pub fn var(label: &str) -> Self {
        Self {
            variables: vec![VariablePower::var(label)],
        }
    }

    pub fn power(label: &str, power: usize) -> Self {
        Self {
            variables: vec![VariablePower::new(label, power)],
        }
    }
}

impl From<VariablePower> for Monomial {
    fn from(value: VariablePower) -> Self {
        let variables = if value == VariablePower::one() {
            vec![]
        } else {
            vec![value]
        };

        Self { variables }
    }
}

fn reduce_exponents(variables: &[VariablePower]) -> Vec<VariablePower> {
    variables
        .chunk_by(|VariablePower(a, _), VariablePower(b, _)| a == b)
        .map(|same_label| {
            // product of x^a, x^b, ... x^k = x^(a + b + ... + k)
            let VariablePower(label, _) = &same_label[0];
            let sum_of_powers: usize = same_label.iter().map(|VariablePower(_, x)| x).sum();
            VariablePower::new(label, sum_of_powers)
        })
        .collect()
}

impl Mul for Monomial {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        // First, gather all the variables into one vector and sort them.
        let mut variables = self.variables;
        variables.extend(rhs.variables);
        variables.sort();

        // Now we need to combine variables with the same label, adding
        // the exponents using the rule x^a * x^b = x^(a + b)
        let reduced = reduce_exponents(&variables);

        Self { variables: reduced }
    }
}

impl Display for Monomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if *self == Self::one() {
            return write!(f, "1");
        }

        let var_strs: Vec<String> = self.variables.iter().map(|x| format!("{}", x)).collect();
        let combined = var_strs.join("");
        write!(f, "{}", combined)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_is_multiplicative_identity() {
        let one = Monomial::one();
        let mono = Monomial::var("x");

        let one_x = one.clone() * mono.clone();
        let x_one = mono.clone() * one.clone();

        assert_eq!(one_x, mono.clone());
        assert_eq!(x_one, mono.clone());
    }

    #[test]
    fn multiplication_is_commutative() {
        let a = Monomial::var("x");
        let b = Monomial::power("y", 3);

        let ab = a.clone() * b.clone();
        let ba = b.clone() * a.clone();

        assert_eq!(ab, ba);
    }

    #[test]
    fn multiplication_combines_exponents() {
        let a = Monomial::power("x", 2);
        let b = Monomial::power("x", 4);

        let ab = a * b;

        let expected = Monomial::power("x", 6);
        assert_eq!(ab, expected);
    }

    #[test]
    fn formats_one_as_1() {
        let one = Monomial::one();

        let result = format!("{}", one);

        assert_eq!(result, "1");
    }

    #[test]
    fn formats_var_without_exponent() {
        let x = Monomial::var("x");

        let result = format!("{}", x);

        assert_eq!(result, "x");
    }

    #[test]
    fn formats_power_with_exponent() {
        let x = Monomial::power("x", 3);

        let result = format!("{}", x);

        assert_eq!(result, "x^3");
    }

    #[test]
    fn formats_product_in_sorted_order() {
        let x = Monomial::power("x", 3);
        let y = Monomial::power("y", 2);
        let product = y * x;

        let result = format!("{}", product);

        assert_eq!(result, "x^3y^2");
    }
}

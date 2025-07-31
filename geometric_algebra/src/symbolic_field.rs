use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::field::Field;

// New grammar
/*

term = (coeff, label, power) (or a product of these with different labels)

constant(k) = (k, "1", 1)
variable(x) = (1, x, 1)

since we need multiple labels

term = HashMap<String, Term>
where the key is a variable label ("1" is reserved for constants)
...no wait, you also need to include the powers, e.g. x^2y is not the same
as xy^2

...also even if there are multiple variables and powers, there's only one
coefficient

a Polynomial is then a hashmap of hashmaps, keyed by the key

...I'm tired. Another day.
 */

// Field of rational functiions F[x_i] for some variables x_i
#[derive(Debug, PartialEq, Clone)]
pub enum SymbolicField {
    Constant(f64),
    /// A variable e.g. "A_x", "d", whatever
    Variable(String),
    Sum(Vec<SymbolicField>),
    Product(Vec<SymbolicField>),
    Div(Box<SymbolicField>, Box<SymbolicField>),
}

impl SymbolicField {
    pub fn var(name: &str) -> Self {
        Self::Variable(name.to_owned())
    }
}

impl Display for SymbolicField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SymbolicField::*;
        match self {
            Variable(x) => write!(f, "{}", x),
            Sum(symbolic_fields) => {
                let terms: Vec<String> = symbolic_fields
                    .iter()
                    .map(|expr| format!("{}", expr))
                    .collect();
                let sum = terms.join(" + ");
                write!(f, "{}", sum)
            }
            Constant(c) => write!(f, "{}", c),
            Product(symbolic_fields) => {
                let factors: Vec<String> = symbolic_fields
                    .iter()
                    .map(|expr| format!("({})", expr))
                    .collect();
                let product = factors.join("");
                write!(f, "{}", product)
            }
            Div(numerator, denominator) => write!(f, "({})/({})", numerator, denominator),
        }
    }
}

impl Add for SymbolicField {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        use SymbolicField::*;
        match (self.clone(), rhs.clone()) {
            (Sum(mut terms_a), Sum(mut terms_b)) => {
                terms_a.append(&mut terms_b);
                Sum(terms_a)
            }
            (Sum(mut terms), _) => {
                terms.push(rhs.clone());
                Sum(terms)
            }
            (_, Sum(mut terms)) => {
                // This puts the loose variable on the right, but this
                // is allowed since addition is commutative
                terms.push(self.clone());
                Sum(terms)
            }
            (Constant(a), Constant(b)) => Constant(a + b),
            _ => Sum(vec![self, rhs]),
        }
    }
}

impl Neg for SymbolicField {
    type Output = Self;

    fn neg(self) -> Self::Output {
        use SymbolicField::*;
        match self {
            Constant(x) => Constant(-x),
            Div(numerator, denominator) => Div(Box::new(numerator.neg()), denominator),
            _ => Product(vec![Constant(-1.0), self]),
        }
    }
}

impl Sub for SymbolicField {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul for SymbolicField {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        use SymbolicField::*;
        match (self.clone(), rhs.clone()) {
            (Constant(a), Constant(b)) => Constant(a + b),
            (Product(mut factors_a), Sum(mut factors_b)) => {
                factors_a.append(&mut factors_b);
                Product(factors_a)
            }
            (Product(mut factors), _) => {
                factors.push(rhs.clone());
                Product(factors)
            }
            (_, Product(mut factors)) => {
                // This puts the loose variable on the right, but this
                // is allowed since addition is commutative
                factors.push(self.clone());
                Product(factors)
            }
            (Div(num1, denom1), Div(num2, denom2)) => {
                Div(Box::new(*num1 * *num2), Box::new(*denom1 * *denom2))
            }
            (Div(num, denom), _) => Div(Box::new(*num * rhs), denom),
            _ => Product(vec![self, rhs]),
        }
    }
}

impl Div for SymbolicField {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Div(Box::new(self), Box::new(rhs))
    }
}

impl Field for SymbolicField {
    fn zero() -> Self {
        Self::Constant(0.0)
    }

    fn one() -> Self {
        Self::Constant(1.0)
    }

    fn inverse(&self) -> Self {
        Self::Div(Box::new(Self::Constant(1.0)), Box::new(self.clone()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn display_with_var_gives_label() {
        let x = SymbolicField::var("A_x");

        let result = format!("{}", x);

        assert_eq!(result, "A_x");
    }

    #[test]
    pub fn display_with_sum_formats_with_plus() {
        let a = SymbolicField::var("a");
        let b = SymbolicField::var("b");

        let sum = a + b;
        let result = format!("{}", sum);

        assert_eq!(result, "a + b");
    }
}

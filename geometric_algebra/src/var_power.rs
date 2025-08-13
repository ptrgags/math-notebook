use std::fmt::Display;

/// A single variable (a string label like "x") raised to an
/// exponent. E.g. VariablePower("x", 3)
#[derive(Debug, Hash, PartialOrd, Ord, Clone)]
pub struct VariablePower(pub String, pub usize);

impl VariablePower {
    pub fn one() -> Self {
        // Use the empty string as label so it comes first when sorted
        Self(String::from(""), 0usize)
    }

    pub fn var(label: &str) -> Self {
        Self(String::from(label), 1usize)
    }

    pub fn new(label: &str, power: usize) -> Self {
        if power == 0 {
            return Self::one();
        }

        Self(String::from(label), power)
    }
}

impl PartialEq for VariablePower {
    fn eq(&self, other: &Self) -> bool {
        let Self(a, x) = self;
        let Self(b, y) = other;

        // x^0 is 1, so the label doesn't matter
        if *x == 0 && *y == 0 {
            return true;
        }

        a == b && x == y
    }
}

impl Eq for VariablePower {}

impl Display for VariablePower {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self(_, 0) => write!(f, "1"),
            Self(label, 1) => write!(f, "{}", label),
            Self(label, x) => write!(f, "{}**{}", label, x),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn with_zero_power_is_one() {
        let one = VariablePower::one();
        let zero_power = VariablePower::new("x", 0);

        assert_eq!(one, zero_power);
    }

    #[test]
    pub fn with_one_formats_as_one() {
        let one = VariablePower::one();

        let result = format!("{}", one);

        assert_eq!(result, "1");
    }

    #[test]
    pub fn formats_zero_power_as_one() {
        let zero_power = VariablePower::new("x", 0);

        let result = format!("{}", zero_power);

        assert_eq!(result, "1");
    }

    #[test]
    pub fn formats_linear_var_without_exponent() {
        let var = VariablePower::var("A_x");

        let result = format!("{}", var);

        assert_eq!(result, "A_x");
    }

    #[test]
    pub fn formats_power_with_asterisks() {
        let power = VariablePower::new("x", 4);

        let result = format!("{}", power);

        assert_eq!(result, "x**4");
    }
}

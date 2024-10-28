use std::{fmt::Display, ops::Mul};

use abstraction::{Group, Semigroup};

/// Fractal adddress symbol, using the "uppercase is inverse" notation
/// a la _Indra's Pearls_.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Symbol {
    // A forward transformation with the given index in the IFS
    Forward(usize),
    // An inverse transformation with the given index in the IFS
    Inverse(usize),
}

impl Symbol {
    pub fn inverse(&self) -> Self {
        match self {
            Self::Forward(i) => Self::Inverse(*i),
            Self::Inverse(i) => Self::Forward(*i),
        }
    }

    pub fn is_inverse_pair(a: Symbol, b: Symbol) -> bool {
        match (a, b) {
            (Symbol::Forward(a), Symbol::Inverse(b)) if a == b => true,
            (Symbol::Inverse(a), Symbol::Forward(b)) if a == b => true,
            _ => false,
        }
    }
}

impl TryFrom<char> for Symbol {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let ord_value = value as isize;
        let ord_a = 'a' as isize;
        let ord_z = 'z' as isize;
        let ord_a_inv = 'A' as isize;
        let ord_z_inv = 'Z' as isize;
        if ord_a <= ord_value && ord_value <= ord_z {
            Ok(Self::Forward((ord_value - ord_a) as usize))
        } else if ord_a_inv <= ord_value && ord_value <= ord_z_inv {
            Ok(Self::Inverse((ord_value - ord_a_inv) as usize))
        } else {
            Err(format!("Invalid fractal address symbol {}", value))
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Forward(i) => {
                let symbol = ('a' as u8 + *i as u8) as char;
                write!(f, "{}", symbol)
            }
            Self::Inverse(i) => {
                let symbol = ('A' as u8 + *i as u8) as char;
                write!(f, "{}", symbol)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct FractalAddress {
    symbols: Vec<Symbol>,
}

impl FractalAddress {
    pub fn new(symbols: Vec<Symbol>) -> Self {
        Self { symbols }
    }

    pub fn len(&self) -> usize {
        self.symbols.len()
    }

    pub fn leftmost(&self) -> Symbol {
        self.symbols[0]
    }

    pub fn rightmost(&self) -> Symbol {
        self.symbols[self.symbols.len() - 1]
    }
}

// Promote a symbol to an address of length 1
impl From<Symbol> for FractalAddress {
    fn from(value: Symbol) -> Self {
        Self {
            symbols: vec![value],
        }
    }
}

impl TryFrom<&str> for FractalAddress {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut symbols = Vec::new();
        for c in value.chars() {
            let symbol: Symbol = c.try_into()?;
            symbols.push(symbol);
        }

        Ok(Self { symbols })
    }
}

impl Mul for FractalAddress {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        // TODO: cancel out symbols at the join
        let mut symbols = Vec::with_capacity(self.symbols.len() + rhs.symbols.len());
        symbols.extend_from_slice(&self.symbols[..]);
        symbols.extend_from_slice(&rhs.symbols[..]);
        Self { symbols }
    }
}

impl PartialEq for FractalAddress {
    fn eq(&self, other: &Self) -> bool {
        if self.symbols.len() != other.symbols.len() {
            return false;
        }

        self.symbols
            .iter()
            .zip(other.symbols.iter())
            .all(|(a, b)| a == b)
    }
}

impl Semigroup for FractalAddress {
    fn identity() -> Self {
        Self { symbols: vec![] }
    }
}

impl Group for FractalAddress {
    fn inverse(&self) -> Self {
        let symbols = self.symbols.iter().rev().map(|x| x.inverse()).collect();
        Self { symbols }
    }
}

impl Display for FractalAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted: Vec<String> = self.symbols.iter().map(|x| x.to_string()).collect();
        write!(f, "{}", formatted.join(""))
    }
}

#[cfg(test)]
mod test {
    use abstraction::{test_associativity, test_identity, test_inverse};

    use super::*;

    test_identity!(
        FractalAddress,
        [
            (single_symbol, FractalAddress::try_from("c").unwrap()),
            (a_few_symbols, FractalAddress::try_from("abb").unwrap())
        ]
    );

    test_associativity!(
        FractalAddress,
        [(
            three_arbitrary_addresses,
            FractalAddress::try_from("aabA").unwrap(),
            FractalAddress::try_from("aBbA").unwrap(),
            FractalAddress::try_from("abAA").unwrap()
        )]
    );

    test_inverse!(
        FractalAddress,
        [
            (single_symbol, FractalAddress::try_from("a").unwrap()),
            (many_symbols, FractalAddress::try_from("aBc").unwrap())
        ]
    );
}

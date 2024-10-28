use std::{fmt::Display, ops::Mul};

use abstraction::Semigroup;

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
    pub fn is_inverse_pair(a: Symbol, b: Symbol) -> bool {
        match (a, b) {
            (Symbol::Forward(a), Symbol::Inverse(b)) if a == b => true,
            (Symbol::Inverse(a), Symbol::Forward(b)) if a == b => true,
            _ => false,
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

#[derive(Clone)]
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

impl Mul for FractalAddress {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let n = self.symbols.len().min(rhs.symbols.len());

        let mut index = 0;
        for i in 0..n {
            if !Symbol::is_inverse_pair(self.symbols[n - 1 - i], rhs.symbols[i]) {
                break;
            }

            index += 1;
        }

        let capacity = self.symbols.len() + rhs.symbols.len() - 2 * index;
        let mut symbols = Vec::with_capacity(capacity);
        symbols.extend_from_slice(&self.symbols[..(n - 1 - index)]);
        symbols.extend_from_slice(&rhs.symbols[index..]);
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

impl Display for FractalAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted: Vec<String> = self.symbols.iter().map(|x| x.to_string()).collect();
        write!(f, "{}", formatted.join(""))
    }
}

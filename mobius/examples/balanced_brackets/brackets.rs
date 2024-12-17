use mobius::geometry::integer_arcs::Hemisphere;

#[derive(Debug, thiserror::Error)]
pub enum BracketError {
    #[error("characters must be [ or ], got {0}")]
    InvalidCharacter(char),
    #[error("unbalanced brackets: {0}")]
    Unbalanced(String),
    #[error("bracket sequences must have the same length: {0}, {0}")]
    UnmachedLengths(usize, usize),
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Bracket {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct BalancedBrackets {
    brackets: Vec<Bracket>,
}

impl BalancedBrackets {
    pub fn new(bracket_string: &str) -> Result<Self, BracketError> {
        let mut depth: i64 = 0;
        let mut brackets = Vec::new();
        for c in bracket_string.chars() {
            match c {
                '[' => {
                    brackets.push(Bracket::Left);
                    depth += 1;
                }
                ']' => {
                    brackets.push(Bracket::Right);
                    depth -= 1;
                }
                x => return Err(BracketError::InvalidCharacter(x)),
            }
            if depth < 0 {
                return Err(BracketError::Unbalanced(String::from(bracket_string)));
            }
        }

        if depth != 0 {
            return Err(BracketError::Unbalanced(String::from(bracket_string)));
        }

        Ok(Self { brackets })
    }

    pub fn len(&self) -> usize {
        self.brackets.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        BracketIterator::new(&self.brackets)
    }
}

struct BracketIterator<'a> {
    brackets: &'a [Bracket],
    index: usize,
    stack: Vec<usize>,
}

impl<'a> BracketIterator<'a> {
    pub fn new(brackets: &'a [Bracket]) -> Self {
        Self {
            brackets,
            index: 0,
            stack: Vec::new(),
        }
    }
}

impl<'a> Iterator for BracketIterator<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.brackets.len() {
            // Increment the index whether we short circuit or not.
            let current = self.index;
            self.index += 1;

            match self.brackets[current] {
                Bracket::Left => self.stack.push(current),
                Bracket::Right => {
                    let a = self.stack.pop().unwrap();
                    let b = current;
                    return Some((a, b));
                }
            }
        }

        None
    }
}

/// A pair of balanced brackets that have the same length. One is for the
/// northern hemisphere, one for the southern one
pub struct MatchedBalancedBrackets(BalancedBrackets, BalancedBrackets);

impl MatchedBalancedBrackets {
    pub fn new(north: BalancedBrackets, south: BalancedBrackets) -> Result<Self, BracketError> {
        if north.len() != south.len() {
            return Err(BracketError::UnmachedLengths(north.len(), south.len()));
        }

        Ok(Self(north, south))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, Hemisphere)> + '_ {
        let Self(north, south) = self;
        let labeled_north = north.iter().map(|(a, b)| (a, b, Hemisphere::North));
        let labeled_south = south.iter().map(|(a, b)| (a, b, Hemisphere::South));
        labeled_north.chain(labeled_south)
    }
}

use std::collections::HashSet;

/// Mathematical permutation of N elements. An element of the symmetric group S_N
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Permutation<const N: usize> {
    values: [usize; N]
}

impl<const N: usize> Permutation<N> {

    pub fn new(values: [usize; N]) -> Result<Self, String> {
        let unique_values: HashSet<usize> = HashSet::from_iter(values.iter().cloned());
        if unique_values.len() < values.len() {
            return Err(String::from("values must not have repeat elements"))
        }

        for value in values {
            if value > values.len() {
                return Err(String::from("values must be less than length of slice"))
            }
        }

        Ok(Self {
            values
        })
    }

    pub fn compose(a: Self, b: Self) -> Self {
        let mut product = [0; N];

        for i in 0..N {
            product[i] = a.values[b.values[i]];
        }

        Self::new(product).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn new_returns_error_for_out_of_range_element() {
        let result = Permutation::new([0, 1, 2, 10]);
        
        assert!(result.is_err_and(|e| e.contains("values must be less than length of slice")));
    }

    #[test]
    pub fn new_returns_error_for_duplicate_element() {
        let result = Permutation::new([0, 2, 2, 1]);
        
        assert!(result.is_err_and(|e| e.contains("values must not have repeat elements")));
    }

    #[test]
    pub fn new_returns_permutation_for_valid_elements() {
        let result = Permutation::new([0, 2, 3, 1]);

        assert!(result.is_ok());
    }

    #[test]
    pub fn compose_applies_a_after_b() {
        let a = Permutation::new([0, 2, 3, 1]).unwrap();
        let b = Permutation::new([0, 1, 3, 2]).unwrap();

        let result = Permutation::compose(a, b);

        // a = (1 2 3)
        // b = (2 3)
        // ab = (1 2 3)(2 3) = (1 2) <-- we want this one
        // ba = (2 3)(1 2 3) = (1 3)
        let expected = Permutation::new([0, 2, 1, 3]).unwrap();
        assert_eq!(result, expected);

    }
}
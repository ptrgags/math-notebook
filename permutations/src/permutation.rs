use std::collections::HashSet;

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Permutation {
    values: Vec<usize>
}

impl Permutation {
    pub fn new(values: &[usize]) -> Result<Self, String> {
        if values.len() == 0 {
            return Err(String::from("values must not be empty"));
        }

        let unique_values: HashSet<usize> = HashSet::from_iter(values.iter().cloned());
        if unique_values.len() < values.len() {
            return Err(String::from("values must not have repeat elements"))
        }

        for value in values {
            if *value > values.len() {
                return Err(String::from("values must be less than length of slice"))
            }
        }

        Ok(Self {
            values: values.to_vec()
        })
    }

    pub fn compose(a: Self, b: Self) -> Result<Self, String> {
        let n = a.values.len();
        if b.values.len() != n {
            return Err(String::from("permutations must have the same length"));
        }

        let mut product = vec![0; a.values.len()];

        for i in 0..n {
            product[i] = a.values[b.values[i]];
        }

        Self::new(&product)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn new_returns_error_for_empty_slice() {
        let result = Permutation::new(&[]);

        assert!(result.is_err_and(|e| e.contains("values must not be empty")));
    }

    #[test]
    pub fn new_returns_error_for_out_of_range_element() {
        let result = Permutation::new(&[0, 1, 2, 10]);
        
        assert!(result.is_err_and(|e| e.contains("values must be less than length of slice")));
    }

    #[test]
    pub fn new_returns_error_for_duplicate_element() {
        let result = Permutation::new(&[0, 2, 2, 1]);
        
        assert!(result.is_err_and(|e| e.contains("values must not have repeat elements")));
    }

    #[test]
    pub fn new_returns_permutation_for_valid_elements() {
        let result = Permutation::new(&[0, 2, 3, 1]);

        assert!(result.is_ok());
    }

    #[test]
    pub fn compose_returns_error_for_mismatched_lengths() {
        let a = Permutation::new(&[0, 1, 2]).unwrap();
        let b = Permutation::new(&[0]).unwrap();

        let result = Permutation::compose(a, b);

        assert!(result.is_err_and(|e| e.contains("permutations must have the same length")))
    }

    #[test]
    pub fn compose_applies_a_after_b() {
        let a = Permutation::new(&[0, 2, 3, 1]).unwrap();
        let b = Permutation::new(&[0, 1, 3, 2]).unwrap();

        let result = Permutation::compose(a, b);

        // a = (1 2 3)
        // b = (2 3)
        // ab = (1 2 3)(2 3) = (1 2) <-- we want this one
        // ba = (2 3)(1 2 3) = (1 3)
        let expected = Permutation::new(&[0, 2, 1, 3]).unwrap();
        assert!(result.is_ok_and(|x| x == expected));

    }
}
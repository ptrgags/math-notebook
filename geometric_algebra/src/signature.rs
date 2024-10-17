pub struct Signature {
    positive: usize,
    negative: usize,
    zero: usize,
}

impl Signature {
    pub fn new(positive: usize, negative: usize, zero: usize) -> Result<Self, String> {
        if positive + negative + zero > 8usize {
            return Err(String::from("Only up to 8 dimensions are supported"));
        }

        Ok(Self {
            positive,
            negative,
            zero,
        })
    }

    pub fn get_dimensions(&self) -> usize {
        return (self.positive + self.negative + self.zero) as usize;
    }

    pub fn get_sign(&self, dimension: usize) -> Result<i8, String> {
        let n = self.get_dimensions();
        if dimension < self.positive {
            Ok(1)
        } else if dimension < self.positive + self.negative {
            Ok(-1)
        } else if dimension < n {
            Ok(0)
        } else {
            Err(format!("dimension must be in [0, {}]", n - 1))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_returns_error_for_too_many_dimensions() {
        let result = Signature::new(4, 4, 4);

        assert!(result.is_err_and(|e| e.contains("Only up to 8 dimensions are supported")));
    }

    #[test]
    fn get_dimensions_computes_total_dimensions() {
        let signature = Signature::new(3, 1, 2).unwrap();

        let result = signature.get_dimensions();

        assert_eq!(result, 6);
    }

    #[test]
    fn get_sign_returns_error_for_out_of_bounds() {
        let signature = Signature::new(3, 2, 1).unwrap();

        let higher_dimension = 10usize;
        let result = signature.get_sign(higher_dimension);

        assert!(result.is_err_and(|e| e.contains("dimension must be in [0, 5]")));
    }

    #[test]
    fn get_sign_returns_one_for_positive_dimension() {
        let signature = Signature::new(3, 2, 1).unwrap();

        let last_positive_dim = 2usize;
        let result = signature.get_sign(last_positive_dim);

        assert!(result.is_ok_and(|s| s == 1))
    }

    #[test]
    fn get_sign_returns_neg_one_for_neg_dimension() {
        let signature = Signature::new(3, 2, 1).unwrap();

        let last_negative_dim = 4usize;
        let result = signature.get_sign(last_negative_dim);

        assert!(result.is_ok_and(|s| s == -1))
    }

    #[test]
    fn get_sign_returns_zero_for_null_dimension() {
        let signature = Signature::new(3, 2, 1).unwrap();

        let zero_dim = 5usize;
        let result = signature.get_sign(zero_dim);

        assert!(result.is_ok_and(|s| s == 0))
    }
}

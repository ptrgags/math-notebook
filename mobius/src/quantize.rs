pub fn quantize(x: f64, n: i32) -> isize {
    (x * (2.0f64.powi(n))).floor() as isize
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn quantize_with_zero_gives_zero() {
        let bits = 4;
        let zero = 0.0;

        let result = quantize(zero, bits);

        let expected = 0isize;
        assert_eq!(result, expected);
    }

    #[test]
    pub fn quantize_with_nearby_values_give_same_result() {
        let bits = 4;
        // 1/16 = 0.0625
        // so let's offset things by something smaller than that;
        let a = 0.5;
        let b = 0.50001;

        let result_a = quantize(a, bits);
        let result_b = quantize(b, bits);

        assert_eq!(result_a, result_b);
    }
}

use crate::nearly::is_nearly;

pub fn format_term(coefficient: f64, base: &str) -> Option<String> {
    if is_nearly(coefficient, 0.0) {
        None
    } else if is_nearly(coefficient, 1.0) {
        Some(String::from(base))
    } else {
        Some(format!("{:.3}{}", coefficient, base))
    }
}

pub fn format_term_list(terms: &[(f64, &str)]) -> String {
    let nonzero_terms: Vec<String> = terms
        .iter()
        .map(|&(coefficient, base)| format_term(coefficient, base))
        .flatten()
        .collect();

    let result = nonzero_terms.join(" + ");

    if result == "" {
        String::from("0")
    } else {
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn format_term_with_zero_returns_none() {
        let result = format_term(0.0, "x");

        assert!(result.is_none())
    }

    #[test]
    pub fn format_term_formats_term() {
        let result = format_term(3.25432, "xy").unwrap();

        let expected = "3.254xy";
        assert_eq!(result, expected);
    }

    #[test]
    pub fn format_term_omits_coeff_of_one() {
        let result = format_term(1.0, "z").unwrap();

        assert_eq!(result, "z");
    }

    #[test]
    pub fn format_term_list_with_empty_slice_returns_zero() {
        let result = format_term_list(&[]);

        assert_eq!(result, "0");
    }

    #[test]
    pub fn format_term_list_with_nonzero_terms_formats_as_linear_combo() {
        let result = format_term_list(&[(-1.0, ""), (2.0, "xy"), (3.0, "xyz")]);

        assert_eq!(result, "-1.000 + 2.000xy + 3.000xyz");
    }

    #[test]
    pub fn format_term_list_with_coefficient_zero_omits_term() {
        let result = format_term_list(&[(-1.0, ""), (0.0, "xy"), (3.0, "xyz")]);

        assert_eq!(result, "-1.000 + 3.000xyz");
    }

    #[test]
    pub fn format_term_list_with_coefficient_one_abbreviates_term() {
        let result = format_term_list(&[(-1.0, ""), (1.0, "xy"), (3.0, "xyz")]);

        assert_eq!(result, "-1.000 + xy + 3.000xyz");
    }
}

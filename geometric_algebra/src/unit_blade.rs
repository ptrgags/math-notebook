#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct UnitBlade {
    vectors: u8,
}

impl UnitBlade {
    pub fn new(vectors: u8) -> Self {
        Self { vectors }
    }

    pub fn pretty(&self, labels: &[&str]) -> String {
        let components: Vec<&str> = (0..8)
            .map(|i| {
                if self.vectors >> i & 1 == 1 {
                    labels[i]
                } else {
                    ""
                }
            })
            .collect();
        components.join("")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pretty_labels_vector() {
        let x_hat = UnitBlade::new(0b1);

        let result = x_hat.pretty(&["x", "y", "z"]);

        assert_eq!(result, "x");
    }

    #[test]
    fn pretty_labels_bivector() {
        let x_hat = UnitBlade::new(0b101);

        let result = x_hat.pretty(&["x", "y", "z"]);

        assert_eq!(result, "xz");
    }
}

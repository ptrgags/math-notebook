mod signature;
mod unit_blade;

pub use signature::Signature;
use unit_blade::UnitBlade;

fn choose_bits(n: usize, choices: &[u8]) -> Vec<u8> {
    if n == 0 || choices.len() == 0 {
        return vec![0];
    }

    let mut result = Vec::new();
    for i in 0..choices.len() {
        let choice = choices[i];
        let rest = &choices[i + 1..];
        let partial_results: Vec<u8> = choose_bits(n - 1, rest)
            .iter()
            .map(|x| choice | x)
            .collect();
        result.extend_from_slice(&partial_results);
    }

    result
}

fn bit_permutations(dimensions: usize) -> Vec<u8> {
    let basis_vectors: Vec<u8> = (0..dimensions).map(|i| 1 << i as u8).collect();

    let mut result = Vec::new();
    for i in 0..=dimensions {
        result.extend_from_slice(&choose_bits(i, &basis_vectors));
    }

    result
}

pub fn make_blades(signature: Signature) -> Vec<UnitBlade> {
    let n = signature.get_dimensions();

    let permutations = bit_permutations(n);
    permutations.iter().map(|x| UnitBlade::new(*x)).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn make_blades_returns_identity_for_0d() {
        let signature = Signature::new(0, 0, 0).unwrap();

        let result = make_blades(signature);

        let expected_blades = vec![UnitBlade::new(0b0)];

        assert_eq!(result, expected_blades);
    }

    #[test]
    fn make_blades_returns_identity_and_pseudoscalar_for_1d() {
        let signature = Signature::new(1, 0, 0).unwrap();

        let result = make_blades(signature);

        let expected_blades: Vec<UnitBlade> = [
            // scalar
            0b0, // pseudoscalar
            0b1,
        ]
        .map(|x| UnitBlade::new(x))
        .to_vec();

        assert_eq!(result, expected_blades);
    }

    #[test]
    fn make_blades_returns_scalars_vectors_bivectors_pseudoscalr_for_3d() {
        let signature = Signature::new(3, 0, 0).unwrap();

        let result = make_blades(signature);

        let expected_blades: Vec<UnitBlade> = [
            // scalar
            0b000, // vectors
            0b001, 0b010, 0b100, // bivectors
            0b011, 0b101, 0b110, // pseudoscalar
            0b111,
        ]
        .map(|x| UnitBlade::new(x))
        .to_vec();

        assert_eq!(result, expected_blades);
    }

    #[test]
    fn make_blades_returns_correct_blades_for_4d() {
        let signature = Signature::new(2, 1, 1).unwrap();

        let result = make_blades(signature);

        let expected_blades: Vec<UnitBlade> = [
            // scalar
            0b0000, // vectors
            0b0001, 0b0010, 0b0100, 0b1000, // bivectors
            0b0011, 0b0101, 0b1001, 0b0110, 0b1010, 0b1100, //trivectors
            0b0111, 0b1011, 0b1101, 0b1110, // pseudoscalr
            0b1111,
        ]
        .map(|x| UnitBlade::new(x))
        .to_vec();

        assert_eq!(result, expected_blades)
    }
}

mod signature;
mod unit_blade;

use std::collections::HashSet;

use signature::Signature;
use unit_blade::UnitBlade;

fn bit_permutations(dimensions: usize, one_bits: usize) -> Vec<u8> {
    return vec![];
}

pub fn make_blades(signature: Signature) -> Vec<UnitBlade> {
    let n = signature.get_dimensions();

    // Always include a scalar (1)
    let mut result = vec!(UnitBlade::new(0));

    for i in 1..n {
        for val in bit_permutations(n, i) {
            result.push(UnitBlade::new(val))
        }
    }

    result
}
use std::error::Error;

use geometric_algebra::{format_basis_blade::format_basis_blade, symbolic_multivector::SymCGA2};

fn main() -> Result<(), Box<dyn Error>> {
    let a = SymCGA2::generic_vector("A");

    let x = SymCGA2::vector(0);
    let y = SymCGA2::vector(1);
    let obj = x;

    println!("obj = {}", obj);

    let opns = obj.wedge(a);

    println!("OPNS: obj wedge A = 0");

    for (blade, coeff) in opns.get_nonzero_terms() {
        println!(
            "{}: {} = 0",
            format_basis_blade::<3, 1, 0>(
                &blade,
                geometric_algebra::format_basis_blade::ScalarFormat::One
            ),
            coeff
        );
    }

    Ok(())
}

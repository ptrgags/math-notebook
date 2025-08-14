use std::error::Error;

use geometric_algebra::{format_basis_blade::format_basis_blade, symbolic_multivector::SymCGA2};

fn main() -> Result<(), Box<dyn Error>> {
    let a = SymCGA2::generic_vector("A");

    let x = SymCGA2::vector(0);
    let y = SymCGA2::vector(1);
    let obj = x + y;

    // let's check:
    // (x + y) ^(ax:x + ay:y + ap:p + am:m)
    // (ay:xy + ap:xp + am:xm) + (-ax:xy + ap:yp + am:ym)
    // = (ay - ax):xy + ap:xp + am:xm + ap:yp + am:ym
    // yup!

    // the solution constraints are
    // ap = 0
    // am = 0
    // ay - ax = 0
    // i.e. ay = ax
    // In other words, solution vectors are of the form
    // a:x + a:y = 0
    // this is equivalent to the original vector, (x + y) by homogeneity, as
    // we would expect for a plane

    println!("A = {}", a);
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

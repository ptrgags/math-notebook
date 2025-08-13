use std::error::Error;

use geometric_algebra::{
    basis_blade::BasisBlade,
    format_basis_blade::{format_basis_blade, ScalarFormat},
    versor::{SymVersorCGA2, SymbolicVersor},
};

fn declare_versor<const P: u8, const N: u8, const Z: u8>(
    versor: &SymbolicVersor<P, N, Z>,
) -> String {
    let assignments: Vec<String> = versor
        .get_all_terms()
        .into_iter()
        .map(|(blade, coeff)| {
            format!(
                "{}: {}",
                format_basis_blade::<P, N, Z>(&blade, ScalarFormat::Variable),
                coeff
            )
        })
        .collect();
    let destructured = assignments.join(", ");
    format!("{{{}}}", destructured)
}

fn package_result<const P: u8, const N: u8, const Z: u8>(
    result: &SymbolicVersor<P, N, Z>,
) -> String {
    let class_name = match result {
        SymbolicVersor::Even(_) => "Even",
        SymbolicVersor::Odd(_) => "Odd",
    };

    let vars: Vec<String> = BasisBlade::get_all_blades(4)
        .into_iter()
        .map(|blade| format_basis_blade::<P, N, Z>(&blade, ScalarFormat::Variable))
        .collect();

    let comma_separated = vars.join(", ");
    format!("new {}({})", class_name, comma_separated)
}

fn main() -> Result<(), Box<dyn Error>> {
    // For this first iteration, let's just do the geometric product
    // for even * even
    let a = SymVersorCGA2::even("a");
    let b = SymVersorCGA2::even("b");
    let product = a.clone() * b.clone();

    let a_declaration = declare_versor(&a);
    let b_declaration = declare_versor(&b);

    let packaged = package_result(&product);

    println!("mul_even(other) {{");
    println!("  const {} = this;", a_declaration);
    println!("  const {} = other;", b_declaration);
    println!("");

    for (blade, coeff) in product.get_all_terms() {
        println!(
            "  const {} = {};",
            format_basis_blade::<3, 1, 0>(&blade, ScalarFormat::Variable),
            coeff
        );
    }
    println!("");

    println!("  return {};", packaged);
    println!("}}");

    Ok(())
}

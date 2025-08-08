use std::error::Error;

use geometric_algebra::{
    multivector::CGA2,
    rational_poly::RationalPolynomial,
    versor::{SymVersorCGA2, SymbolicVersor},
};

fn declare_versor(versor: &SymVersorCGA2) -> String {
    let assignments: Vec<String> = versor
        .get_all_terms()
        .into_iter()
        .map(|(blade, coeff)| {
            format!(
                "{}: {}",
                CGA2::<RationalPolynomial>::format_blade(blade),
                coeff
            )
        })
        .collect();
    let destructured = assignments.join(", ");
    format!("{{{}}}", destructured)
}

fn package_result(result: &SymVersorCGA2) -> String {
    let class_name = match result {
        SymbolicVersor::Even(_) => "Even",
        SymbolicVersor::Odd(_) => "Odd",
    };

    // TODO: generate this from the result
    let vars = vec!["x", "y", "xyz"];
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

    // Print the variable assignemnts here
    println!("{}", product);

    println!("  return {};", packaged);
    println!("}}");

    Ok(())
}

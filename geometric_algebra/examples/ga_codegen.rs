use std::error::Error;

use geometric_algebra::versor::SymbolicVersor;

fn declare_versor(versor: &SymbolicVersor) -> String {
    // TODO: generate this from versor
    let assignments = vec!["x: ax", "y: ay"];
    let destructured = assignments.join(", ");
    format!("{{{}}}", destructured)
}

fn package_result(result: &SymbolicVersor) -> String {
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
    let a = SymbolicVersor::even("a");
    let b = SymbolicVersor::even("b");
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

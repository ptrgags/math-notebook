use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // For this first iteration, let's just do the geometric product
    // for even * even

    //let a = SymbolicVersor::even("a");
    //let b = SymbolicVersor::even("b");
    //let product = a * b;

    let a_declaration = "{}";
    let b_declaration = "{}";

    let package_result = "new Even()";

    println!("mul_even(other) {{");
    println!("  const {} = this;", a_declaration);
    println!("  const {} = other;", b_declaration);

    // Print the variable assignemnts here

    println!("  return {};", package_result);
    println!("}}");

    Ok(())
}

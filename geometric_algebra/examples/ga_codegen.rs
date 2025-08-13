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

pub fn print_product_func<const P: u8, const N: u8, const Z: u8>(
    a: &SymbolicVersor<P, N, Z>,
    b: &SymbolicVersor<P, N, Z>,
) {
    let b_parity = match b {
        SymbolicVersor::Even(_) => "even",
        SymbolicVersor::Odd(_) => "odd",
    };

    let product = a.clone() * b.clone();

    let a_declaration = declare_versor(&a);
    let b_declaration = declare_versor(&b);

    let packaged = package_result(&product);

    println!("mul_{}(other) {{", b_parity);
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
}

fn main() -> Result<(), Box<dyn Error>> {
    let a_even = SymVersorCGA2::even("a");
    let b_even = SymVersorCGA2::even("b");
    let a_odd = SymVersorCGA2::odd("a");
    let b_odd = SymVersorCGA2::odd("b");

    println!("// even * even");
    print_product_func(&a_even, &b_even);

    println!("");
    println!("// even * odd");
    print_product_func(&a_even, &b_odd);

    println!("");
    println!("// odd * odd");
    print_product_func(&a_odd, &b_odd);

    println!("");
    println!("// odd * even");
    print_product_func(&a_odd, &b_even);

    Ok(())
}

use geometric_algebra::make_blades;

fn make_cayley_table(positive: u8, negative: u8, zero: u8) {
    let signature = Signature::new(positive, negative, zero);
    let blades = make_blades(signature);

    for i in 0..blades.len() {
        for j in 0..blades.len() {
            product =
        }
    }
}


fn main() {
    println!("Hello, world!");
}

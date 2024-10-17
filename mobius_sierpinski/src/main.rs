use rand::random;

fn chaos_game(xforms: &[Mobius], start_point: Complex, n: usize) -> Vec<Complex> {
    let mut z = start_point;
    (0..n)
        .map(|_| {
            z = xforms[random::<usize>() % xforms.len()] * z;
            z
        })
        .collect()
}

fn main() {
    let MobiusSierpinski { a, b, c } = compute_xforms();

    /*
    println!("A:");
    println!("{}", a);
    println!("type: {:?}", a.classify());
    println!("fixed points: {}", a.fixed_points());

    println!("B:");
    println!("{}", b);
    println!("type: {:?}", b.classify());
    println!("fixed points: {}", b.fixed_points());

    println!("C:");
    println!("{}", c);
    println!("type: {:?}", c.classify());
    println!("fixed points: {}", c.fixed_points());
    */

    //let forward_only = vec![a, b, c];
    /*for z in chaos_game(&forward_only, 10000) {
        println!("{},{}", z.real(), z.imag());
    }
    */

    /*
    for _ in 0..100 {
        let start_point = Complex::new(
            4.0 * random::<f64>() - 2.0,
            4.0 * random::<f64>() - 2.0
        );
        for z in chaos_game(&forward_only, start_point, 1000) {
            println!("{},{}", z.real(), z.imag());
        }
    }
    */

    let tile = ClineTile::new(vec![
        Cline::line(Complex::I, 0.0).unwrap(),
        Cline::line(Complex::ONE, 0.0).unwrap(),
        Cline::circle(Complex::Zero, 1.0),
    ]);

    let tile_a = tile.transform(a);
    let tile_b = tile.transform(b);
    let tile_c = tile.transform(c);

    println!("Original\n{}", tile);
    println!("A\n{}\n{}", a, tile_a);
    println!("B\n{}\n{}", b, tile_b);
    println!("C\n{}\n{}", c, tile_c);
}

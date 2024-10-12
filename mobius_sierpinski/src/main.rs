use mobius::{scale, Complex, Mobius};
use rand::random;

struct MobiusSierpinski {
    a: Mobius,
    b: Mobius,
    c: Mobius,
}

fn compute_xforms() -> MobiusSierpinski {
    // Transform A just shrinks the unit circle to the circle with
    // radius 1/2
    let xform_a = scale(0.5).unwrap();

    // Transform B has the following properties:
    // B(0) = 1/2
    //  --> d = 2b
    //  --> B = [a  b]
    //        = [c 2b]
    // B(1) = 1
    //  --> a + b = c + d
    //          a = c + (2b) - b
    //          a = c + b
    //  --> B = [(c + b)  b]
    //          [c       2b]
    // B(i) = sqrt(i)  (call this omega)
    //  --> (ai + b) = omega (ci + d)
    //       (c + b)i + b = omega(ci + 2b)
    //       ci + bi + b = (omega c)i + 2 omega b
    //       (1 - omega)i c = (2 omega - i - 1) b
    //       (1 - omega)i / (2 omega - i - 1) c = b
    //  let's call the mess on the left gamma, so that
    //       gamma c = b
    //  --> [(c + gamma c)   (gamma c)]
    //      [c             (2 gamma c)]
    // det B = 1
    //  --> (c + gamma c)(2 gamma c) - c(gamma c) = 1
    //      2 gamma c^2 + 2 gamma^2 c^2 - gamma c^2
    //      (2 gamma + 2 gamma^2 - gamma)c^2 = 1
    //      (gamma + 2 gamma^2)c^2 = 1
    //      c = sqrt(1 / (gamma + 2 gamma^2))
    let omega = Complex::I.sqrt();
    let two: Complex = (2.0).into();
    let gamma_numerator = (Complex::ONE - omega) * Complex::I;
    let gamma_denominator: Complex = two * omega - Complex::new(1.0, 1.0);
    let gamma = gamma_numerator / gamma_denominator;
    let c = (Complex::ONE / (gamma + two * gamma * gamma)).sqrt();
    let b = gamma * c;
    let d = two * b;
    let a = b + c;
    let xform_b = Mobius::new(a, b, c, d).expect("Determinant not one???");

    // the transform C is essentially the same thing as B,
    // except mirrored over y = x
    // let mirror(z) = i * conj(z)
    // and note that mirror^(-1) = mirror
    //
    // we want 
    // C = mirror 🥪 B
    //   = mirror B mirror
    //   = i conj((a (i conj(z)) + b) / (c (i conj (z)) + d))
    //   = i (conj(a) conj(i) z + conj(b)) / (conj(c) conj(i) z) + conj(d))
    //   = i (-i conj(a) z + conj(b)) / (-i conj(c) z) + conj(d))
    //   = (conj(a) z + i conj(b)) / (-i conj(c) z + conj(d))
    //
    // so a' = conj(a)
    //    b' = i conj(b) = mirror(b)
    //    c' = - i conj(c) = -mirror(c)
    //    d' = conj(d)
    let xform_c = Mobius::new(
        a.conj(),
        Complex::I * b.conj(),
        -Complex::I * c.conj(),
        d.conj(),
    ).expect("Determinant not 1???");

    MobiusSierpinski {
        a: xform_a,
        b: xform_b,
        c: xform_c,
    }
}

fn chaos_game(xforms: &[Mobius], start_point: Complex, n: usize) -> Vec<Complex> {
    let mut z = start_point;
    (0..n).map(|_| {
        z = xforms[random::<usize>() % xforms.len()] * z;
        z
    }).collect()
}

fn main() {
    let MobiusSierpinski{a, b, c} = compute_xforms();

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

    
    let forward_only = vec![a, b, c];
    /*for z in chaos_game(&forward_only, 10000) {
        println!("{},{}", z.real(), z.imag());
    }
    */

    for _ in 0..100 {
        let start_point = Complex::new(
            4.0 * random::<f64>() - 2.0,
            4.0 * random::<f64>() - 2.0
        );
        for z in chaos_game(&forward_only, start_point, 1000) {
            println!("{},{}", z.real(), z.imag());
        }
    }
}
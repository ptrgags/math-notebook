use crate::basis_blade::BasisBlade;

fn format_labeled(blade: &BasisBlade, basis_labels: &[&str], dimension: u8) -> String {
    let &BasisBlade(bits) = blade;

    if bits == 0 {
        return String::from("1");
    }

    let vector_labels: Vec<&str> = (0..dimension)
        .filter_map(|i| {
            if bits >> i & 1 == 0 {
                None
            } else {
                Some(basis_labels[i as usize])
            }
        })
        .collect();

    vector_labels.join("")
}

const VGA_BASIS: [&str; 3] = ["x", "y", "z"];
fn format_vga(blade: &BasisBlade, dimension: u8) -> String {
    format_labeled(blade, &VGA_BASIS, dimension)
}

fn format_pga(blade: &BasisBlade, dimension: u8) -> String {
    let pga_basis = match dimension {
        0 => vec!["o"],
        1 => vec!["x", "o"],
        2 => vec!["x", "y", "o"],
        3 => vec!["x", "y", "z", "o"],
        _ => panic!("only supported for 0-3D PGA"),
    };

    // PGA is G(N, 0, 1), so the overall dimension is N + 1
    format_labeled(blade, &pga_basis, dimension + 1)
}

fn format_cga(blade: &BasisBlade, euclidean_dimension: u8) -> String {
    let pga_basis = match euclidean_dimension {
        0 => vec!["p", "m"],
        1 => vec!["x", "p", "m"],
        2 => vec!["x", "y", "p", "m"],
        3 => vec!["x", "y", "z", "p", "m"],
        _ => panic!("only supported for 0-3D CGA"),
    };

    // CGA is G(N + 1, 1), so the overall dimension is N + 2
    format_labeled(blade, &pga_basis, euclidean_dimension + 2)
}

fn format_general(blade: &BasisBlade) -> String {
    let &BasisBlade(bits) = blade;

    if bits == 0 {
        return String::from("1");
    }

    let vector_labels: Vec<String> = (0..8)
        .filter_map(|i| {
            if bits >> i & 1 == 0 {
                None
            } else {
                Some(format!("{}", i))
            }
        })
        .collect();

    format!("e_{}", vector_labels.join(""))
}

/// Format a basis blade in a more human-readable form. This depends on the
/// algebra because I like shorthand.
///
/// For VGA, this will use basis x, [y, [z]]
/// for PGA, this will use basis x  [y, [z]], o
/// for CGA, this will use basis x, [y, [z]], p, m
pub fn format_basis_blade<const P: u8, const N: u8, const Z: u8>(blade: &BasisBlade) -> String {
    match (P, N, Z) {
        (p, 0, 0) if p <= 3 => format_vga(blade, P),
        (p, 0, 1) if p <= 3 => format_pga(blade, P),
        (p, 1, 0) if p <= 4 => format_cga(blade, P - 1),
        _ => format_general(blade),
    }
}

#[cfg(test)]
mod test {
    use crate::basis_blade::BasisBlade;

    use super::*;

    mod vga {
        use super::*;

        #[test]
        pub fn formats_scalar_as_one() {
            let scalar = BasisBlade::scalar();

            let result = format_basis_blade::<2, 0, 0>(&scalar);

            assert_eq!(result, "1");
        }

        #[test]
        pub fn formats_vector() {
            let y = BasisBlade::vector(1);

            let result = format_basis_blade::<2, 0, 0>(&y);

            assert_eq!(result, "y");
        }

        #[test]
        pub fn formats_bivector() {
            let xz = BasisBlade::bivector(0, 2);

            let result = format_basis_blade::<3, 0, 0>(&xz);

            assert_eq!(result, "xz");
        }
    }

    mod pga {

        use super::*;

        #[test]
        pub fn formats_scalar_as_one() {
            let scalar = BasisBlade::scalar();

            let result = format_basis_blade::<2, 0, 1>(&scalar);

            assert_eq!(result, "1");
        }

        #[test]
        pub fn formats_vector() {
            let y = BasisBlade::vector(1);

            let result = format_basis_blade::<2, 0, 1>(&y);

            assert_eq!(result, "y");
        }

        #[test]
        pub fn formats_null_vector_as_o() {
            let o = BasisBlade::vector(2);

            let result = format_basis_blade::<2, 0, 1>(&o);

            assert_eq!(result, "o");
        }

        #[test]
        pub fn formats_bivector_without_o() {
            let xz = BasisBlade::bivector(0, 2);

            let result = format_basis_blade::<3, 0, 1>(&xz);

            assert_eq!(result, "xz");
        }

        #[test]
        pub fn formats_bivector_with_o() {
            let xo = BasisBlade::bivector(0, 2);

            let result = format_basis_blade::<2, 0, 1>(&xo);

            assert_eq!(result, "xo");
        }
    }

    mod cga {
        use super::*;

        #[test]
        pub fn formats_scalar_as_one() {
            let scalar = BasisBlade::scalar();

            let result = format_basis_blade::<3, 1, 0>(&scalar);

            assert_eq!(result, "1");
        }

        #[test]
        pub fn formats_vector() {
            let y = BasisBlade::vector(1);

            let result = format_basis_blade::<3, 1, 0>(&y);

            assert_eq!(result, "y");
        }

        #[test]
        pub fn formats_plus_vec_as_p() {
            let p = BasisBlade::vector(2);

            let result = format_basis_blade::<3, 1, 0>(&p);

            assert_eq!(result, "p");
        }

        #[test]
        pub fn formats_minus_vec_as_m() {
            let m = BasisBlade::vector(3);

            let result = format_basis_blade::<3, 1, 0>(&m);

            assert_eq!(result, "m");
        }

        #[test]
        pub fn formats_bivector_without_pm() {
            let xz = BasisBlade::bivector(0, 2);

            let result = format_basis_blade::<4, 1, 0>(&xz);

            assert_eq!(result, "xz");
        }

        #[test]
        pub fn formats_trivector_with_pm() {
            let xpm = BasisBlade::trivector(0, 2, 3);

            let result = format_basis_blade::<3, 1, 0>(&xpm);

            assert_eq!(result, "xpm");
        }
    }

    mod general {
        use super::*;

        #[test]
        pub fn formats_scalar_as_one() {
            let scalar = BasisBlade::scalar();

            let result = format_basis_blade::<1, 1, 1>(&scalar);

            assert_eq!(result, "1");
        }

        #[test]
        pub fn formats_vector_with_one_indexedsubscript() {
            let y = BasisBlade::vector(1);

            let result = format_basis_blade::<1, 1, 1>(&y);

            assert_eq!(result, "e_1");
        }

        #[test]
        pub fn formats_bivector_with_multiple_indices() {
            let e_02 = BasisBlade::bivector(0, 2);

            let result = format_basis_blade::<1, 1, 1>(&e_02);

            assert_eq!(result, "e_02");
        }
    }
}

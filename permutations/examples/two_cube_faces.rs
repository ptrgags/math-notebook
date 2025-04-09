use abstraction::Group;
use permutations::{DisjointCycles, Permutation};

// Two faces of a 2x2 cube
type TwoFaces = Permutation<18>;

fn display(label: &str, x: &TwoFaces) {
    println!("{}: {}", label, x);
    println!("order: {}", x.order());
    println!("out of place: {}", x.out_of_place());
}

// This example explores different sequences of rotating two adjacent faces of a
// 2x2 twisty cube. This permutes 18 out of the 24 facelets of the cube, leaving
// the far edge's 6 faces unchanged
pub fn main() {
    let a: TwoFaces = Permutation::from_disjoint_cycles(DisjointCycles(vec![
        vec![0, 1, 2, 3],
        vec![4, 8, 10, 12],
        vec![7, 9, 11, 13],
    ]))
    .unwrap();
    let b: TwoFaces = Permutation::from_disjoint_cycles(DisjointCycles(vec![
        vec![1, 14, 16, 8],
        vec![2, 13, 15, 17],
        vec![4, 5, 6, 7],
    ]))
    .unwrap();

    let commutator_ab = Permutation::commutator(a, b);

    display("a", &a);
    display("b", &b);
    display("A", &a.inverse());
    display("B", &b.inverse());

    let ab = a * b;
    let ab_inv = a * b.inverse();
    display("ab", &ab);
    display("aB", &ab_inv);
    display("abAB", &commutator_ab);

    let comm_a_b_inv = Permutation::commutator(a, b.inverse());
    display("aBAb", &comm_a_b_inv);

    let comm_ab2 = commutator_ab * commutator_ab;
    display("(abAB)^2", &comm_ab2);

    let comm_ab3 = commutator_ab * comm_ab2;
    display("(abAB)^3", &comm_ab3);
}

use std::collections::HashMap;

use permutations::Permutation;

const ORDER: usize = 24;
const OCTAHEDRAL_GROUP_DATA: [(&str, [usize; 4]); ORDER] = [
    ("I", [0, 1, 2, 3]),
    ("x", [3, 2, 0, 1]),
    ("x^2", [1, 0, 3, 2]),
    ("x^3", [2, 3, 1, 0]),
    ("y", [2, 0, 3, 1]),
    ("y^2", [3, 2, 1, 0]),
    ("y^3", [1, 3, 0, 2]),
    ("z", [1, 2, 3, 0]),
    ("z^2", [2, 3, 0, 1]),
    ("z^3", [3, 0, 1, 2]),
    ("E01", [1, 0, 2, 3]),
    ("E02", [2, 1, 0, 3]),
    ("E03", [3, 1, 2, 0]),
    ("E12", [0, 2, 1, 3]),
    ("E13", [0, 3, 2, 1]),
    ("E23", [0, 1, 3, 2]),
    ("C0", [0, 3, 1, 2]),
    ("C0^2", [0, 2, 3, 1]),
    ("C1", [3, 1, 0, 2]),
    ("C1^2", [2, 1, 3, 0]),
    ("C2", [3, 0, 2, 1]),
    ("C2^2", [1, 3, 2, 0]),
    ("C3", [2, 0, 1, 3]),
    ("C3^2", [1, 2, 0, 3]),
];

fn make_forward_map() -> HashMap<String, Permutation> {
    let mut result = HashMap::new();
    for (label, elements) in OCTAHEDRAL_GROUP_DATA.iter() {
        result.insert(String::from(*label), Permutation::new(elements).unwrap());
    }

    result
}

fn make_reverse_map() -> HashMap<Permutation, String> {
    let mut result = HashMap::new();
    for (label, elements) in OCTAHEDRAL_GROUP_DATA.iter() {
        result.insert(Permutation::new(elements).unwrap(), String::from(*label));
    }

    result
}


fn main() {
    let label_order: Vec<&str> = OCTAHEDRAL_GROUP_DATA.iter().map(|(s, _)| *s).collect();
    let forward = make_forward_map();
    let backward = make_reverse_map();
    
    let mut table: Vec<String> = Vec::new();
    for a_label in label_order.iter() {
        let a_perm = forward.get(*a_label).unwrap();
        let mut row: Vec<String> = Vec::new();
        for b_label in label_order.iter() {
            let b_perm = forward.get(*b_label).unwrap();
            let product = Permutation::compose(a_perm.clone(), b_perm.clone()).unwrap();
             
            let product_label = backward.get(&product).unwrap().clone();
            row.push(product_label)
        }
        table.push(row.join(","));
    }

    for row in table {
        println!("{}", row);
    }
}

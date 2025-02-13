use std::{f64::consts::PI, io::Error};

use abstraction::Monoid;
use mobius::{
    algorithms::{OrbitIFS, OrbitTile},
    isogonal::Isogonal,
    rotation, translation, Complex,
};

fn main() -> Result<(), Error> {
    let mirror_x = Isogonal::conj();
    let r180: Isogonal = rotation(PI).unwrap().into();
    let mirror_y = r180 * mirror_x;
    let translate_x: Isogonal = translation(Complex::ONE * (2.0).into()).unwrap().into();
    let translate_y: Isogonal = translation(Complex::I * (2.0).into()).unwrap().into();

    let neighbor_tile_xforms = vec![
        translate_x * mirror_y,
        translate_y * mirror_x,
        mirror_y,
        mirror_x,
    ];

    let test_point = Complex::new(0.5, 0.5);

    //let initial_tile = OrbitTile::new(Isogonal::identity(), neighbor_tile_xforms, test_point);
    //let ifs = OrbitIFS::new(initial_tile);

    let right = neighbor_tile_xforms[0] * test_point;
    let up = neighbor_tile_xforms[1] * test_point;
    let left = neighbor_tile_xforms[2] * test_point;
    let down = neighbor_tile_xforms[3] * test_point;

    println!("Right: {}", right);
    println!("Up: {}", up);
    println!("Left: {}", left);
    println!("Down: {}", down);

    Ok(())
}

use geometric_algebra::pga_2d::{line::Line, point::Point};

pub fn main() {
    // The goal is to draw a perspective box room on a 2D piece of paper
    // The front side of the box is a rectangle from (-6, 6) to (6, -4).

    // Let's start with some basic tasks. Let's compute the 4 lines that
    // bound the rectangle, and use the meet operator to compute the 4
    // corners.
    // I'll use the convention that the normals point outside of the box.
    let left_side = Line::new(-1.0, 0.0, 6.0);
    let right_side = Line::new(1.0, 0.0, 6.0);
    let top_side = Line::new(0.0, 1.0, 6.0);
    let bottom_side = Line::new(0.0, -1.0, 4.0);

    println!("Rectangle sides:");
    println!("left: {}", left_side);
    println!("top: {}", top_side);
    println!("right: {}", right_side);
    println!("bottom: {}", bottom_side);

    // now let's compute the four corners
    let top_left = top_side.meet(left_side);
    let top_right = top_side.meet(right_side);
    let bottom_left = bottom_side.meet(left_side);
    let bottom_right = bottom_side.meet(right_side);

    println!("Corners:");
    println!("top left: {}", top_left);
    println!("top right: {}", top_right);
    println!("bottom left: {}", bottom_left);
    println!("bottom right: {}", bottom_right);

    // Create a vanishing point
    let vanish = Point::new(1.0, 2.0);
    println!("vanishing point: {}", vanish);

    // Now join the corners to the vanishing point
    let top_left_diag = top_left.join(vanish);
    let top_right_diag = top_right.join(vanish);
    let bottom_left_diag = bottom_left.join(vanish);
    let bottom_right_diag = bottom_right.join(vanish);

    println!("tl diag: {}", top_left_diag);
    println!("tr diag: {}", top_right_diag);
    println!("bl diag: {}", bottom_left_diag);
    println!("br diag: {}", bottom_right_diag);
}

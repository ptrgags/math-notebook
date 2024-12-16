use geometric_algebra::pga_2d::line::Line;

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
    let top_left = top_side.meet(&left_side);
    let top_right = top_side.meet(&right_side);
    let bottom_left = bottom_side.meet(&left_side);
    let bottom_right = bottom_side.meet(&right_side);

    println!("Corners:");
    println!("top left: {}", top_left);
    println!("top right: {}", top_right);
    println!("bottom left: {}", bottom_left);
    println!("bottom right: {}", bottom_right);
}

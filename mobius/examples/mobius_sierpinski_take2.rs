use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};

use mobius::{
    cline_arc::ClineArc,
    cline_tile::ClineArcTile,
    map_triple, scale,
    svg_plot::{add_geometry, make_card, svg_cline_arc_tiles},
    Complex, Mobius,
};
use svg::node::element::Group;

struct IFS {
    xforms: Vec<Mobius>,
}

impl IFS {
    pub fn new(xforms: Vec<Mobius>) -> Self {
        Self { xforms }
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, Mobius> {
        self.xforms.iter()
    }

    pub fn dfs(&self, max_depth: usize) -> IFSDepthFirstIterator {
        IFSDepthFirstIterator::new(self, max_depth)
    }
}

struct IFSDepthFirstIterator<'a> {
    ifs: &'a IFS,
    max_depth: usize,
    // pairs of (depth, xform)
    stack: Vec<(usize, Mobius)>,
}

impl<'a> IFSDepthFirstIterator<'a> {
    fn new(ifs: &'a IFS, max_depth: usize) -> Self {
        Self {
            ifs,
            max_depth,
            stack: vec![(0, Mobius::IDENTITY)],
        }
    }
}

impl<'a> Iterator for IFSDepthFirstIterator<'a> {
    type Item = (usize, Mobius);

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            None => None,
            Some((depth, xform)) => {
                if depth < self.max_depth {
                    for next_xform in self.ifs.iter().cloned() {
                        self.stack.push((depth + 1, next_xform * xform));
                    }
                }
                Some((depth, xform))
            }
        }
    }
}

fn compute_xforms() -> Vec<Mobius> {
    // Similar to the Sierpinski triangle, we want to send the overall triangle
    // to the 3 corners, and the three triangles should touch exactly at the
    // corners. To make this happen, start with a tangency point at
    //      sqrt(i) = exp(i * pi / 4)
    //
    // This is a 45 degree angle from the center of the unit circle.
    // drawing a tangent line, it meets the real axis at sqrt(2)
    // A circle with center there that goes through the tangency point will
    // have radius 1 by mirror symmetry about x=cos(pi/4).
    //
    // From there, we can compute where this second circle intersects the
    // real axis: r = sqrt(2) - 1
    //
    // See https://www.desmos.com/calculator/pcwwbwk2qr
    let radius: f64 = (2.0f64).sqrt() - 1.0;

    // The A transform is a simple shrink transform from 1 -> r
    let xform_a = scale(radius).unwrap();

    // curved triangle that bounds the first quadrant of the unit circle
    let triangle_corners = (Complex::Zero, Complex::ONE, Complex::I);

    // The B transform has a fixed point at 1 and maps the imaginary segment
    // to the arc between r and sqrt(i)
    let sqrt_i = Complex::I.sqrt();
    let xform_b = map_triple(
        triangle_corners,
        (Complex::new(radius, 0.0), Complex::ONE, sqrt_i),
    )
    .unwrap();

    // The C transform has a fixed point at i and maps the real segment
    // to the arc between ri and sqrt(i)

    let xform_c = map_triple(
        triangle_corners,
        (Complex::new(0.0, radius), sqrt_i, Complex::I),
    )
    .unwrap();

    vec![xform_a, xform_b, xform_c]
}

fn main() {
    let modified_sierpinski = IFS::new(compute_xforms());

    let tile = ClineArcTile::new(vec![
        ClineArc::line_segment(Complex::Zero, Complex::ONE),
        ClineArc::from_circle_and_angles(Complex::Zero, 1.0, 0.0, FRAC_PI_4, FRAC_PI_2),
        ClineArc::line_segment(Complex::I, Complex::Zero),
    ]);

    let sierpinski_tiles: Vec<ClineArcTile> = modified_sierpinski
        .dfs(6)
        .map(|(_, xform)| tile.transform(xform))
        .collect();

    let svg_tiles = svg_cline_arc_tiles(&sierpinski_tiles);

    let mut geometry = Group::new()
        .set("stroke", "orange")
        .set("stroke-width", "0.125%")
        .set("fill", "none");
    geometry = add_geometry(geometry, svg_tiles);

    let flipped = Group::new().set("transform", "scale(1, -1)").add(geometry);

    let doc = make_card(Complex::new(0.5, 0.5), 0.5001).add(flipped);
    svg::save("output/mobius_sierpinski2.svg", &doc).unwrap();
}

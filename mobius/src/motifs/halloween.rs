use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, PI, SQRT_2, TAU};

use crate::{
    geometry::{Circle, CircularArc, LineSegment},
    rendering::Style,
    scale,
    transformable::{ClineArcTile, Motif, Transformable},
    Complex,
};

fn circle_to_arcs(circle: Circle) -> (CircularArc, CircularArc) {
    let top = CircularArc::new(circle, 0.0, FRAC_PI_2, PI);
    let bottom = CircularArc::new(circle, PI, 3.0 * FRAC_PI_2, TAU);

    (top, bottom)
}

pub fn ghost() -> (ClineArcTile, Style) {
    const SIDE_HEIGHT: f64 = 1.5;
    const CIRCLE_SPACING: f64 = 2.0 / 5.0;
    const BOTTOM_CIRCLE_RADIUS: f64 = 1.0 / 5.0;
    let head_circle = Circle::unit_circle();
    let bottom_circles: Vec<Circle> = (0..5)
        .map(|i| {
            Circle::new(
                Complex::new((-2.0 + i as f64) * CIRCLE_SPACING, -SIDE_HEIGHT),
                BOTTOM_CIRCLE_RADIUS,
            )
        })
        .collect();

    let left_eye = Circle::new(Complex::new(-0.5, 0.0), 0.25);
    let right_eye = Circle::new(Complex::new(0.5, 0.0), 0.25);
    let mouth = Circle::new(Complex::new(0.0, -0.5), 0.125);

    let (left_eye_top, left_eye_bottom) = circle_to_arcs(left_eye);
    let (right_eye_top, right_eye_bottom) = circle_to_arcs(right_eye);
    let (mouth_top, mouth_bottom) = circle_to_arcs(mouth);

    let ghost = ClineArcTile::new(vec![
        // top of ghost head is a semi-circle
        CircularArc::new(head_circle, 0.0, FRAC_PI_2, PI).into(),
        // Left side
        LineSegment::new(-Complex::ONE, Complex::new(-1.0, -SIDE_HEIGHT)).into(),
        // Five semi-circles for the bottom
        CircularArc::new(bottom_circles[0], PI, 3.0 * FRAC_PI_2, TAU).into(),
        CircularArc::new(bottom_circles[1], PI, FRAC_PI_2, 0.0).into(),
        CircularArc::new(bottom_circles[2], PI, 3.0 * FRAC_PI_2, TAU).into(),
        CircularArc::new(bottom_circles[3], PI, FRAC_PI_2, 0.0).into(),
        CircularArc::new(bottom_circles[4], PI, 3.0 * FRAC_PI_2, TAU).into(),
        // Right side
        LineSegment::new(Complex::new(1.0, -SIDE_HEIGHT), Complex::ONE).into(),
        // Eyes and mouths are circles drawn as two semicircles
        left_eye_top.into(),
        left_eye_bottom.into(),
        right_eye_top.into(),
        right_eye_bottom.into(),
        mouth_top.into(),
        mouth_bottom.into(),
    ]);

    let style = Style::stroke(0xc5, 0xf2, 0xfa).with_width(0.25);

    (ghost, style)
}

fn lerp(a: Complex, b: Complex, t: f64) -> Complex {
    a * (1.0 - t).into() + b * t.into()
}

/// Make a candy corn motif
/// See my [desmos sketch](https://www.desmos.com/calculator/4gidpmhf7c) for
/// an illustration
pub fn candy_corn() -> (Motif, Vec<Style>) {
    // Start with a triangle at the third roots of unity
    let a = Complex::ONE;
    let b = Complex::from_polar(1.0, 2.0 * FRAC_PI_3);
    let c = Complex::from_polar(1.0, 4.0 * FRAC_PI_3);

    // Get points 1/3 and 2/3 along the sides to mark where
    // the colors transition
    let ab1 = lerp(a, b, 1.0 / 3.0);
    let ab2 = lerp(a, b, 2.0 / 3.0);
    let ac1 = lerp(a, c, 1.0 / 3.0);
    let ac2 = lerp(a, c, 2.0 / 3.0);

    // We want to round the corners, so compute points
    // a little inside from the corners.
    let radius = 0.1;
    let scale_factor = Complex::from(1.0 - 2.0 * radius);
    let center_a = scale_factor * a;
    let center_b = scale_factor * b;
    let center_c = scale_factor * c;

    // Compute the six intersection points between the round circles
    // and the sides. Since the original triangle is equilateral,
    // these are just offsets from the rounding circle centers at
    // 60 degrees + a multiple of 120 degrees.
    let offset_ab = Complex::from_polar(radius, FRAC_PI_3);
    let offset_bc = Complex::from(-radius);
    let offset_ca = Complex::from_polar(radius, -FRAC_PI_3);
    let isx_a_ab = center_a + offset_ab;
    let isx_b_ab = center_b + offset_ab;
    let isx_b_bc = center_b + offset_bc;
    let isx_c_bc = center_c + offset_bc;
    let isx_c_ca = center_c + offset_ca;
    let isx_a_ca = center_a + offset_ca;

    // Compute the line segments in counterclockwise order
    let line_a_ab1 = LineSegment::new(isx_a_ab, ab1);
    let line_ab1_ab2 = LineSegment::new(ab1, ab2);
    let line_ab2_b = LineSegment::new(ab2, isx_b_ab);
    let line_bc = LineSegment::new(isx_b_bc, isx_c_bc);
    let line_c_ac2 = LineSegment::new(isx_c_ca, ac2);
    let line_ac2_ac1 = LineSegment::new(ac2, ac1);
    let line_ac1_a = LineSegment::new(ac1, isx_a_ca);

    // Compute the arcs for the corners
    let circle_a = Circle::new(center_a, radius);
    let circle_b = Circle::new(center_b, radius);
    let circle_c = Circle::new(center_c, radius);
    let arc_a = CircularArc::new(circle_a, -FRAC_PI_3, 0.0, FRAC_PI_3);
    let arc_b = CircularArc::new(circle_b, FRAC_PI_3, 2.0 * FRAC_PI_3, PI);
    let arc_c = CircularArc::new(circle_c, -PI, -2.0 * FRAC_PI_3, -FRAC_PI_3);

    // Compute arcs dividing the 3 parts of the candy corn
    let len_ab = (b - a).mag();
    let radius1 = len_ab / 3.0;
    let radius2 = 2.0 * radius1;
    let circle1 = Circle::new(a, radius1);
    let circle2 = Circle::new(a, radius2);
    let arc1 = CircularArc::new(circle1, 5.0 * FRAC_PI_6, PI, 7.0 * FRAC_PI_6);
    let arc2 = CircularArc::new(circle2, 5.0 * FRAC_PI_6, PI, 7.0 * FRAC_PI_6);

    // Build the three parts of the candy corn
    let base = ClineArcTile::new(vec![
        line_ab2_b.into(),
        arc_b.into(),
        line_bc.into(),
        arc_c.into(),
        line_c_ac2.into(),
        arc2.into(),
    ]);
    let mid = ClineArcTile::new(vec![
        line_ab1_ab2.into(),
        arc2.into(),
        line_ac2_ac1.into(),
        arc1.into(),
    ]);
    let tip = ClineArcTile::new(vec![
        arc_a.into(),
        line_a_ab1.into(),
        arc1.into(),
        line_ac1_a.into(),
    ]);

    let styles = vec![
        // Yellow base
        Style::stroke(255, 255, 0).with_width(0.25),
        // Orange middle
        Style::stroke(255, 127, 0).with_width(0.25),
        // White tip
        Style::stroke(255, 255, 255).with_width(0.25),
    ];

    let candy_corn = Motif::new(vec![(base, 0), (mid, 1), (tip, 2)]);

    (candy_corn, styles)
}

/// Create a vertical cartoon bone that fits in [-2, 2] x [- (length/2 + 1), (length/2 + 1)]
/// see https://www.desmos.com/calculator/tabdjja814
///
/// Since that's rather large, this shrinks the result down so the epiphyses
/// are at -i and i. So the length parameter really determines the length
/// to width ratio in practice.
pub fn bone(length: f64) -> ClineArcTile {
    let half_width = Complex::ONE;
    let half_height = Complex::new(0.0, 0.5 * length);

    // Create four 3/4 circles of radius 1 at the corners of the rectangle. These
    // circles will stick out a little bit. apparently that part of a bone
    // is called an "epiphysis"
    let center_top_right = half_width + half_height;
    let center_bottom_left = -center_top_right;
    let center_top_left = center_bottom_left.conj();
    let center_bottom_right = center_top_right.conj();

    let circle_top_left = Circle::new(center_top_left, 1.0);
    let circle_top_right = Circle::new(center_top_right, 1.0);
    let circle_bottom_left = Circle::new(center_bottom_left, 1.0);
    let circle_bottom_right = Circle::new(center_bottom_right, 1.0);

    let arc_top_left = CircularArc::new(circle_top_left, 0.0, FRAC_PI_2, 3.0 * FRAC_PI_2);
    let arc_top_right = CircularArc::new(circle_top_right, -FRAC_PI_2, FRAC_PI_2, PI);
    let arc_bottom_left = CircularArc::new(circle_bottom_left, FRAC_PI_2, PI, 3.0 * FRAC_PI_2);
    let arc_bottom_right = CircularArc::new(circle_bottom_right, -PI, -FRAC_PI_2, FRAC_PI_2);

    let left_side = LineSegment::new(
        center_top_left - Complex::I,
        center_bottom_left + Complex::I,
    );
    let right_side = LineSegment::new(
        center_bottom_right + Complex::I,
        center_top_left - Complex::I,
    );

    // the bone line's connected to the bone arc
    // the bone arc's connected to the bone arc... 🎵
    let big_bone = ClineArcTile::new(vec![
        right_side.into(),
        arc_top_right.into(),
        arc_top_left.into(),
        left_side.into(),
        arc_bottom_left.into(),
        arc_bottom_right.into(),
    ]);

    let shrink = scale(2.0 / length).unwrap();

    big_bone.transform(shrink)
}

/// Create a motif shaped like a witch's hat.
/// It's normalized so it fits in the unit circle
pub fn witch_hat() -> Motif {
    // The circles can be found at https://www.desmos.com/calculator/nrdpneh58g
    // The brim of the hat is made from 3 circular arcs
    let circle_brim_bottom = Circle::new(Complex::Zero, 2.0);
    let circle_brim_left = Circle::new(-Complex::ONE, 1.0);
    let circle_brim_right = Circle::new(Complex::ONE, 1.0);
    // This hat is very pointy! in fact the angle at the point is 0!
    let circle_point_bottom = Circle::new(Complex::new(2.0, 1.0), 1.0);
    let circle_point_top = Circle::new(Complex::new(1.0, 1.0), 2.0);

    // Outline the outside of the hat
    let arc_brim_left = CircularArc::new(circle_brim_left, FRAC_PI_2, 3.0 * FRAC_PI_4, PI);
    let arc_brim_bottom = CircularArc::new(circle_brim_bottom, -PI, -FRAC_PI_2, 0.0);
    let arc_brim_right = CircularArc::new(circle_brim_right, 0.0, FRAC_PI_4, FRAC_PI_2);
    let arc_point_bottom = CircularArc::new(circle_point_bottom, PI, FRAC_PI_2, 0.0);
    let arc_point_top = CircularArc::new(circle_point_top, 0.0, FRAC_PI_2, PI);
    let outside = ClineArcTile::new(vec![
        arc_brim_left.into(),
        arc_brim_bottom.into(),
        arc_brim_right.into(),
        arc_point_bottom.into(),
        arc_point_top.into(),
    ]);

    // Add a band in the middle in a different color to make it look more
    // like a hat
    let circle_band_top = Circle::new(Complex::new(0.0, 2.0), SQRT_2);
    let circle_band_bottom = Circle::new(Complex::I, SQRT_2);

    let arc_band_top = CircularArc::new(
        circle_band_top,
        -FRAC_PI_4,
        -3.0 * FRAC_PI_2,
        -3.0 * FRAC_PI_4,
    );
    let band_left = LineSegment::new(Complex::new(-1.0, 1.0), -Complex::ONE);
    let arc_band_bottom = CircularArc::new(
        circle_band_bottom,
        -3.0 * FRAC_PI_4,
        -3.0 * FRAC_PI_2,
        -FRAC_PI_4,
    );
    let band_right = LineSegment::new(Complex::ONE, Complex::new(1.0, 1.0));

    let band = ClineArcTile::new(vec![
        arc_band_top.into(),
        band_left.into(),
        arc_band_bottom.into(),
        band_right.into(),
    ]);

    let shrink = scale(1.0 / 4.0).unwrap();

    Motif::new(vec![(outside, 0), (band, 1)]).transform(shrink)
}

/// Create a skull motif that _nearly_ fits in the unit circle. the teeth
/// stick out a tiny bit.
pub fn skull() -> ClineArcTile {
    let top_circle = Circle::new(Complex::Zero, 2.0);
    let left_circle = Circle::new(-Complex::ONE, 1.0);
    let right_circle = Circle::new(Complex::ONE, 1.0);

    let arc_top = CircularArc::new(top_circle, 0.0, FRAC_PI_2, PI);
    let arc_left = CircularArc::new(left_circle, PI, 5.0 * FRAC_PI_4, 3.0 * FRAC_PI_2);
    let arc_right = CircularArc::new(right_circle, -FRAC_PI_2, -FRAC_PI_4, 0.0);

    let teeth_left = Complex::new(-1.0, -1.0);
    let teeth_spacing = Complex::from(0.5);
    let bottom_offset = -Complex::I;
    let teeth_bottom = LineSegment::new(
        teeth_left + bottom_offset,
        teeth_left + Complex::from(4.0) * teeth_spacing,
    );
    let teeth_verticals: Vec<LineSegment> = (0..5)
        .map(|i| {
            let x_offset = teeth_spacing * Complex::new(i as f64, 0.0);
            LineSegment::new(teeth_left + x_offset, teeth_left + bottom_offset + x_offset)
        })
        .collect();

    const EYE_RADIUS: f64 = 0.75;
    const EYE_X: f64 = 1.0;
    const EYE_Y: f64 = 1.0 / 3.0;
    let left_eye = Circle::new(Complex::new(-EYE_X, EYE_Y), EYE_RADIUS);
    let right_eye = Circle::new(Complex::new(EYE_X, EYE_Y), EYE_RADIUS);

    let (left_eye_top, left_eye_bottom) = circle_to_arcs(left_eye);
    let (right_eye_top, right_eye_bottom) = circle_to_arcs(right_eye);

    let skull = ClineArcTile::new(vec![
        arc_right.into(),
        arc_top.into(),
        arc_left.into(),
        teeth_bottom.into(),
        teeth_verticals[0].into(),
        teeth_verticals[1].into(),
        teeth_verticals[2].into(),
        teeth_verticals[3].into(),
        teeth_verticals[4].into(),
        left_eye_top.into(),
        left_eye_bottom.into(),
        right_eye_top.into(),
        right_eye_bottom.into(),
    ]);

    let shrink = scale(0.5).unwrap();

    skull.transform(shrink)
}

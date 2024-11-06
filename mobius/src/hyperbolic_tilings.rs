use std::f64::consts::{FRAC_PI_2, PI, TAU};

use abstraction::Group;

use crate::{
    geometry::{Circle, CircularArc, LineSegment},
    isogonal::Isogonal,
    rotation,
    transformable::ClineArcTile,
    Complex, Mobius,
};

fn compute_edge_circle(p: usize, q: usize) -> Circle {
    let angle_p = PI / (p as f64);
    let angle_q = PI / (q as f64);

    // Third mirror: circle invert in a circle with the following properties:
    // 1. The circle is centered on the real line
    // 2. The circle is orthogonal to the unit circle
    // 3. The circle intersects the line for r_conj making angle pi/q
    // it takes some algebra and trig, but you get
    //
    // center = cos(pi/q) * K
    // radius = sin(pi/p) * K
    // where K = sqrt(1 / (cos^2(pi/q) - sin^2(pi/p)))
    let cos_q = angle_q.cos();
    let sin_p = angle_p.sin();
    let k = (1.0 / (cos_q * cos_q - sin_p * sin_p)).sqrt();
    let center = cos_q * k;
    let radius = sin_p * k;

    Circle::new(Complex::from(center), radius)
}

pub fn reflection_group(p: usize, q: usize) -> Result<(Isogonal, Isogonal, Isogonal), String> {
    if p < 3 {
        return Err(String::from("p must be at least 3"));
    }

    if q < 3 {
        return Err(String::from("q must be at least 3"));
    }

    if (p - 2) * (q - 2) <= 4 {
        return Err(String::from(
            "To make a hyperbolic tiling, (p - 2)(q - 2) must be greater than 4",
        ));
    }

    // First mirror: flip over the real line
    // this mirror is the edge bisector of the polygon
    let conj = Isogonal::conj();

    // Second mirror: R * conj
    // where R is a rotation around the origin by pi/p
    // This mirror goes through the vertices of the polygon
    let rot_p = rotation(TAU / (p as f64)).unwrap();
    let r_conj = Isogonal::from(rot_p) * conj;

    // Third mirror is in an orthogonal circle that defines the edge
    // of the polygon
    let Circle { center, radius } = compute_edge_circle(p, q);

    // The circle inversion is (translate(center) * scale(radius)) sandwich (unit_circle_inversion)
    // which surprisingly simplifies nicely to:
    //
    // E2 * conj = [center (radius^2 - center^2)] * conj
    //             [1            -center        ]
    //
    // I call it E2 because it's an elliptic 2-fold rotation.
    let e2 = Mobius::from_unnormalized(
        center,
        (radius * radius - center.norm()).into(),
        Complex::ONE,
        -center,
    )
    .unwrap();
    let e2_conj = Isogonal::from(e2) * conj;

    Ok((conj, r_conj, e2_conj))
}

pub fn corner_rotation_group(p: usize, q: usize) -> Result<(Isogonal, Isogonal, Isogonal), String> {
    let (conj, r_conj, e2_conj) = reflection_group(p, q)?;

    // p-fold rotation around the center of the first polygon
    let r = r_conj * conj;

    // 2-fold elliptic rotation around the edge of the first polygon
    let e2 = e2_conj * conj;

    // The q-fold elliptic rotation around the vertex.
    // Technically this is redundant as a generator since it's just e2 * r^-1
    // but it may be handy for the caller to have.
    let eq = e2 * r.inverse();

    Ok((r, e2, eq))
}

pub fn center_edge_subgroup(p: usize, q: usize) -> Result<(Isogonal, Isogonal), String> {
    if q % 2 != 0 {
        return Err(String::from("q must be even"));
    }

    let (conj, r_conj, e2_conj) = reflection_group(p, q)?;
    let r = r_conj * conj;

    Ok((r, e2_conj))
}

pub fn bisector_vertex_subgroup(p: usize, q: usize) -> Result<(Isogonal, Isogonal), String> {
    if p % 2 != 0 {
        return Err(String::from("p must be even"));
    }

    let (conj, r_conj, e2_conj) = reflection_group(p, q)?;

    let e2 = e2_conj * conj;
    let r = r_conj * conj;
    let eq = e2 * r.inverse();

    Ok((eq, conj))
}

pub fn get_fundamental_region(
    p: usize,
    q: usize,
) -> Result<(ClineArcTile, (Complex, Complex, Complex)), String> {
    // tile center
    let center = Complex::Zero;

    // Compute the midpoint of the edge. Start at the center of the circular
    // mirror and walk left until we reach its surface
    let edge_circle = compute_edge_circle(p, q);
    let edge_midpoint = edge_circle.center - edge_circle.radius.into();

    // angle from the center of the circle to the vertex of the tile
    // determined by looking at the geometry.
    let angle_to_vertex = FRAC_PI_2 - PI / (p as f64) - PI / (q as f64);
    let suppliment = PI - angle_to_vertex;
    let vertex = edge_circle.center + Complex::from_polar(edge_circle.radius, suppliment);

    let edge_bisector = LineSegment::new(center, edge_midpoint);
    let angle_midpoint = (suppliment + PI) / 2.0;
    let edge = CircularArc::new(edge_circle, PI, angle_midpoint, suppliment);
    let angle_bisector = LineSegment::new(vertex, center);

    Ok((
        ClineArcTile::new(vec![
            edge_bisector.into(),
            edge.into(),
            angle_bisector.into(),
        ]),
        (center, edge_midpoint, vertex),
    ))
}

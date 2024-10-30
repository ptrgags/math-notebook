use std::f64::consts::PI;

use abstraction::Group;

use crate::{isogonal::Isogonal, rotation, Complex, Mobius};

pub fn reflection_group(p: usize, q: usize) -> Result<(Isogonal, Isogonal, Isogonal), String> {
    if p < 3 {
        return Err(String::from("p must be at least 3"));
    }

    if q < 3 {
        return Err(String::from("q must be at least 3"));
    }

    if (p - 2) * (q - 2) > 4 {
        return Err(String::from(
            "To make a hyperbolic tiling, (p - 2)(q - 2) must be greater than 4",
        ));
    }

    let angle_p = PI / (p as f64);
    let angle_q = PI / (q as f64);

    // First mirror: flip over the real line
    let conj = Isogonal::conj();

    // Second mirror: R * conj
    // where R is a rotation around the origin by pi/p
    // This is
    let rot_p = rotation(angle_p).unwrap();
    let r_conj = Isogonal::from(rot_p) * conj;

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

    // The circle inversion is (translate(center) * scale(radius)) sandwich (unit_circle_inversion)
    // which surprisingly simplifies nicely to:
    //
    // E2 * conj = [center (radius^2 - center^2)] * conj
    //             [1            -center        ]
    //
    // I call it E2 because it's an elliptic 2-fold rotation.
    let e2 = Mobius::new(
        center.into(),
        (radius * radius - center * center).into(),
        Complex::ONE,
        (-center).into(),
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

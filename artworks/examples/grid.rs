use core::f64;
use std::error::Error;

use abstraction::{Group, Monoid};
use mobius::{
    elliptic, hyperbolic,
    transformable::{Cline, Collection, Transformable},
};
use rendering::{render_svg, style::Style, RenderPrimitive, Renderable, View};

fn main() -> Result<(), Box<dyn Error>> {
    let h = hyperbolic(2.0).unwrap();
    let h_powers = h.power_iter().take(20);
    let h_inv_powers = h.inv_power_iter().take(20);

    let center_line = Cline::imag_axis();

    let h_clines: Vec<Cline> = h_powers
        .chain(h_inv_powers)
        .map(|x| center_line.transform(x))
        .collect();

    let yellow = Style::stroke(255, 255, 0).with_width(0.25);
    let parallels = Collection::new(h_clines).render_group(yellow)?;

    let e = elliptic(f64::consts::PI / 8.0).unwrap();
    let e_powers = e.power_iter().take(16);

    let real_axis = Cline::real_axis();

    let e_clines: Vec<Cline> = e_powers.map(|x| real_axis.transform(x)).collect();

    let red = Style::stroke(255, 0, 0).with_width(0.25);
    let meridians = Collection::new(e_clines).render_group(red)?;

    render_svg(
        "output",
        "parallels_and_meridians",
        &[View("", 0.0, 0.0, 2.0)],
        RenderPrimitive::group(vec![parallels, meridians]),
    )?;

    Ok(())
}

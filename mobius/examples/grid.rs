use core::f64;
use std::io::Error;

use abstraction::{Semigroup, Group};
use mobius::{
    cline::Cline, elliptic, hyperbolic, style::Style, svg_plot::{add_geometry, render_views, style_group, View}
};
use svg::node::element::Group as SvgGroup;

fn main() -> Result<(), Error>{
    let h = hyperbolic(2.0).unwrap();
    let h_powers = h.power_iter().take(20);
    let h_inv_powers = h.inv_power_iter().take(20);

    let center_line = Cline::imag_axis();

    let h_clines: Vec<Cline> = h_powers.chain(h_inv_powers)
        .map(|x| Cline::transform(x, center_line))
        .collect();

    let mut parallels = style_group(Style::stroke(255, 255, 0).with_width(0.25));
    parallels = add_geometry(parallels, &h_clines[..]);

    let e = elliptic(f64::consts::PI / 8.0).unwrap();
    let e_powers = e.power_iter().take(16);

    let real_axis = Cline::real_axis();

    let e_clines: Vec<Cline> = e_powers
        .map(|x| Cline::transform(x, real_axis))
        .collect();

    let mut meridians = style_group(Style::stroke(255, 0, 0).with_width(0.25));
    meridians = add_geometry(meridians, &e_clines[..]);

    let geometry = SvgGroup::new().add(parallels).add(meridians);

    render_views("output", "parallels_and_meridians", &[
        View("", 0.0, 0.0, 2.0)
    ], geometry)
}

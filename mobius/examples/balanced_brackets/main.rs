use std::{error::Error, f64::consts::PI};

use mobius::{
    cline_arc::ClineArc,
    integer_arcs::{cyclotomic_arc_by_hemisphere, integer_arc_by_hemisphere},
    rendering::Style,
    rotation,
    svg_plot::{render_views, style_geometry, union, View},
    transformable::{Cline, ClineArcTile, Transformable},
    translation, Complex,
};

mod brackets;

use brackets::{BalancedBrackets, MatchedBalancedBrackets};

pub fn main() -> Result<(), Box<dyn Error>> {
    let north_brackets = BalancedBrackets::new("[][[]][][][[[]][]]")?;
    let south_brackets = BalancedBrackets::new("[[][][]][[[]][]][]")?;
    let brackets = MatchedBalancedBrackets::new(north_brackets, south_brackets)?;

    let arcs: Result<Vec<ClineArc>, Box<dyn Error>> = brackets
        .iter()
        .map(|(a, b, hemisphere)| -> Result<ClineArc, Box<dyn Error>> {
            let arc = integer_arc_by_hemisphere(a, b, hemisphere)?;
            Ok(ClineArc::from(arc))
        })
        .collect();
    let arcs = arcs?;

    let tile = ClineArcTile::new(arcs);
    let rot90 = rotation(PI / 2.0)?;
    let radius = 0.5 * (brackets.len() as f64);
    let translate_center = translation(Complex::new(0.0, -radius)).unwrap();
    let in_view = tile.transform(translate_center * rot90);

    let yellow = Style::stroke(255, 255, 0).with_width(0.5);
    let white = Style::stroke(255, 255, 255).with_width(0.25);
    render_views(
        "output",
        "bracket_test",
        &[View("", 0.0, 0.0, radius)],
        union(vec![
            style_geometry(yellow, &in_view),
            style_geometry(white, &Cline::imag_axis()),
        ]),
    )?;

    let n = brackets.len();
    let arcs: Result<Vec<ClineArc>, Box<dyn Error>> = brackets
        .iter()
        .map(|(a, b, hemisphere)| -> Result<ClineArc, Box<dyn Error>> {
            let arc = cyclotomic_arc_by_hemisphere(a, b, n, hemisphere)?;
            Ok(ClineArc::from(arc))
        })
        .collect();
    let arcs = arcs?;
    let circle_tile = ClineArcTile::new(arcs);

    render_views(
        "output",
        "bracket_test_circle",
        &[View("", 0.0, 0.0, 2.0)],
        union(vec![
            style_geometry(yellow, &circle_tile),
            style_geometry(white, &Cline::unit_circle()),
        ]),
    )?;

    Ok(())
}

use std::{error::Error, f64::consts::PI};

use clap::Parser;
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

#[derive(Parser)]
struct Cli {
    #[arg(value_parser=BalancedBrackets::new, help="Balanced set of square brackets to create a pattern for the northern hemisphere. It must be the same length as SOUTH_BRACKETS")]
    north_brackets: BalancedBrackets,
    #[arg(value_parser=BalancedBrackets::new, help="Balanced set of square brackets to create a pattern for the southern hemisphere. It must be the same length as NORTH_BRACKETS")]
    south_brackets: BalancedBrackets,
    #[arg(
        short,
        long,
        help = "if specified, filename will be output/bracket_<line|circle>_<suffix>.svg"
    )]
    suffix: Option<String>,
    #[arg(short, long, action, help = "if set, draw the equator for reference")]
    equator: bool,
}

pub fn render_line(suffix: &str, brackets: &MatchedBalancedBrackets) -> Result<(), Box<dyn Error>> {
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
        &format!("brackets_line{}", suffix),
        &[View("", 0.0, 0.0, radius)],
        union(vec![
            style_geometry(yellow, &in_view),
            style_geometry(white, &Cline::imag_axis()),
        ]),
    )?;

    Ok(())
}

pub fn render_circle(
    suffix: &str,
    brackets: &MatchedBalancedBrackets,
) -> Result<(), Box<dyn Error>> {
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

    let yellow = Style::stroke(255, 255, 0).with_width(0.5);
    let white = Style::stroke(255, 255, 255).with_width(0.25);

    render_views(
        "output",
        &format!("brackets_circle{}", suffix),
        &[View("", 0.0, 0.0, 2.0)],
        union(vec![
            style_geometry(yellow, &circle_tile),
            style_geometry(white, &Cline::unit_circle()),
        ]),
    )?;

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let brackets = MatchedBalancedBrackets::new(cli.north_brackets, cli.south_brackets)?;

    let suffix = cli.suffix.map_or("".into(), |x| format!("_{}", x));

    render_line(&suffix, &brackets)?;
    render_circle(&suffix, &brackets)?;

    Ok(())
}

use std::error::Error;
use std::str::FromStr;

use clap::Parser;
use mobius::{
    cline_arc::ClineArc,
    geometry::integer_arcs::{arc_on_circle_by_hemisphere, arc_on_line_by_hemisphere, Hemisphere},
    transformable::ClineArcTile,
};
use permutations::{DisjointCycles, Permutation};
use rendering::{render_svg, style::Style, Renderable, View};

type BigPermutation = Permutation<50>;

#[derive(Parser)]
struct Cli {
    #[arg(value_parser=BigPermutation::from_str, help="Permutation in cycle notation with at most 50 elements. e.g. (1 2 3)(4 5)")]
    permutation: BigPermutation,
}

fn max_element(cycles: &[Vec<usize>]) -> usize {
    let mut result = 0usize;
    for cycle in cycles {
        for &element in cycle {
            result = result.max(element)
        }
    }
    result
}

fn make_pairs(cycle: &[usize]) -> Vec<(usize, usize)> {
    let n = cycle.len();
    // Ignore empty cycles and short cycles
    if n < 2 {
        return vec![];
    }

    cycle
        .iter()
        .enumerate()
        .map(|(i, &a)| (a, cycle[(i + 1) % n]))
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let DisjointCycles(cycles) = cli.permutation.cycle_decomposition();
    let n = max_element(&cycles) + 1;

    let indices: Vec<(usize, usize)> = cycles.iter().flat_map(|cycle| make_pairs(cycle)).collect();

    let arcs: Vec<ClineArc> = indices
        .iter()
        .enumerate()
        .map(|(i, &(a, b))| {
            let hemisphere = if i % 2 == 0 {
                Hemisphere::North
            } else {
                Hemisphere::South
            };
            let arc = arc_on_line_by_hemisphere(a as i64, b as i64, hemisphere).unwrap();

            ClineArc::from(arc)
        })
        .collect();
    let tile = ClineArcTile::new(arcs);

    let green = Style::stroke(0, 255, 0).with_width(0.5);

    render_svg(
        "output",
        "perm_arcs_line",
        &[View("", 0.5 * (n as f64), 0.0, 0.5 * (n as f64))],
        tile.render_group(green)?,
    )?;

    let arcs: Vec<ClineArc> = indices
        .iter()
        .enumerate()
        .map(|(i, &(a, b))| {
            let hemisphere = if i % 2 == 0 {
                Hemisphere::North
            } else {
                Hemisphere::South
            };
            let arc = arc_on_circle_by_hemisphere(a, b, n, hemisphere).unwrap();

            ClineArc::from(arc)
        })
        .collect();
    let tile = ClineArcTile::new(arcs);

    let green = Style::stroke(0, 255, 0).with_width(0.5);

    render_svg(
        "output",
        "perm_arcs_circle",
        &[View("", 0.0, 0.0, 2.0)],
        tile.render_group(green)?,
    )?;

    Ok(())
}

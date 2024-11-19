use std::error::Error;

use clap::Parser;
use permutations::Permutation;

type BigPermutation = Permutation<100>;

#[derive(Parser)]
struct Cli {
    #[arg(value_parser=parse_cycles::<100>, help="Permutation in cycle notation. e.g. (1 2 3)(4 5)")]
    permutation: BigPermutation,
}

fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}

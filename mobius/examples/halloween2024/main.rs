mod artworks;

use std::error::Error;

use artworks::{
    bone_tree, candy_corners, ghost_double_spiral, ghost_gasket, ghost_octahedral, hex_grid,
    rib_cage, warpedpaper,
};
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Command {
    CandyCorners,
    HexGrid,
    BoneTree,
    RibCage,
    GhostOctahedral,
    GhostDoubleSpiral,
    GhostGasket,
    Warpedpaper,
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

pub fn run_all() -> Result<(), Box<dyn Error>> {
    println!("Candy Corners ======");
    candy_corners()?;
    println!("Hex Grid ======");
    hex_grid()?;
    println!("Bone Tree ======");
    bone_tree()?;
    println!("Rib Cage ======");
    rib_cage()?;
    println!("Ghost Octahedral ====");
    ghost_octahedral()?;
    println!("Ghost Double Spiral ====");
    ghost_double_spiral()?;
    println!("Ghost Gasket ====");
    ghost_gasket()?;
    println!("Warpedpaper ====");
    warpedpaper()?;

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    use Command::*;
    if let Some(cmd) = cli.command {
        match cmd {
            CandyCorners => candy_corners(),
            HexGrid => hex_grid(),
            BoneTree => bone_tree(),
            RibCage => rib_cage(),
            GhostOctahedral => ghost_octahedral(),
            GhostDoubleSpiral => ghost_double_spiral(),
            GhostGasket => ghost_gasket(),
            Warpedpaper => warpedpaper(),
        }
    } else {
        run_all()
    }
}

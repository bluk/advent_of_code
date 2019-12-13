use std::io::{self};

use aoc_2019_13::{
    self,
    arcade::{Tile, Type},
    error::Error,
    intcode::Prog,
};

fn main() -> Result<(), Error> {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input)?;
    let mem_state = aoc_2019_13::intcode::parse_mem_state(&input)?;

    let prog = Prog::new(&mem_state);
    let tiles = aoc_2019_13::arcade::run_loop(prog)?;

    println!(
        "{}",
        tiles
            .into_iter()
            .filter(|t| t.tile_id == Type::Block)
            .collect::<Vec<Tile>>()
            .len()
    );

    Ok(())
}

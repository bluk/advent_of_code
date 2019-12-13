use std::io::{self};

use aoc_2019_13::{self, error::Error, intcode::Prog};

fn main() -> Result<(), Error> {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input)?;
    let mut mem_state = aoc_2019_13::intcode::parse_mem_state(&input)?;
    mem_state[0] = 2;

    let prog = Prog::new(&mem_state);
    let score = aoc_2019_13::arcade::run_loop(prog)?;

    println!("{}", score);

    Ok(())
}

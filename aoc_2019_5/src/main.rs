use std::io::{self};

use aoc_2019_5::{self, error::Error, StdInProgInput, StdOutProgOutput};

fn main() -> Result<(), Error> {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input)?;
    let mem_state = aoc_2019_5::parse_mem_state(&input)?;
    let _ = aoc_2019_5::run_prog(
        &mem_state,
        StdInProgInput::new(),
        &mut StdOutProgOutput::new(),
    )?;

    Ok(())
}

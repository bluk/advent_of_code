use std::io::{self};

use aoc_2019_9::{self, error::Error, Prog, StdInProgInput, StdOutProgOutput};

fn main() -> Result<(), Error> {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input)?;
    let mem_state = aoc_2019_9::parse_mem_state(&input)?;

    let mut prog = Prog::new(&mem_state);
    prog.run(&mut StdInProgInput::new(), &mut StdOutProgOutput::new())?;

    Ok(())
}

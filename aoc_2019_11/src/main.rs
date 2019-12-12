use std::io::{self};

use aoc_2019_11::{self, error::Error, Prog};

fn main() -> Result<(), Error> {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input)?;
    let mem_state = aoc_2019_11::parse_mem_state(&input)?;

    let prog = Prog::new(&mem_state);
    let panels = aoc_2019_11::paint_hull(prog)?;
    aoc_2019_11::display_panels(panels);

    Ok(())
}

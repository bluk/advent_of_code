use std::io::{self};

use aoc_2019_7::{self, error::Error};

fn main() -> Result<(), Error> {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input)?;
    let mem_state = aoc_2019_7::parse_mem_state(&input)?;

    let result = aoc_2019_7::find_max_thrust_signal(&mem_state)?;
    println!("{result:?}");

    let result = aoc_2019_7::find_max_thrust_signal_in_feedback_loop(&mem_state)?;
    println!("{result:?}");

    Ok(())
}

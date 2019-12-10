use std::io::prelude::*;
use std::io::{self};

use aoc_2019_10::{self, error::Error};

fn main() -> Result<(), Error> {
    let input = io::stdin()
        .lock()
        .lines()
        .collect::<Result<Vec<String>, io::Error>>()?
        .join("\n");
    let map = aoc_2019_10::build_map(&input)?;

    let (pos, count) = aoc_2019_10::find_best_monitoring_pos(&map)?.unwrap();
    println!("{:?} - {}", pos, count);

    Ok(())
}

use num::Integer;
use std::io::{self};

use aoc_2019_12::{self, error::Error, Moon};

fn main() -> Result<(), Error> {
    let mut moons: Vec<Moon> = vec![];
    loop {
        let mut input = String::new();
        let read = io::stdin().read_line(&mut input)?;

        if read == 0 {
            break;
        }

        let moon = aoc_2019_12::parse_init_moon(&input)?;
        moons.push(moon);
    }

    let x_cycle = aoc_2019_12::find_cycle(moons.iter().map(|m| m.pos.x).collect());
    let y_cycle = aoc_2019_12::find_cycle(moons.iter().map(|m| m.pos.y).collect());
    let z_cycle = aoc_2019_12::find_cycle(moons.iter().map(|m| m.pos.z).collect());

    let min_cycle = x_cycle.lcm(&y_cycle).lcm(&z_cycle);

    println!("{}", min_cycle);

    Ok(())
}

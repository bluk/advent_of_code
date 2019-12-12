use std::io::{self};

use aoc_2019_12::{self, error::Error, Moon, TimeStepIter};

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

    let iter = TimeStepIter::new(moons);
    let mut iter = iter.skip(999);
    let moons = iter.next().unwrap();
    println!("{}", aoc_2019_12::total_energy(&moons));

    Ok(())
}

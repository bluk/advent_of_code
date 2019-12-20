use std::collections::HashMap;
use std::io::{self};

use aoc_2019_14::{self, error::Error};

fn main() -> Result<(), Error> {
    let mut reactions = HashMap::new();

    loop {
        let mut input = String::new();
        let read = io::stdin().read_line(&mut input)?;
        if read == 0 {
            break;
        }

        let (output_chem, output_qty, input_chems) = aoc_2019_14::parse_line(&input)?;
        reactions.insert(output_chem, (output_qty, input_chems));
    }

    let ore_count = aoc_2019_14::find_ore_for_fuel(&reactions)?;
    println!("{}", ore_count);

    Ok(())
}

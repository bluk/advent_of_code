use std::io::{self};

use aoc_2019_8::{self, error::Error, SpaceImg};

fn main() -> Result<(), Error> {
    let mut input = String::new();
    let bytes_read = io::stdin().read_line(&mut input)?;
    assert!(bytes_read > 0);
    let input = aoc_2019_8::parse_input(&input)?;
    let img = SpaceImg::new(input, 25, 6);
    println!("{}", img.verify()?);

    Ok(())
}

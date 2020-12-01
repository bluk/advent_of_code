use std::io::{self, BufRead};

#[derive(Debug)]
enum Error {
    ParseError,
    InvalidInput,
}

fn find_sum_2020(input: &[u32]) -> Result<(u32, u32), Error> {
    let mut remaining_data = input;
    loop {
        if let Some((first, remainder)) = remaining_data.split_first() {
            remaining_data = remainder;

            for r in remaining_data {
                if first + r == 2020 {
                    return Ok((*first, *r));
                }
            }
        } else {
            return Err(Error::InvalidInput);
        }
    }
}

fn main() -> Result<(), Error> {
    let stdin = io::stdin();
    let mut input: Vec<u32> = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        input.push(line.parse().map_err(|_| Error::ParseError)?)
    }

    if let Ok((first, second)) = find_sum_2020(&input) {
        println!("First Part Answer: {}", first * second);
    }

    Ok(())
}

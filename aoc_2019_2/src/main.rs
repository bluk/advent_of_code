use std::io::{self};

use aoc_2019_2;

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => match aoc_2019_2::parse_input(&input) {
            Ok(mut prog) => {
                prog[1] = 12;
                prog[2] = 2;

                let output = aoc_2019_2::run_prog(&prog);
                println!("{}", output[0]);
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        },
        Err(e) => {
            eprintln!("error: {}", e);
        }
    }
}

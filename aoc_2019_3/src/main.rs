use std::io::{self};

use aoc_2019_3;

fn main() {
    let mut input1 = String::new();
    match io::stdin().read_line(&mut input1) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    };

    let mut input2 = String::new();
    match io::stdin().read_line(&mut input2) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    };

    match aoc_2019_3::find_fewest_steps(&input1, &input2) {
        Ok(fewest_steps) => {
            if let Some(fewest_steps) = fewest_steps {
                println!("{}", fewest_steps);
            } else {
                eprintln!("could not find the fewest steps");
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    }
}

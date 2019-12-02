use std::io::{self};

use aoc_2019_2;

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => match aoc_2019_2::parse_input(&input) {
            Ok(mut prog) => {
                'end: for noun in 0..=99 {
                    for verb in 0..=99 {
                        prog[1] = noun;
                        prog[2] = verb;

                        let output = aoc_2019_2::run_prog(&prog);
                        if output[0] == 19690720 {
                            break 'end;
                        }
                    }
                }

                println!("{}", 100 * prog[1] + prog[2]);
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

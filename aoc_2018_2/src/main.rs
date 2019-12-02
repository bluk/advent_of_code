use std::collections::HashMap;
use std::io::{self};

fn main() {
    let mut contains_two_total = 0;
    let mut contains_three_total = 0;

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    break;
                }

                let mut chars = HashMap::new();
                for c in input.trim().chars() {
                    *chars.entry(c).or_insert(0) += 1;
                }

                let (contains_two, contains_three) =
                    chars.values().fold((false, false), |result, v| {
                        if *v == 2 {
                            (true, result.1)
                        } else if *v == 3 {
                            (result.0, true)
                        } else {
                            result
                        }
                    });

                if contains_two {
                    contains_two_total += 1;
                }

                if contains_three {
                    contains_three_total += 1;
                }
            }
            Err(e) => {
                eprintln!("error: {}", e);
                break;
            }
        }
    }

    println!("{}", contains_two_total * contains_three_total);
}

use std::io::{self};

use aoc_2019_1::find_fuel_required;

fn main() {
    let mut total_fuel_req: u32 = 0;
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    break;
                }

                match input.trim().parse::<u32>() {
                    Ok(mass) => {
                        total_fuel_req += find_fuel_required(mass);
                    }
                    Err(e) => {
                        eprintln!("input: {input}, error: {e}");
                        break;
                    }
                };
            }
            Err(e) => {
                eprintln!("error: {e}");
                break;
            }
        }
    }
    println!("{total_fuel_req}");
}

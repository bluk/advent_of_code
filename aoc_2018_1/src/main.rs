use std::collections::BTreeSet;
use std::io::{self};

fn main() {
    let mut freq: i64 = 0;
    let mut freq_hist = BTreeSet::<i64>::new();
    let mut freq_change_input = Vec::<i64>::new();

    freq_hist.insert(freq);

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    break;
                }

                match input.trim().parse::<i64>() {
                    Ok(change) => {
                        freq_change_input.push(change);
                    }
                    Err(e) => {
                        eprintln!("input: {}, error: {}", input, e);
                        break;
                    }
                };
            }
            Err(e) => {
                eprintln!("error: {}", e);
                break;
            }
        }
    }

    'quit: loop {
        for change in freq_change_input.iter() {
            freq += change;

            if freq_hist.contains(&freq) {
                break 'quit;
            }

            freq_hist.insert(freq);
        }
    }

    println!("{}", freq);
}

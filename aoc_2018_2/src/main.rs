use std::io::{self};

fn find_off_by(a: &str, b: &str) -> usize {
    let mut off_by = 0;
    if a.len() != b.len() {
        if a.len() > b.len() {
            off_by = a.len() - b.len();
        } else {
            off_by = b.len() - a.len();
        }
    }

    let chars = a.chars().zip(b.chars());
    for (a, b) in chars {
        if a != b {
            off_by += 1;
        }
    }

    off_by
}

fn common_chars(a: &str, b: &str) -> String {
    let mut ret = String::new();

    let chars = a.chars().zip(b.chars());
    for (a, b) in chars {
        if a == b {
            ret.push(a);
        }
    }

    ret
}

fn main() {
    let mut ids = Vec::<String>::new();

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    break;
                }

                for i in &ids {
                    let off_by_count = find_off_by(i, &input);
                    if off_by_count <= 1 {
                        println!("{}", common_chars(i, &input));
                        return;
                    }
                }

                ids.push(input);
            }
            Err(e) => {
                eprintln!("error: {e}");
                break;
            }
        }
    }
}

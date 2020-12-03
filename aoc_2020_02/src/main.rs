use std::io::{self, BufRead};

fn main() {
    let mut valid_count = 0;

    for line in io::stdin().lock().lines() {
        let line = line.expect("Line to be readable");
        let mut space_split = line.split(" ");

        match (space_split.next(), space_split.next(), space_split.next()) {
            (Some(count), Some(ch), Some(password)) => {
                let mut split_count = count.split("-");
                let (min_range, max_range): (usize, usize) =
                    match (split_count.next(), split_count.next()) {
                        (Some(min), Some(max)) => (
                            min.parse().expect("a number"),
                            max.parse().expect("a number"),
                        ),
                        _ => panic!("unexpected range"),
                    };
                let ch = ch.chars().next().unwrap();

                let count = password.chars().filter(|c| *c == ch).count();
                if (min_range..=max_range).contains(&count) {
                    valid_count += 1;
                }
            }
            _ => panic!("Unexpected input"),
        }
    }

    println!("Valid Password Count: {}", valid_count);
}

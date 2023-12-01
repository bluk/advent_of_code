use std::io::{self, BufRead};

fn main() {
    let mut valid_count = 0;

    for line in io::stdin().lock().lines() {
        let line = line.expect("Line to be readable");
        let mut space_split = line.split(' ');

        match (space_split.next(), space_split.next(), space_split.next()) {
            (Some(count), Some(ch), Some(password)) => {
                let mut split_count = count.split('-');
                let (first_pos, second_pos): (usize, usize) =
                    match (split_count.next(), split_count.next()) {
                        (Some(min), Some(max)) => (
                            min.parse().expect("a number"),
                            max.parse().expect("a number"),
                        ),
                        _ => panic!("unexpected range"),
                    };
                let ch = ch.chars().next().unwrap();

                let password = password.chars();
                let mut password = password.skip(first_pos - 1);
                let first = password.next();
                let mut password = password.skip(second_pos - first_pos - 1);
                let second = password.next();
                match (first, second) {
                    (Some(first), Some(second))
                        if (first == ch && second != ch) || (first != ch && second == ch) =>
                    {
                        valid_count += 1;
                    }
                    (Some(first), None) if first == ch => valid_count += 1,
                    (None, Some(second)) if second == ch => valid_count += 1,
                    _ => {}
                };
            }
            _ => panic!("Unexpected input"),
        }
    }

    println!("Valid Password Count: {valid_count}");
}

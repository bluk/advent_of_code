use std::io;

fn find_digit(line: &str) -> Option<(usize, u8)> {
    line.chars()
        .enumerate()
        .find(|(_, ch)| ch.is_ascii_digit())
        .map(|(pos, ch)| (pos, ch as u8 - b'0'))
}

fn find_digit_rev(line: &str) -> Option<(usize, u8)> {
    line.chars()
        .rev()
        .enumerate()
        .find(|(_, ch)| ch.is_ascii_digit())
        .map(|(pos, ch)| (line.chars().count() - pos, ch as u8 - b'0'))
}

const NUMBER_WORDS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[allow(clippy::cast_possible_truncation)]
fn find_word(line: &str) -> Option<(usize, u8)> {
    let mut found: Option<(usize, u8)> = None;
    for (word_pos, word) in NUMBER_WORDS.iter().enumerate() {
        if let Some(found_pos) = line.match_indices(word).next() {
            if let Some(existing) = found {
                if found_pos.0 < existing.0 {
                    found = Some((found_pos.0, (word_pos + 1) as u8));
                }
            } else {
                found = Some((found_pos.0, (word_pos + 1) as u8));
            }
        }
    }

    found
}

#[allow(clippy::cast_possible_truncation)]
fn find_word_rev(line: &str) -> Option<(usize, u8)> {
    let mut found: Option<(usize, u8)> = None;
    for (word_pos, word) in NUMBER_WORDS.iter().enumerate() {
        if let Some(found_pos) = line.rmatch_indices(word).next() {
            if let Some(existing) = found {
                if found_pos.0 > existing.0 {
                    found = Some((found_pos.0, (word_pos + 1) as u8));
                }
            } else {
                found = Some((found_pos.0, (word_pos + 1) as u8));
            }
        }
    }

    found
}

fn find_value(line: &str) -> u8 {
    match (find_digit(line), find_word(line)) {
        (Some(v_0), Some(v_1)) => {
            if v_0.0 < v_1.0 {
                v_0.1
            } else {
                v_1.1
            }
        }
        (Some(v), None) | (None, Some(v)) => v.1,
        (None, None) => unreachable!(),
    }
}

fn find_value_rev(line: &str) -> u8 {
    match (find_digit_rev(line), find_word_rev(line)) {
        (Some(v_0), Some(v_1)) => {
            if v_0.0 > v_1.0 {
                v_0.1
            } else {
                v_1.1
            }
        }
        (Some(v), None) | (None, Some(v)) => v.1,
        (None, None) => unreachable!(),
    }
}

fn main() -> io::Result<()> {
    let total = itertools::process_results(io::stdin().lines(), |lines| {
        let mut total: u64 = 0;
        for line in lines {
            let first_digit = find_value(&line);
            let second_digit = find_value_rev(&line);

            let subtotal = u64::from(first_digit * 10 + second_digit);

            // eprintln!("{line} - {first_digit} - {second_digit} - {subtotal}");

            total += subtotal;
        }

        total
    })?;

    eprintln!("{total}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::find_word_rev;

    #[test]
    fn test_word_rev() {
        let input = "five3mzqgxnbmdbcmnteightsixtc";
        let value = find_word_rev(input).unwrap();
        assert_eq!(6, value.1);
    }
}

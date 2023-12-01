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
        .map(|(pos, ch)| (pos, ch as u8 - b'0'))
}

fn main() -> io::Result<()> {
    let total = itertools::process_results(io::stdin().lines(), |lines| {
        let mut total: u64 = 0;
        for line in lines {
            let first_digit = find_digit(&line).unwrap().1;
            let second_digit = find_digit_rev(&line).unwrap().1;

            let subtotal = u64::from(first_digit * 10 + second_digit);

            // eprintln!("{line} - {first_digit} - {second_digit} - {subtotal}");

            total += subtotal;
        }

        total
    })?;

    eprintln!("{total}");

    Ok(())
}

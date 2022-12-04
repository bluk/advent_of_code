use std::{io, ops::RangeInclusive};

fn parse_range(s: &str) -> io::Result<RangeInclusive<u32>> {
    let (first, second) = s
        .split_once('-')
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "invalid range"))?;

    let first = first
        .parse::<u32>()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let second = second
        .parse::<u32>()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(first..=second)
}

fn main() -> io::Result<()> {
    let count = itertools::process_results(io::stdin().lines(), |lines| {
        itertools::process_results(
            lines.map(|line| {
                let (first, second) = line
                    .split_once(',')
                    .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "invalid ranges"))?;

                let first = parse_range(first)?;
                let second = parse_range(second)?;

                Ok::<_, io::Error>((first, second))
            }),
            |it| {
                it.map(|(first, second)| {
                    let first_contains_second =
                        first.start() <= second.start() && first.end() >= second.end();
                    let second_contains_first =
                        second.start() <= first.start() && second.end() >= first.end();

                    first_contains_second || second_contains_first
                })
                .map(u32::from)
                .sum::<u32>()
            },
        )
    })??;

    println!("{count}");

    Ok(())
}

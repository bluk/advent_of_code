use itertools::Itertools as _;
use std::io;

type PriorityTy = u32;

fn priority(ch: char) -> io::Result<PriorityTy> {
    match ch {
        'a'..='z' => Ok(ch as u32 - 'a' as u32 + 1),
        'A'..='Z' => Ok(ch as u32 - 'A' as u32 + 1 + 26),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid item type",
        )),
    }
}

fn parse_group(line1: &str, line2: &str, line3: &str) -> io::Result<PriorityTy> {
    line1
        .chars()
        .find(|needle| {
            line2.chars().any(|ch| *needle == ch) && line3.chars().any(|ch| *needle == ch)
        })
        .map_or_else(
            || {
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "missing duplicate item type",
                ))
            },
            priority,
        )
}

fn main() -> io::Result<()> {
    let sum = itertools::process_results(io::stdin().lines(), |it| {
        it.batching(|it| Some(parse_group(&it.next()?, &it.next()?, &it.next()?)))
            .try_fold(0, |acc, priority| priority.map(|priority| acc + priority))
    })??;

    println!("{sum}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn priority_lowercase() -> io::Result<()> {
        assert_eq!(1, priority('a')?);
        assert_eq!(3, priority('c')?);
        assert_eq!(26, priority('z')?);
        Ok(())
    }

    #[test]
    fn priority_uppercase() -> io::Result<()> {
        assert_eq!(27, priority('A')?);
        assert_eq!(29, priority('C')?);
        assert_eq!(52, priority('Z')?);
        Ok(())
    }
}

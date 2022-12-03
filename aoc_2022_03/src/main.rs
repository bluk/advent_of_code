use std::io;

type PriorityTy = u32;

fn priority(ch: char) -> io::Result<PriorityTy> {
    match ch {
        'a'..='z' => Ok(ch as u32 - 'a' as u32 + 1),
        'A'..='Z' => Ok(ch as u32 - 'A' as u32 + 1 + 26),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "missing duplicate item type",
        )),
    }
}

fn parse_line(line: &str) -> io::Result<PriorityTy> {
    let compartment_len = line.len() / 2;
    let (first, second) = line.split_at(compartment_len);
    first
        .chars()
        .find(|c| second.chars().any(|c2| *c == c2))
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
    let sum = io::stdin()
        .lines()
        .map(|line| line.and_then(|line| parse_line(&line)))
        .try_fold(0, |acc, priority| priority.map(|priority| acc + priority))?;

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

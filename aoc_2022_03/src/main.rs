use itertools::Itertools as _;
use std::collections::HashSet;
use std::io;

type PriorityTy = u32;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct ItemTy(char);

impl ItemTy {
    fn priority(&self) -> PriorityTy {
        match self.0 {
            ch if ch.is_ascii_lowercase() => ch as u32 - 'a' as u32 + 1,
            ch if ch.is_ascii_uppercase() => ch as u32 - 'A' as u32 + 1 + 26,
            _ => unreachable!("unknown item type"),
        }
    }
}

impl TryFrom<char> for ItemTy {
    type Error = io::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        value
            .is_ascii_alphabetic()
            .then_some(ItemTy(value))
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "invalid item type"))
    }
}

// Debatable whether this is actually any better than the earlier versions (see git history)
//
// - Use itertools as much as possible
// - Wanted to see how the shape of the code would look like if there were no explicit panics/unwraps and errors were "handled"
// - Try out Iterator::reduce with HashSet::intersection (see https://fasterthanli.me/series/advent-of-code-2022/part-3)

fn main() -> io::Result<()> {
    let sum = itertools::process_results(io::stdin().lines(), |it| {
        // Assume the number of lines in the input is a multiple of 3
        it.chunks(3)
            .into_iter()
            .map(|chunks| {
                itertools::process_results(
                    chunks.map(|s| {
                        s.chars()
                            .map(ItemTy::try_from)
                            .collect::<Result<HashSet<ItemTy>, _>>()
                    }),
                    |it| {
                        it.reduce(|a, b| a.intersection(&b).cloned().collect::<HashSet<_>>())
                            .ok_or_else(|| {
                                io::Error::new(io::ErrorKind::InvalidData, "missing input chunk")
                            })
                            .and_then(|dupes| {
                                dupes
                                    .into_iter()
                                    .next()
                                    .as_ref()
                                    .map(ItemTy::priority)
                                    .ok_or_else(|| {
                                        io::Error::new(
                                            io::ErrorKind::InvalidData,
                                            "missing duplicate item type",
                                        )
                                    })
                            })
                    },
                )?
            })
            .sum::<Result<PriorityTy, _>>()
    })??;

    println!("{sum}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn priority_lowercase() {
        assert_eq!(1, ItemTy('a').priority());
        assert_eq!(3, ItemTy('c').priority());
        assert_eq!(26, ItemTy('z').priority());
    }

    #[test]
    fn priority_uppercase() {
        assert_eq!(27, ItemTy('A').priority());
        assert_eq!(29, ItemTy('c').priority());
        assert_eq!(52, ItemTy('Z').priority());
    }
}

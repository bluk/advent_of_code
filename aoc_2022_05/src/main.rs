use itertools::Itertools as _;
use std::io;

type Cargo = char;

type CargoStacks = Vec<Vec<Cargo>>;

#[derive(Debug)]
struct MoveCmd {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_crate_line(s: &str, stacks: &mut CargoStacks) {
    for (pos, mut chunk) in s.chars().chunks(4).into_iter().enumerate() {
        if let Some('[') = chunk.next() {
            let item = chunk.next().expect("item identifier does not exist");
            if let Some(stack) = stacks.get_mut(pos) {
                stack.push(item);
            } else {
                stacks.resize(pos + 1, Vec::default());
                stacks[pos].push(item);
            }
            assert_eq!(chunk.next(), Some(']'));
        }
    }
}

fn parse_cmd(s: &str) -> io::Result<MoveCmd> {
    let mut words = s.split_whitespace();

    match (
        words.next(),
        words.next(),
        words.next(),
        words.next(),
        words.next(),
        words.next(),
    ) {
        (Some("move"), Some(count), Some("from"), Some(from), Some("to"), Some(to)) => {
            let count = count
                .parse()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
            let from = from
                .parse()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
            let to = to
                .parse()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
            Ok(MoveCmd { count, from, to })
        }
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "invalid command",
        )),
    }
}

fn main() -> io::Result<()> {
    let mut lines = io::stdin().lines();
    let mut stacks = itertools::process_results(&mut lines, |it| {
        let mut stacks = CargoStacks::default();
        for line in it.take_while(|l| !l.is_empty()) {
            parse_crate_line(&line, &mut stacks);
        }

        stacks
    })?;

    for stack in &mut stacks {
        stack.reverse();
    }

    itertools::process_results(&mut lines, |it| {
        for line in it {
            let cmd = parse_cmd(&line)?;

            for _ in 0..cmd.count {
                let item = stacks[cmd.from - 1].pop().unwrap();
                stacks[cmd.to - 1].push(item);
            }
        }

        Ok::<_, io::Error>(())
    })??;

    let tops = stacks
        .iter()
        .map(|stack| stack.last().unwrap_or(&' '))
        .join("");

    println!("{tops}");

    Ok(())
}

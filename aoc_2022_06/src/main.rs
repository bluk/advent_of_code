use itertools::Itertools as _;
use std::{
    collections::VecDeque,
    io::{self, Read},
};

const START_MSG_DISTINCT_CHARS_COUNT: usize = 14;

fn main() -> io::Result<()> {
    let mut offset = START_MSG_DISTINCT_CHARS_COUNT;

    let mut stream: VecDeque<u8> = VecDeque::default();

    let mut buf = [0; 4096];
    let mut stdin = io::stdin();

    loop {
        let read = stdin.read(&mut buf)?;
        if read == 0 {
            break;
        }

        stream.extend(&buf[..read]);

        while stream.len() >= START_MSG_DISTINCT_CHARS_COUNT {
            if stream
                .iter()
                .take(START_MSG_DISTINCT_CHARS_COUNT)
                .unique()
                .count()
                == START_MSG_DISTINCT_CHARS_COUNT
            {
                break;
            }

            stream.pop_front();

            offset += 1;
        }
    }

    println!("{offset}");

    Ok(())
}

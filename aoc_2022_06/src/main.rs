use itertools::Itertools as _;
use std::{
    collections::VecDeque,
    io::{self, Read},
};

fn main() -> io::Result<()> {
    let mut offset = 4;

    let mut stream: VecDeque<u8> = VecDeque::default();

    let mut buf = [0; 4096];
    let mut stdin = io::stdin();

    loop {
        let read = stdin.read(&mut buf)?;
        if read == 0 {
            break;
        }

        stream.extend(&buf[..read]);

        for (a, b, c, d) in stream.iter().tuple_windows() {
            let set = [a, b, c, d];
            if set.iter().unique().count() == set.len() {
                break;
            }
            offset += 1;
        }

        if stream.len() > 4 {
            stream.drain(..stream.len() - 3);
        }
    }

    println!("{offset}");

    Ok(())
}

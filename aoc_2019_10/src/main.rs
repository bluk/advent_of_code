use std::io::prelude::*;
use std::io::{self};

use aoc_2019_10::{self, error::Error, VaporizeIter};

fn main() -> Result<(), Error> {
    let input = io::stdin()
        .lock()
        .lines()
        .collect::<Result<Vec<String>, io::Error>>()?
        .join("\n");
    let map = aoc_2019_10::build_map(&input)?;

    let (pos, count) = aoc_2019_10::find_best_monitoring_pos(&map)?.unwrap();
    println!("{:?} - {}", pos, count);

    let iter = VaporizeIter::new(&pos, &map);
    let mut iter = iter.skip(199);
    let asteriod200 = iter.next().unwrap();
    println!("200th asteriod vaporized: {:?}", asteriod200);
    println!("{}", asteriod200.x * 100 + asteriod200.y);

    Ok(())
}

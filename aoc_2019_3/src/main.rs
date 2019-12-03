use std::collections::BTreeSet;
use std::io::{self};

use aoc_2019_3;

fn main() {
    let mut input = String::new();
    let moves1 = match io::stdin().read_line(&mut input) {
        Ok(_) => match aoc_2019_3::parse_moves(&input) {
            Ok(moves) => moves,
            Err(e) => {
                eprintln!("error: {}", e);
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    };

    let mut input = String::new();
    let moves2 = match io::stdin().read_line(&mut input) {
        Ok(_) => match aoc_2019_3::parse_moves(&input) {
            Ok(moves) => moves,
            Err(e) => {
                eprintln!("error: {}", e);
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    };

    let visited_pos1 = aoc_2019_3::visited_pos(&moves1);
    let visited_pos2 = aoc_2019_3::visited_pos(&moves2);
    let intersection: BTreeSet<_> = visited_pos1.intersection(&visited_pos2).cloned().collect();
    let closest_dist = intersection.iter().map(|p| p.manhattan_dist()).min();

    if let Some(closest_dist) = closest_dist {
        println!("{}", closest_dist);
    } else {
        eprintln!(
            "could not find intersection between {:?} and {:?}",
            visited_pos1, visited_pos2
        );
        std::process::exit(1);
    }
}

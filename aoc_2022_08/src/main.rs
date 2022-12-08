use itertools::{FoldWhile, Itertools as _};
use std::{cmp::Ordering, io};

fn is_visible_from_edge(x: usize, y: usize, grid: &[Vec<u32>]) -> bool {
    let row = &grid[y];
    let min_height = row[x];

    // Left
    if !row.iter().take(x).any(|h| *h >= min_height) {
        return false;
    }
    // Right
    if !row.iter().skip(x + 1).any(|h| *h >= min_height) {
        return false;
    }
    // Up
    if !grid.iter().take(y).any(|row| row[x] >= min_height) {
        return false;
    }
    // Down
    if !grid.iter().skip(y + 1).any(|row| row[x] >= min_height) {
        return false;
    }

    true
}

fn scenic_score(x: usize, y: usize, grid: &[Vec<u32>]) -> u32 {
    let row = &grid[y];
    let min_height = row[x];

    // Left
    let left_score = row
        .iter()
        .rev()
        .skip(row.len() - x)
        .fold_while(0, |acc, h| match h.cmp(&min_height) {
            Ordering::Greater | Ordering::Equal => FoldWhile::Done(acc + 1),
            Ordering::Less => FoldWhile::Continue(acc + 1),
        })
        .into_inner();

    // Right
    let right_score = row
        .iter()
        .skip(x + 1)
        .fold_while(0, |acc, h| match h.cmp(&min_height) {
            Ordering::Greater | Ordering::Equal => FoldWhile::Done(acc + 1),
            Ordering::Less => FoldWhile::Continue(acc + 1),
        })
        .into_inner();

    // Up
    let up_score = grid
        .iter()
        .rev()
        .skip(grid.len() - y)
        .fold_while(0, |acc, row| {
            let h = row[x];
            match h.cmp(&min_height) {
                Ordering::Greater | Ordering::Equal => FoldWhile::Done(acc + 1),
                Ordering::Less => FoldWhile::Continue(acc + 1),
            }
        })
        .into_inner();

    // Down
    let down_score = grid
        .iter()
        .skip(y + 1)
        .fold_while(0, |acc, row| {
            let h = row[x];
            match h.cmp(&min_height) {
                Ordering::Greater | Ordering::Equal => FoldWhile::Done(acc + 1),
                Ordering::Less => FoldWhile::Continue(acc + 1),
            }
        })
        .into_inner();

    left_score * right_score * up_score * down_score
}

fn main() -> io::Result<()> {
    let grid = itertools::process_results(io::stdin().lines(), |it| {
        it.map(|line| {
            line.chars()
                .map(|ch| {
                    ch.to_digit(10)
                        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "invalid number"))
                })
                .collect::<Result<Vec<u32>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()
    })??;

    let row_len = grid[0].len();

    assert!(grid.iter().all(|row| row.len() == row_len));

    let answer = (0..grid.len())
        .filter_map(|y| (0..row_len).map(|x| scenic_score(x, y, &grid)).max())
        .max()
        .expect("grid is empty");

    println!("{answer}");

    Ok(())
}

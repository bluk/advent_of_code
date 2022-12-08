use std::io;

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

    let answer = (0..grid.len()).fold(0, |acc, y| {
        acc + (0..row_len).fold(0, |acc, x| {
            acc + u32::from(!is_visible_from_edge(y, x, &grid))
        })
    });

    println!("{answer}");

    Ok(())
}

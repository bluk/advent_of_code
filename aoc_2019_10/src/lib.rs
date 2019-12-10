use error::Error;
use std::convert::TryFrom;

pub mod error;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    fn dir_from(&self, other: Pos) -> Result<Dir, Error> {
        let y0 = isize::try_from(other.y)?;
        let x0 = isize::try_from(other.x)?;

        let dy = y0 - isize::try_from(self.y)?;
        let dx = isize::try_from(self.x)? - x0;
        Ok(Dir { dx, dy })
    }
}

/// Parse a string into memory state.
pub fn build_map(input: &str) -> Result<Vec<Pos>, Error> {
    let mut map: Vec<Pos> = Vec::new();
    let mut y = 0;

    for line in input.lines() {
        let mut asteriods = line
            .trim()
            .chars()
            .enumerate()
            .filter_map(|(x, c)| if c == '#' { Some(Pos { x, y }) } else { None })
            .collect();
        map.append(&mut asteriods);
        y += 1;
    }

    Ok(map)
}

#[derive(Copy, Clone, Hash, Debug)]
struct Dir {
    dx: isize,
    dy: isize,
}

impl Dir {
    fn len(&self) -> f64 {
        ((self.dx.pow(2) + self.dy.pow(2)) as f64).sqrt()
    }
}

impl PartialEq for Dir {
    fn eq(&self, other: &Self) -> bool {
        let is_x_same_dir = (self.dx > 0 && other.dx > 0)
            || (self.dx < 0 && other.dx < 0)
            || (self.dx == 0 && other.dx == 0);
        if !is_x_same_dir {
            return false;
        }
        let is_y_same_dir = (self.dy > 0 && other.dy > 0)
            || (self.dy < 0 && other.dy < 0)
            || (self.dy == 0 && other.dy == 0);
        if !is_y_same_dir {
            return false;
        }

        let self_dx = self.dx * other.dy;
        let other_dx = other.dx * self.dy;

        self_dx == other_dx
    }
}

impl Eq for Dir {}

fn detectable_asteriods(starting_pos: &Pos, map: &[Pos]) -> Result<Vec<Pos>, Error> {
    let mut nearest: Vec<(Pos, Dir)> = vec![];

    for p in map.iter() {
        if p == starting_pos {
            continue;
        }

        let d = p.dir_from(*starting_pos)?;

        let mut found_same_dir = false;
        for (i, n) in nearest.iter().enumerate() {
            if n.1 == d {
                found_same_dir = true;

                let dm = d.len();
                if dm == f64::min(dm, n.1.len()) {
                    nearest[i] = (*p, d);
                }
                break;
            }
        }

        if !found_same_dir {
            nearest.push((*p, d))
        }
    }

    Ok(nearest.into_iter().map(|(p, _)| p).collect())
}

pub fn find_best_monitoring_pos(map: &[Pos]) -> Result<Option<(&Pos, usize)>, Error> {
    Ok(map.iter().map(|p| (p, detectable_asteriods(p, map))).fold(
        None,
        |acc, (p, detectable_asteriods)| {
            let count = detectable_asteriods.unwrap().len();
            if let Some(acc) = acc {
                if count > acc.1 {
                    Some((p, count))
                } else {
                    Some(acc)
                }
            } else {
                Some((p, count))
            }
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10_dir_eq() {
        assert_eq!(Dir { dx: -3, dy: 2 }, Dir { dx: -3, dy: 2 });

        assert_ne!(Dir { dx: -3, dy: 2 }, Dir { dx: -2, dy: 2 });
    }

    #[test]
    fn day10_ex1() {
        let input = "
.#..#
.....
#####
....#
...##
"
        .trim();

        let map = build_map(input).unwrap();
        assert_eq!(
            find_best_monitoring_pos(&map).unwrap().unwrap(),
            (&Pos { x: 3, y: 4 }, 8)
        );
    }

    #[test]
    fn day10_ex2() {
        let input = "
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####
"
        .trim();

        let map = build_map(input).unwrap();
        assert_eq!(
            find_best_monitoring_pos(&map).unwrap().unwrap(),
            (&Pos { x: 5, y: 8 }, 33)
        );
    }

    #[test]
    fn day10_ex3() {
        let input = "
#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.
"
        .trim();

        let map = build_map(input).unwrap();
        assert_eq!(
            find_best_monitoring_pos(&map).unwrap().unwrap(),
            (&Pos { x: 1, y: 2 }, 35)
        );
    }

    #[test]
    fn day10_ex4() {
        let input = "
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
"
        .trim();

        let map = build_map(input).unwrap();

        assert_eq!(
            find_best_monitoring_pos(&map).unwrap().unwrap(),
            (&Pos { x: 11, y: 13 }, 210)
        );
    }
}

use error::Error;
use std::cmp::Ordering;
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

    fn angle(&self) -> f64 {
        (self.dy as f64).atan2(self.dx as f64)
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

fn sort_asteriods(starting_pos: &Pos, asteriods: &mut [Pos]) {
    asteriods.sort_by(|&a, &b| {
        let mut aa =
            (a.dir_from(*starting_pos).unwrap().angle() - std::f64::consts::FRAC_PI_2) * -1.0;
        if aa < 0.0 {
            aa += 2.0 * std::f64::consts::PI;
        }
        let mut ba =
            (b.dir_from(*starting_pos).unwrap().angle() - std::f64::consts::FRAC_PI_2) * -1.0;
        if ba < 0.0 {
            ba += 2.0 * std::f64::consts::PI;
        }
        if aa == ba {
            return Ordering::Equal;
        }

        if aa == f64::min(aa, ba) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
}

pub struct VaporizeIter {
    map: Vec<Pos>,
    starting_pos: Pos,
    cur_pass: Option<Vec<Pos>>,
}

impl VaporizeIter {
    pub fn new(starting_pos: &Pos, map: &[Pos]) -> Self {
        let mut m = Vec::with_capacity(map.len());
        m.extend_from_slice(map);
        VaporizeIter {
            map: m,
            starting_pos: *starting_pos,
            cur_pass: None,
        }
    }
}

impl Iterator for VaporizeIter {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur_pass) = self.cur_pass.as_mut() {
            let ret = cur_pass.pop();
            if ret.is_some() {
                return ret;
            }
            self.cur_pass = None;
        }

        if self.map.is_empty() {
            return None;
        }

        let mut asteriods = detectable_asteriods(&self.starting_pos, &self.map).unwrap();
        sort_asteriods(&self.starting_pos, &mut asteriods);

        self.map = self
            .map
            .iter()
            .copied()
            .filter(|p| !asteriods.contains(p))
            .collect();

        asteriods.reverse();

        let ret = asteriods.pop();
        self.cur_pass = Some(asteriods);
        ret
    }
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

    #[test]
    fn test_sort() {
        let starting_pos = Pos { x: 11, y: 13 };
        let mut asteriods = [
            Pos { x: 14, y: 15 },
            Pos { x: 11, y: 12 },
            Pos { x: 12, y: 13 },
            Pos { x: 10, y: 16 },
            Pos { x: 10, y: 10 },
            Pos { x: 12, y: 10 },
            Pos { x: 10, y: 13 },
            Pos { x: 11, y: 14 },
        ];
        sort_asteriods(&starting_pos, &mut asteriods);
        assert_eq!(
            [
                Pos { x: 11, y: 12 },
                Pos { x: 12, y: 10 },
                Pos { x: 12, y: 13 },
                Pos { x: 14, y: 15 },
                Pos { x: 11, y: 14 },
                Pos { x: 10, y: 16 },
                Pos { x: 10, y: 13 },
                Pos { x: 10, y: 10 },
            ],
            asteriods
        );
    }

    #[test]
    fn day10_ex5() {
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

        let starting_pos = Pos { x: 11, y: 13 };

        let mut iter = VaporizeIter::new(&starting_pos, &map);
        assert_eq!(iter.next(), Some(Pos { x: 11, y: 12 }));
        assert_eq!(iter.next(), Some(Pos { x: 12, y: 1 }));
        assert_eq!(iter.next(), Some(Pos { x: 12, y: 2 }));

        let mut iter = iter.skip(6);
        assert_eq!(iter.next(), Some(Pos { x: 12, y: 8 }));

        let mut iter = iter.skip(9);
        assert_eq!(iter.next(), Some(Pos { x: 16, y: 0 }));

        let mut iter = iter.skip(29);
        assert_eq!(iter.next(), Some(Pos { x: 16, y: 9 }));

        let mut iter = iter.skip(49);
        assert_eq!(iter.next(), Some(Pos { x: 10, y: 16 }));

        let mut iter = iter.skip(98);
        assert_eq!(iter.next(), Some(Pos { x: 9, y: 6 }));
        assert_eq!(iter.next(), Some(Pos { x: 8, y: 2 }));
        assert_eq!(iter.next(), Some(Pos { x: 10, y: 9 }));

        let mut iter = iter.skip(97);
        assert_eq!(iter.next(), Some(Pos { x: 11, y: 1 }));

        assert_eq!(iter.next(), None);
    }
}

use std::collections::BTreeSet;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::iter::FromIterator;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Move {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

pub fn parse_moves(input: &str) -> Result<Vec<Move>, std::num::ParseIntError> {
    input
        .trim()
        .split(',')
        .map(|s| {
            let s = s.trim();
            let dir = &s[0..1];
            let value = s[1..].parse::<usize>()?;
            Ok(match dir {
                "U" => Move::Up(value),
                "D" => Move::Down(value),
                "L" => Move::Left(value),
                "R" => Move::Right(value),
                _ => panic!(),
            })
        })
        .collect::<Result<Vec<Move>, std::num::ParseIntError>>()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    pub fn manhattan_dist(&self) -> usize {
        usize::try_from(self.x.abs()).unwrap() + usize::try_from(self.y.abs()).unwrap()
    }
}

pub fn visited_pos_with_steps(moves: &[Move]) -> HashMap<Pos, usize> {
    let mut pos = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    let mut step = 0;

    for m in moves {
        let (dx, dy, len) = match m {
            Move::Up(len) => (0, 1, len),
            Move::Down(len) => (0, -1, len),
            Move::Left(len) => (-1, 0, len),
            Move::Right(len) => (1, 0, len),
        };

        (0..*len).for_each(|_| {
            x += dx;
            y += dy;
            step += 1;

            let p = Pos { x, y };
            pos.entry(p).or_insert(step);
        })
    }

    pos
}

pub fn find_closest_manhattan_dist(
    input1: &str,
    input2: &str,
) -> Result<Option<usize>, std::num::ParseIntError> {
    let moves1 = parse_moves(&input1)?;
    let moves2 = parse_moves(&input2)?;
    let visited_pos_with_steps1 = visited_pos_with_steps(&moves1);
    let visited_pos_with_steps2 = visited_pos_with_steps(&moves2);
    let visited_pos1 = BTreeSet::from_iter(visited_pos_with_steps1.keys());
    let visited_pos2 = BTreeSet::from_iter(visited_pos_with_steps2.keys());

    let intersection: BTreeSet<_> = visited_pos1.intersection(&visited_pos2).cloned().collect();
    Ok(intersection.into_iter().map(|&p| p.manhattan_dist()).min())
}

pub fn find_fewest_steps(
    input1: &str,
    input2: &str,
) -> Result<Option<usize>, std::num::ParseIntError> {
    let moves1 = parse_moves(&input1)?;
    let moves2 = parse_moves(&input2)?;
    let visited_pos_with_steps1 = visited_pos_with_steps(&moves1);
    let visited_pos_with_steps2 = visited_pos_with_steps(&moves2);
    let visited_pos1 = BTreeSet::from_iter(visited_pos_with_steps1.keys());
    let visited_pos2 = BTreeSet::from_iter(visited_pos_with_steps2.keys());

    let intersection: BTreeSet<_> = visited_pos1.intersection(&visited_pos2).cloned().collect();
    Ok(intersection
        .into_iter()
        .filter_map(|p| {
            match (
                visited_pos_with_steps1.get(p),
                visited_pos_with_steps2.get(p),
            ) {
                (Some(dist1), Some(dist2)) => Some(dist1 + dist2),
                _ => None,
            }
        })
        .min())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        let input = "U12,D34,L56,R78";
        assert_eq!(
            Ok(vec![
                Move::Up(12),
                Move::Down(34),
                Move::Left(56),
                Move::Right(78)
            ]),
            parse_moves(&input),
        );
    }

    #[test]
    fn test_visited_pos() {
        let input = "U3,D2,L4";
        let moves = parse_moves(&input).unwrap();
        assert_eq!(
            BTreeSet::from_iter(vec![
                &Pos { x: 0, y: 1 },
                &Pos { x: 0, y: 2 },
                &Pos { x: 0, y: 3 },
                &Pos { x: -1, y: 1 },
                &Pos { x: -2, y: 1 },
                &Pos { x: -3, y: 1 },
                &Pos { x: -4, y: 1 },
            ]),
            BTreeSet::from_iter(visited_pos_with_steps(&moves).keys())
        );
    }

    #[test]
    fn ex1() {
        let input1 = "R8,U5,L5,D3";
        let input2 = "U7,R6,D4,L4";
        assert_eq!(find_closest_manhattan_dist(&input1, &input2), Ok(Some(6)));
    }

    #[test]
    fn ex2() {
        let input1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        let input2 = "U62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(find_closest_manhattan_dist(&input1, &input2), Ok(Some(159)));
    }

    #[test]
    fn ex3() {
        let input1 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
        let input2 = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        assert_eq!(find_closest_manhattan_dist(&input1, &input2), Ok(Some(135)));
    }

    #[test]
    fn ex4() {
        let input1 = "R8,U5,L5,D3";
        let input2 = "U7,R6,D4,L4";
        assert_eq!(find_fewest_steps(&input1, &input2), Ok(Some(30)));
    }

    #[test]
    fn ex5() {
        let input1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        let input2 = "U62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(find_fewest_steps(&input1, &input2), Ok(Some(610)));
    }

    #[test]
    fn ex6() {
        let input1 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
        let input2 = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        assert_eq!(find_fewest_steps(&input1, &input2), Ok(Some(410)));
    }
}

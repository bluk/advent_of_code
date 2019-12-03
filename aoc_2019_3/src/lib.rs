use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Move {
    Up(isize),
    Down(isize),
    Left(isize),
    Right(isize),
}

pub fn parse_moves(input: &str) -> Result<Vec<Move>, std::num::ParseIntError> {
    input
        .trim()
        .split(',')
        .map(|s| {
            let s = s.trim();
            let dir = &s[0..1];
            let value = s[1..].parse::<isize>()?;
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    pub fn manhattan_dist(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
}

pub fn visited_pos(moves: &[Move]) -> BTreeSet<Pos> {
    let mut pos = BTreeSet::new();
    let mut x = 0;
    let mut y = 0;

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
            pos.insert(Pos { x, y });
        })
    }

    pos
}

#[cfg(test)]
mod test {
    use super::*;

    use std::iter::FromIterator;

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
                Pos { x: 0, y: 1 },
                Pos { x: 0, y: 2 },
                Pos { x: 0, y: 3 },
                Pos { x: -1, y: 1 },
                Pos { x: -2, y: 1 },
                Pos { x: -3, y: 1 },
                Pos { x: -4, y: 1 },
            ]),
            visited_pos(&moves)
        );
    }

    #[test]
    fn ex1() {
        let input1 = "R8,U5,L5,D3";
        let input2 = "U7,R6,D4,L4";
        let moves1 = parse_moves(&input1).unwrap();
        let moves2 = parse_moves(&input2).unwrap();
        let pos1 = visited_pos(&moves1);
        let pos2 = visited_pos(&moves2);

        let intersection: BTreeSet<_> = pos1.intersection(&pos2).cloned().collect();
        println!("{:?}", intersection);
        assert_eq!(
            intersection
                .iter()
                .map(|p| p.manhattan_dist())
                .min()
                .unwrap(),
            6
        );
    }

    #[test]
    fn ex2() {
        let input1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        let input2 = "U62,R66,U55,R34,D71,R55,D58,R83";
        let moves1 = parse_moves(&input1).unwrap();
        let moves2 = parse_moves(&input2).unwrap();
        let pos1 = visited_pos(&moves1);
        let pos2 = visited_pos(&moves2);

        let intersection: BTreeSet<_> = pos1.intersection(&pos2).cloned().collect();
        assert_eq!(
            intersection
                .iter()
                .map(|p| p.manhattan_dist())
                .min()
                .unwrap(),
            159
        );
    }

    #[test]
    fn ex3() {
        let input1 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
        let input2 = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let moves1 = parse_moves(&input1).unwrap();
        let moves2 = parse_moves(&input2).unwrap();
        let pos1 = visited_pos(&moves1);
        let pos2 = visited_pos(&moves2);

        let intersection: BTreeSet<_> = pos1.intersection(&pos2).cloned().collect();
        assert_eq!(
            intersection
                .iter()
                .map(|p| p.manhattan_dist())
                .min()
                .unwrap(),
            135
        );
    }
}

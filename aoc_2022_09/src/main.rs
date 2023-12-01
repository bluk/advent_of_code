use std::{collections::HashSet, io};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Movement {
    dir: Direction,
    len: i32,
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn move_dir(self, dir: Direction) -> Self {
        let mut x = self.x;
        let mut y = self.y;

        match dir {
            Direction::Up => y += 1,
            Direction::Down => y -= 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
        }

        Self { x, y }
    }

    fn follow(self, other: Position) -> Self {
        let diff_x = other.x - self.x;
        let diff_y = other.y - self.y;

        match (diff_x, diff_y) {
            (-1..=1, -1..=1) => self,
            (0, 2) => self.move_dir(Direction::Up),
            (0, -2) => self.move_dir(Direction::Down),
            (-2, 0) => self.move_dir(Direction::Left),
            (2, 0) => self.move_dir(Direction::Right),

            (-2 | -1, 2) | (-2, 1) => self.move_dir(Direction::Left).move_dir(Direction::Up),
            (1 | 2, 2) | (2, 1) => self.move_dir(Direction::Right).move_dir(Direction::Up),

            (2, -1 | -2) | (1, -2) => self.move_dir(Direction::Right).move_dir(Direction::Down),

            (-2 | -1, -2) | (-2, -1) => self.move_dir(Direction::Left).move_dir(Direction::Down),
            _ => unreachable!(),
        }
    }
}

// X X X X X
// X X X X X
// X X S X X
// X X X X X
// X X X X X

fn parse_line(line: &str) -> nom::IResult<&str, Movement> {
    use nom::{branch, bytes, character, combinator, sequence};

    combinator::map(
        sequence::separated_pair(
            branch::alt((
                combinator::map(bytes::complete::tag("U"), |_| Direction::Up),
                combinator::map(bytes::complete::tag("D"), |_| Direction::Down),
                combinator::map(bytes::complete::tag("L"), |_| Direction::Left),
                combinator::map(bytes::complete::tag("R"), |_| Direction::Right),
            )),
            character::complete::space1,
            combinator::map_res(character::complete::digit1, str::parse),
        ),
        |(dir, len)| Movement { dir, len },
    )(line)
}

fn print_map(knots: &[Position]) {
    for y in -20..=20 {
        for x in -20..=20 {
            if let Some(knot) = knots.iter().position(|pos| pos.y == y && pos.x == x) {
                print!("{knot} ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
}

fn main() -> io::Result<()> {
    let positions = itertools::process_results(io::stdin().lines(), |it| {
        let mut knots = [Position::default(); 10];

        let mut positions: HashSet<Position> = HashSet::default();
        positions.insert(knots[knots.len() - 1]);

        for line in it {
            let (remaining, head_move) = parse_line(&line)
                .map_err(|_| io::Error::new(io::ErrorKind::Other, "invalid move"))?;
            assert_eq!("", remaining);

            for _ in 0..head_move.len {
                knots[0] = knots[0].move_dir(head_move.dir);
                for i in 1..knots.len() {
                    knots[i] = knots[i].follow(knots[i - 1]);
                }
                positions.insert(knots[knots.len() - 1]);
            }

            // print_map(&knots);
        }

        Ok::<_, io::Error>(positions)
    })??;

    let answer = positions.len();

    println!("{answer}");

    Ok(())
}

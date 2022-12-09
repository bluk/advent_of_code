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
        let diff_x = self.x - other.x;
        let diff_y = self.y - other.y;

        match (diff_x, diff_y) {
            (0 | -1 | 1, 0 | -1 | 1) => self,
            (0, -2) => self.move_dir(Direction::Up),
            (1, -2) => self.move_dir(Direction::Up).move_dir(Direction::Left),
            (-1, -2) => self.move_dir(Direction::Up).move_dir(Direction::Right),
            (0, 2) => self.move_dir(Direction::Down),
            (1, 2) => self.move_dir(Direction::Down).move_dir(Direction::Left),
            (-1, 2) => self.move_dir(Direction::Down).move_dir(Direction::Right),

            (2, 0) => self.move_dir(Direction::Left),
            (2, -1) => self.move_dir(Direction::Left).move_dir(Direction::Up),
            (2, 1) => self.move_dir(Direction::Left).move_dir(Direction::Down),

            (-2, 0) => self.move_dir(Direction::Right),
            (-2, 1) => self.move_dir(Direction::Right).move_dir(Direction::Down),
            (-2, -1) => self.move_dir(Direction::Right).move_dir(Direction::Up),
            _ => unreachable!(),
        }
    }
}

// * X X X *
// X X X X *
// X X S X X
// X X X X *
// * X X X *

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

fn main() -> io::Result<()> {
    let positions = itertools::process_results(io::stdin().lines(), |it| {
        let mut head_pos = Position::default();
        let mut tail_pos = Position::default();

        let mut positions: HashSet<Position> = HashSet::default();
        positions.insert(tail_pos);

        for line in it {
            let (remaining, head_move) = parse_line(&line)
                .map_err(|_| io::Error::new(io::ErrorKind::Other, "invalid move"))?;
            assert_eq!("", remaining);

            for _ in 0..head_move.len {
                head_pos = head_pos.move_dir(head_move.dir);
                tail_pos = tail_pos.follow(head_pos);
                positions.insert(tail_pos);
            }
        }

        Ok::<_, io::Error>(positions)
    })??;

    let answer = positions.len();

    println!("{answer}");

    Ok(())
}

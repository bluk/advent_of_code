use std::cmp::Ordering;
use std::collections::VecDeque;
use std::str::FromStr;

use crate::error::Error;
use crate::intcode::{Prog, ProgState};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Type {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

impl FromStr for Type {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Type::Empty),
            "1" => Ok(Type::Wall),
            "2" => Ok(Type::Block),
            "3" => Ok(Type::HorizontalPaddle),
            "4" => Ok(Type::Ball),
            _ => Err(Error::UnknownValue),
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Pos {
    x: i64,
    y: i64,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Tile {
    pub pos: Pos,
    pub tile_id: Type,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Joystick {
    Neutral,
    Left,
    Right,
}

impl From<Joystick> for String {
    fn from(other: Joystick) -> String {
        match other {
            Joystick::Neutral => String::from("0"),
            Joystick::Left => String::from("-1"),
            Joystick::Right => String::from("1"),
        }
    }
}

pub fn run_loop(mut prog: Prog) -> Result<i64, Error> {
    let mut input = VecDeque::new();
    let mut output = VecDeque::new();

    let mut score: i64 = 0;
    let mut tiles = Vec::<Tile>::new();

    loop {
        let mut new_tiles = Vec::<Tile>::new();
        prog.run(&mut input, &mut output)?;

        while let Some(x) = output.pop_front() {
            if let Some(y) = output.pop_front() {
                if let Some(id) = output.pop_front() {
                    if x == "-1" && y == "0" {
                        score = id.parse::<i64>()?;
                    } else {
                        new_tiles.push(Tile {
                            pos: Pos {
                                x: x.parse::<i64>()?,
                                y: y.parse::<i64>()?,
                            },
                            tile_id: Type::from_str(&id)?,
                        });
                    }
                } else {
                    panic!("unexpected program state");
                }
            } else {
                panic!("unexpected program state");
            }
        }

        let pos_to_replace = new_tiles.iter().map(|t| t.pos).collect::<Vec<Pos>>();

        tiles = tiles
            .into_iter()
            .filter(|t| !pos_to_replace.contains(&t.pos))
            .collect::<Vec<Tile>>();

        tiles.append(&mut new_tiles);

        // display(tiles.clone());

        match prog.state() {
            ProgState::Halt => break,
            ProgState::NotStarted => unreachable!(),
            ProgState::NeedInput => {
                let paddle = tiles
                    .iter()
                    .find(|&&t| t.tile_id == Type::HorizontalPaddle)
                    .expect("paddle to exist");

                let ball = tiles
                    .iter()
                    .find(|&&t| t.tile_id == Type::Ball)
                    .expect("ball to exist");

                if ball.pos.x == paddle.pos.x {
                    input.push_back(String::from(Joystick::Neutral));
                } else if ball.pos.x > paddle.pos.x {
                    input.push_back(String::from(Joystick::Right));
                } else if ball.pos.x < paddle.pos.x {
                    input.push_back(String::from(Joystick::Left));
                }
            }
        }
    }

    Ok(score)
}

pub fn display(mut tiles: Vec<Tile>) {
    let max_x = tiles.iter().map(|t| t.pos.x).max().unwrap();
    let min_x = tiles.iter().map(|t| t.pos.x).min().unwrap();
    let max_y = tiles.iter().map(|t| t.pos.y).max().unwrap();
    let min_y = tiles.iter().map(|t| t.pos.y).min().unwrap();

    tiles.sort_by(|a, b| {
        if a.pos.y > b.pos.y {
            Ordering::Less
        } else if a.pos.y < b.pos.y {
            Ordering::Greater
        } else {
            a.pos.x.cmp(&b.pos.x)
        }
    });

    let mut iter = tiles.iter().peekable();

    let rng = max_y - min_y;
    for y in 0..=rng {
        let y = max_y - y;
        for x in min_x..=max_x {
            if let Some(&next_tile) = iter.peek() {
                let pos = Pos { x, y };
                if next_tile.pos == pos {
                    print!(
                        "{}",
                        match next_tile.tile_id {
                            Type::Empty => " ",
                            Type::Wall => "|",
                            Type::Block => "=",
                            Type::HorizontalPaddle => "-",
                            Type::Ball => "*",
                        }
                    );
                    iter.next();
                }
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

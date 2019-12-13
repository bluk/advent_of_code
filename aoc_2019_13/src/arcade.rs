use std::collections::VecDeque;
use std::str::FromStr;

use crate::error::Error;
use crate::intcode::Prog;

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
pub struct Tile {
    pub x: i64,
    pub y: i64,
    pub tile_id: Type,
}

pub fn run_loop(mut prog: Prog) -> Result<Vec<Tile>, Error> {
    let mut tiles = Vec::<Tile>::new();

    let mut input = VecDeque::new();
    let mut output = VecDeque::new();

    prog.run(&mut input, &mut output)?;

    while let Some(x) = output.pop_front() {
        if let Some(y) = output.pop_front() {
            if let Some(id) = output.pop_front() {
                tiles.push(Tile {
                    x: x.parse::<i64>()?,
                    y: y.parse::<i64>()?,
                    tile_id: Type::from_str(&id)?,
                })
            } else {
                panic!("unexpected program state");
            }
        } else {
            panic!("unexpected program state");
        }
    }

    Ok(tiles)
}

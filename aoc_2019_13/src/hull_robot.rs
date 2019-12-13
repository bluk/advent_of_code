use std::cmp::Ordering;
use std::convert::TryFrom;

use crate::error::Error;
use crate::intcode::{Prog, ProgState, VecDequeProgInput, VecDequeProgOutput};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Color {
    Black,
    White,
}

impl TryFrom<String> for Color {
    type Error = Error;

    fn try_from(other: String) -> Result<Self, Self::Error> {
        if other == "0" {
            Ok(Color::Black)
        } else if other == "1" {
            Ok(Color::White)
        } else {
            Err(Error::UnknownValue)
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum TurnDir {
    Left,
    Right,
}

impl TryFrom<String> for TurnDir {
    type Error = Error;

    fn try_from(other: String) -> Result<Self, Self::Error> {
        if other == "0" {
            Ok(TurnDir::Left)
        } else if other == "1" {
            Ok(TurnDir::Right)
        } else {
            Err(Error::UnknownValue)
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum RobotDir {
    Up,
    Down,
    Left,
    Right,
}

impl RobotDir {
    fn turn(self, dir: TurnDir) -> RobotDir {
        match (self, dir) {
            (RobotDir::Up, TurnDir::Left) => RobotDir::Left,
            (RobotDir::Up, TurnDir::Right) => RobotDir::Right,
            (RobotDir::Left, TurnDir::Left) => RobotDir::Down,
            (RobotDir::Left, TurnDir::Right) => RobotDir::Up,
            (RobotDir::Down, TurnDir::Left) => RobotDir::Right,
            (RobotDir::Down, TurnDir::Right) => RobotDir::Left,
            (RobotDir::Right, TurnDir::Left) => RobotDir::Up,
            (RobotDir::Right, TurnDir::Right) => RobotDir::Down,
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Pos {
    x: isize,
    y: isize,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Panel {
    pos: Pos,
    color: Color,
}

pub fn paint_hull(mut prog: Prog) -> Result<Vec<Panel>, Error> {
    let mut robot_dir = RobotDir::Up;
    let mut robot_pos = Pos { x: 0, y: 0 };
    let mut panels = Vec::<Panel>::new();

    let mut input = VecDequeProgInput::new();
    let mut output = VecDequeProgOutput::new();

    loop {
        let index = panels.iter().position(|p| p.pos == robot_pos);
        let color = if let Some(index) = index {
            panels[index].color
        } else {
            let start_pos = Pos { x: 0, y: 0 };
            if robot_pos == start_pos {
                Color::White
            } else {
                Color::Black
            }
        };

        input.data.push_back(match color {
            Color::Black => "0".to_string(),
            Color::White => "1".to_string(),
        });
        prog.run(&mut input, &mut output)?;

        let panel = if let Some(index) = index {
            panels.get_mut(index).unwrap()
        } else {
            let panel = Panel {
                pos: robot_pos,
                color: Color::Black,
            };
            panels.push(panel);
            panels.last_mut().unwrap()
        };

        if let Some(v) = output.data.pop_front() {
            let color = Color::try_from(v)?;
            panel.color = color;
        } else {
            panic!("unexpected program state");
        }

        if let Some(v) = output.data.pop_front() {
            let turn_dir = TurnDir::try_from(v)?;
            robot_dir = robot_dir.turn(turn_dir);
            match robot_dir {
                RobotDir::Up => robot_pos.y += 1,
                RobotDir::Left => robot_pos.x -= 1,
                RobotDir::Down => robot_pos.y -= 1,
                RobotDir::Right => robot_pos.x += 1,
            }
        } else {
            panic!("unexpected program state");
        }

        match prog.state() {
            ProgState::Halt => {
                break;
            }
            ProgState::NeedInput => {}
            ProgState::NotStarted => unreachable!(),
        }
    }

    assert!(input.data.is_empty());
    assert!(output.data.is_empty());

    Ok(panels)
}

pub fn display_panels(mut panels: Vec<Panel>) {
    let max_x = panels.iter().map(|p| p.pos.x).max().unwrap();
    let min_x = panels.iter().map(|p| p.pos.x).min().unwrap();
    let max_y = panels.iter().map(|p| p.pos.y).max().unwrap();
    let min_y = panels.iter().map(|p| p.pos.y).min().unwrap();

    panels.sort_by(|a, b| {
        if a.pos.y > b.pos.y {
            Ordering::Less
        } else if a.pos.y < b.pos.y {
            Ordering::Greater
        } else {
            a.pos.x.cmp(&b.pos.x)
        }
    });

    let rng = max_y - min_y;
    for y in 0..=rng {
        let y = max_y - y;
        for x in min_x..=max_x {
            if let Some(panel) = panels.iter().find(|p| p.pos == Pos { x, y }) {
                match panel.color {
                    Color::Black => print!(" "),
                    Color::White => print!("*"),
                }
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

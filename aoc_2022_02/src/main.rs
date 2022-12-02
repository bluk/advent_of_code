use std::io;

type ScoreTy = u32;

#[derive(Debug)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn score(&self) -> ScoreTy {
        use Outcome::{Draw, Lose, Win};

        match self {
            Win => 6,
            Lose => 0,
            Draw => 3,
        }
    }
}

#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn opponent_pick(s: &str) -> io::Result<Self> {
        use Shape::{Paper, Rock, Scissors};

        match s {
            "A" => Ok(Rock),
            "B" => Ok(Paper),
            "C" => Ok(Scissors),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "unknown opponent pick",
            )),
        }
    }

    fn my_pick(s: &str) -> io::Result<Self> {
        use Shape::{Paper, Rock, Scissors};

        match s {
            "X" => Ok(Rock),
            "Y" => Ok(Paper),
            "Z" => Ok(Scissors),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "unknown player pick",
            )),
        }
    }

    fn score(&self) -> ScoreTy {
        use Shape::{Paper, Rock, Scissors};

        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn play(&self, other: &Shape) -> Outcome {
        match (self, other) {
            (Shape::Rock, Shape::Rock)
            | (Shape::Paper, Shape::Paper)
            | (Shape::Scissors, Shape::Scissors) => Outcome::Draw,
            (Shape::Rock, Shape::Scissors)
            | (Shape::Paper, Shape::Rock)
            | (Shape::Scissors, Shape::Paper) => Outcome::Win,
            (Shape::Paper, Shape::Scissors)
            | (Shape::Scissors, Shape::Rock)
            | (Shape::Rock, Shape::Paper) => Outcome::Lose,
        }
    }
}

fn main() -> io::Result<()> {
    let mut total_score = 0;

    for line in io::stdin().lines() {
        let line = line?;
        let mut split_line = line.split_whitespace();

        match (split_line.next(), split_line.next(), split_line.next()) {
            (Some(opponent_pick), Some(my_pick), None) => {
                let opponent_pick = Shape::opponent_pick(opponent_pick)?;
                let my_pick = Shape::my_pick(my_pick)?;
                let outcome = my_pick.play(&opponent_pick);
                total_score += my_pick.score() + outcome.score();
            }
            _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, line)),
        }
    }

    println!("{total_score}");

    Ok(())
}

use std::io;

type ScoreTy = u32;

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn score(self) -> ScoreTy {
        use Outcome::{Draw, Lose, Win};

        match self {
            Win => 6,
            Lose => 0,
            Draw => 3,
        }
    }

    fn desired(s: &str) -> io::Result<Self> {
        use Outcome::{Draw, Lose, Win};

        match s {
            "X" => Ok(Lose),
            "Y" => Ok(Draw),
            "Z" => Ok(Win),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "unknown outcome",
            )),
        }
    }
}

#[derive(Debug, Clone, Copy)]
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

    fn score(self) -> ScoreTy {
        use Shape::{Paper, Rock, Scissors};

        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn play(self, other: Shape) -> Outcome {
        use Outcome::{Draw, Lose, Win};
        use Shape::{Paper, Rock, Scissors};

        match (self, other) {
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Draw,
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Win,
            (Paper, Scissors) | (Scissors, Rock) | (Rock, Paper) => Lose,
        }
    }

    fn for_outcome(self, desired: Outcome) -> Self {
        use Outcome::{Draw, Lose, Win};
        use Shape::{Paper, Rock, Scissors};

        match (self, desired) {
            (_, Draw) => self,
            (Rock, Lose) | (Paper, Win) => Scissors,
            (Rock, Win) | (Scissors, Lose) => Paper,
            (Paper, Lose) | (Scissors, Win) => Rock,
        }
    }
}

fn main() -> io::Result<()> {
    let mut total_score = 0;

    for line in io::stdin().lines() {
        let line = line?;
        let mut split_line = line.split_whitespace();

        match (split_line.next(), split_line.next(), split_line.next()) {
            (Some(opponent_pick), Some(desired), None) => {
                let opponent_pick = Shape::opponent_pick(opponent_pick)?;
                let desired = Outcome::desired(desired)?;
                let my_pick = opponent_pick.for_outcome(desired);
                total_score += my_pick.score() + desired.score();
            }
            _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, line)),
        }
    }

    println!("{total_score}");

    Ok(())
}

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

fn parse_line(line: &str) -> io::Result<(Shape, Outcome)> {
    let mut split_line = line.split_whitespace();

    match (split_line.next(), split_line.next(), split_line.next()) {
        (Some(opponent_pick), Some(desired), None) => Ok((
            Shape::opponent_pick(opponent_pick)?,
            Outcome::desired(desired)?,
        )),
        _ => Err(io::Error::new(io::ErrorKind::InvalidInput, line)),
    }
}

fn fold_score(acc: ScoreTy, opponent_pick: Shape, outcome: Outcome) -> ScoreTy {
    let my_pick = opponent_pick.for_outcome(outcome);
    acc + my_pick.score() + outcome.score()
}

fn main() -> io::Result<()> {
    let total_score = io::stdin()
        .lines()
        .map(|result| result.and_then(|s| parse_line(&s)))
        .try_fold(0, |acc, result| {
            result.map(|(opponent_pick, outcome)| fold_score(acc, opponent_pick, outcome))
        })?;

    println!("{total_score}");

    Ok(())
}

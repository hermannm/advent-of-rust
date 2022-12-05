pub enum GameChoice {
    Rock,
    Paper,
    Scissors,
}

impl GameChoice {
    pub fn score(&self) -> i32 {
        match self {
            GameChoice::Rock => 1,
            GameChoice::Paper => 2,
            GameChoice::Scissors => 3,
        }
    }
}

pub enum GameOutcome {
    Loss,
    Draw,
    Win,
}

impl GameOutcome {
    pub fn score(&self) -> i32 {
        match self {
            GameOutcome::Loss => 0,
            GameOutcome::Draw => 3,
            GameOutcome::Win => 6,
        }
    }
}

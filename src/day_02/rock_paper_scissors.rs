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

    pub fn from_game_outcome(game_outcome: &GameOutcome, enemy_choice: &GameChoice) -> GameChoice {
        use GameChoice::*;
        use GameOutcome::*;

        match (game_outcome, enemy_choice) {
            (Draw, Rock) | (Loss, Paper) | (Win, Scissors) => Rock,
            (Win, Rock) | (Draw, Paper) | (Loss, Scissors) => Paper,
            (Loss, Rock) | (Win, Paper) | (Draw, Scissors) => Scissors,
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

    pub fn from_game_choices(enemy_choice: &GameChoice, our_choice: &GameChoice) -> GameOutcome {
        use GameChoice::*;
        use GameOutcome::*;

        match (enemy_choice, our_choice) {
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Loss,
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Draw,
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Win,
        }
    }
}

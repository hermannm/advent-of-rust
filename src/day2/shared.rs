pub enum GameChoice {
    Rock,
    Paper,
    Scissors,
}

impl GameChoice {
    fn score(&self) -> i32 {
        match self {
            GameChoice::Rock => 1,
            GameChoice::Paper => 2,
            GameChoice::Scissors => 3,
        }
    }
}

pub fn calculate_score(enemy_choice: GameChoice, our_choice: GameChoice) -> i32 {
    use GameChoice::*;

    let mut score = our_choice.score();

    score += match (enemy_choice, our_choice) {
        // Loss
        (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => 0,
        // Draw
        (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3,
        // Win
        (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => 6,
    };

    score
}

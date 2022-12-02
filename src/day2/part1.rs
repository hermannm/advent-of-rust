#[allow(dead_code)]
pub fn solve_puzzle() -> Result<i32, String> {
    let input = include_str!("input.txt");

    input
        .lines()
        .map(|line| {
            let (enemy_choice, our_choice) = choices_from_input_line(line)?;
            Ok(calculate_score(enemy_choice, our_choice))
        })
        .sum()
}

fn choices_from_input_line(input_line: &str) -> Result<(GameChoice, GameChoice), String> {
    let choice_chars: Vec<char> = input_line.chars().collect();
    if choice_chars.len() < 3 {
        return Err("Input line did not contain enough characters".to_string());
    }

    let enemy_choice = GameChoice::try_from(choice_chars[0])?;
    let our_choice = GameChoice::try_from(choice_chars[2])?;

    Ok((enemy_choice, our_choice))
}

fn calculate_score(enemy_choice: GameChoice, our_choice: GameChoice) -> i32 {
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

enum GameChoice {
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

impl TryFrom<char> for GameChoice {
    type Error = String;

    fn try_from(choice_char: char) -> Result<Self, Self::Error> {
        match choice_char {
            'A' | 'X' => Ok(GameChoice::Rock),
            'B' | 'Y' => Ok(GameChoice::Paper),
            'C' | 'Z' => Ok(GameChoice::Scissors),
            _ => Err(format!("Invalid game choice character: {choice_char}")),
        }
    }
}

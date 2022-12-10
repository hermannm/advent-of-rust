use super::shared::{GameChoice, GameOutcome};

pub fn solve_puzzle(input: &str) -> Result<i32, String> {
    input
        .lines()
        .map(|line| {
            let (enemy_choice, our_choice) = parse_input_line(line)?;
            let outcome = GameOutcome::from_game_choices(&enemy_choice, &our_choice);
            Ok(our_choice.score() + outcome.score())
        })
        .sum()
}

fn parse_input_line(input_line: &str) -> Result<(GameChoice, GameChoice), String> {
    let input_chars: Vec<char> = input_line.chars().collect();
    if input_chars.len() < 3 {
        return Err("Input line did not contain enough characters".to_string());
    }

    let enemy_choice = GameChoice::try_from_char(input_chars[0])?;
    let our_choice = GameChoice::try_from_char(input_chars[2])?;

    Ok((enemy_choice, our_choice))
}

impl GameOutcome {
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

/// Re-implementation of TryFrom<char> to allow duplicate implementations in part 1 and part 2.
trait TryFromChar: Sized {
    type Error;

    fn try_from_char(choice_char: char) -> Result<Self, Self::Error>;
}

impl TryFromChar for GameChoice {
    type Error = String;

    fn try_from_char(choice_char: char) -> Result<Self, Self::Error> {
        match choice_char {
            'A' | 'X' => Ok(GameChoice::Rock),
            'B' | 'Y' => Ok(GameChoice::Paper),
            'C' | 'Z' => Ok(GameChoice::Scissors),
            _ => Err(format!("Invalid game choice character: {choice_char}")),
        }
    }
}

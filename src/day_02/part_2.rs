use super::rock_paper_scissors::{GameChoice, GameOutcome};

pub fn solve_puzzle(input: &str) -> Result<i32, String> {
    input
        .lines()
        .map(|line| {
            let (enemy_choice, game_outcome) = parse_input_line(line)?;
            let our_choice = GameChoice::from_game_outcome(&game_outcome, &enemy_choice);
            Ok(our_choice.score() + game_outcome.score())
        })
        .sum()
}

fn parse_input_line(input_line: &str) -> Result<(GameChoice, GameOutcome), String> {
    let input_chars: Vec<char> = input_line.chars().collect();
    if input_chars.len() < 3 {
        return Err("Input line did not contain enough characters".to_string());
    }

    let enemy_choice = GameChoice::try_from_char(input_chars[0])?;
    let game_outcome = GameOutcome::try_from(input_chars[2])?;

    Ok((enemy_choice, game_outcome))
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
            'A' => Ok(GameChoice::Rock),
            'B' => Ok(GameChoice::Paper),
            'C' => Ok(GameChoice::Scissors),
            _ => Err(format!("Invalid game choice character: {choice_char}")),
        }
    }
}

impl TryFrom<char> for GameOutcome {
    type Error = String;

    fn try_from(outcome_char: char) -> Result<Self, Self::Error> {
        match outcome_char {
            'X' => Ok(GameOutcome::Loss),
            'Y' => Ok(GameOutcome::Draw),
            'Z' => Ok(GameOutcome::Win),
            _ => Err(format!("Invalid game outcome character: {outcome_char}")),
        }
    }
}

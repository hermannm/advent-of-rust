use super::shared::{calculate_score, GameChoice};

pub fn solve_puzzle(input: &str) -> Result<i32, String> {
    input
        .lines()
        .map(|line| {
            let (enemy_choice, game_outcome) = parse_input_line(line)?;
            let our_choice = choose_from_outcome(&enemy_choice, &game_outcome);
            Ok(calculate_score(enemy_choice, our_choice))
        })
        .sum()
}

fn parse_input_line(input_line: &str) -> Result<(GameChoice, GameOutcome), String> {
    let input_chars: Vec<char> = input_line.chars().collect();
    if input_chars.len() < 3 {
        return Err("Input line did not contain enough characters".to_string());
    }

    let enemy_choice = game_choice_from_char(input_chars[0])?;
    let game_outcome = game_outcome_from_char(input_chars[2])?;

    Ok((enemy_choice, game_outcome))
}

fn game_choice_from_char(choice_char: char) -> Result<GameChoice, String> {
    match choice_char {
        'A' => Ok(GameChoice::Rock),
        'B' => Ok(GameChoice::Paper),
        'C' => Ok(GameChoice::Scissors),
        _ => Err(format!("Invalid game choice character: {choice_char}")),
    }
}

enum GameOutcome {
    Loss,
    Draw,
    Win,
}

fn game_outcome_from_char(outcome_char: char) -> Result<GameOutcome, String> {
    match outcome_char {
        'X' => Ok(GameOutcome::Loss),
        'Y' => Ok(GameOutcome::Draw),
        'Z' => Ok(GameOutcome::Win),
        _ => Err(format!("Invalid game outcome character: {outcome_char}")),
    }
}

fn choose_from_outcome(enemy_choice: &GameChoice, game_outcome: &GameOutcome) -> GameChoice {
    use GameChoice::*;
    use GameOutcome::*;

    match (enemy_choice, game_outcome) {
        (Rock, Draw) | (Paper, Loss) | (Scissors, Win) => Rock,
        (Rock, Win) | (Paper, Draw) | (Scissors, Loss) => Paper,
        (Rock, Loss) | (Paper, Win) | (Scissors, Draw) => Scissors,
    }
}

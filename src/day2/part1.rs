use super::shared::{calculate_score, GameChoice};

#[allow(dead_code)]
pub fn solve_puzzle() -> Result<i32, String> {
    let input = include_str!("input.txt");

    input
        .lines()
        .map(|line| {
            let (enemy_choice, our_choice) = parse_input_line(line)?;
            Ok(calculate_score(enemy_choice, our_choice))
        })
        .sum()
}

fn parse_input_line(input_line: &str) -> Result<(GameChoice, GameChoice), String> {
    let input_chars: Vec<char> = input_line.chars().collect();
    if input_chars.len() < 3 {
        return Err("Input line did not contain enough characters".to_string());
    }

    let enemy_choice = game_choice_from_char(input_chars[0])?;
    let our_choice = game_choice_from_char(input_chars[2])?;

    Ok((enemy_choice, our_choice))
}

fn game_choice_from_char(choice_char: char) -> Result<GameChoice, String> {
    match choice_char {
        'A' | 'X' => Ok(GameChoice::Rock),
        'B' | 'Y' => Ok(GameChoice::Paper),
        'C' | 'Z' => Ok(GameChoice::Scissors),
        _ => Err(format!("Invalid game choice character: {choice_char}")),
    }
}

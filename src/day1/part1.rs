use super::shared::get_calorie_totals;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum PuzzleError {
    ParsingError(ParseIntError),
}

#[allow(dead_code)]
pub fn solve_puzzle() -> Result<i32, PuzzleError> {
    let calorie_totals = get_calorie_totals().map_err(|error| PuzzleError::ParsingError(error))?;

    Ok(*calorie_totals.iter().max().unwrap_or(&0))
}

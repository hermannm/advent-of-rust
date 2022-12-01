use super::shared::get_calorie_totals;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum PuzzleError {
    ParsingError(ParseIntError),
}

#[allow(dead_code)]
pub fn solve_puzzle() -> Result<i32, PuzzleError> {
    let mut calorie_totals =
        get_calorie_totals().map_err(|error| PuzzleError::ParsingError(error))?;

    calorie_totals.sort_by(|total1, total2| total2.cmp(total1));

    let top_three_sum: i32 = calorie_totals.iter().take(3).sum();

    Ok(top_three_sum)
}

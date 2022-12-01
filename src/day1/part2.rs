use super::shared::get_calorie_totals;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum PuzzleError {
    ParsingError(ParseIntError),
}

#[allow(dead_code)]
pub fn solve_puzzle() -> Result<i32, PuzzleError> {
    let calorie_totals = get_calorie_totals().map_err(|error| PuzzleError::ParsingError(error))?;

    let mut top_three = [0, 0, 0];

    for calorie_total in calorie_totals {
        for top in top_three.iter_mut() {
            if calorie_total > *top {
                *top = calorie_total;
                break;
            }
        }
    }

    Ok(top_three.iter().sum())
}

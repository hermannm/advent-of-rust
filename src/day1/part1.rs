use super::shared::get_calorie_totals;

pub fn solve_puzzle(input: &str) -> Result<i32, String> {
    let calorie_totals = get_calorie_totals(input)?;

    Ok(*calorie_totals.iter().max().unwrap_or(&0))
}

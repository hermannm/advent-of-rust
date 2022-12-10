use super::parsing::parse_calorie_totals;

pub fn solve_puzzle(input: &str) -> Result<i32, String> {
    let calorie_totals = parse_calorie_totals(input)?;

    Ok(*calorie_totals.iter().max().unwrap_or(&0))
}

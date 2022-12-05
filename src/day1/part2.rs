use super::shared::get_calorie_totals;

pub fn solve_puzzle(input: &str) -> Result<i32, String> {
    let mut calorie_totals = get_calorie_totals(input)?;

    calorie_totals.sort_by(|total1, total2| total2.cmp(total1));

    let top_three_sum: i32 = calorie_totals.iter().take(3).sum();

    Ok(top_three_sum)
}

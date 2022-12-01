use std::num::ParseIntError;

pub fn get_calorie_totals() -> Result<Vec<i32>, ParseIntError> {
    let input = include_str!("input.txt");

    let mut calorie_totals = Vec::<i32>::new();

    for line in input.lines() {
        if line == "" || calorie_totals.is_empty() {
            calorie_totals.push(0);
            continue;
        }

        match line.parse::<i32>() {
            Ok(input_calories) => {
                let last_index = calorie_totals.len() - 1;
                calorie_totals[last_index] += input_calories;
            }
            Err(error) => return Err(error),
        }
    }

    Ok(calorie_totals)
}

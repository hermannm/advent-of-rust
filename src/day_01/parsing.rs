pub(super) fn parse_calorie_totals(input: &str) -> Result<Vec<i32>, String> {
    let mut calorie_totals = Vec::<i32>::new();

    for line in input.lines() {
        if line.is_empty() || calorie_totals.is_empty() {
            calorie_totals.push(0);
            continue;
        }

        match line.parse::<i32>() {
            Ok(input_calories) => {
                let last_index = calorie_totals.len() - 1;
                calorie_totals[last_index] += input_calories;
            }
            Err(_) => return Err(String::from("Failed to parse line input to integer")),
        }
    }

    Ok(calorie_totals)
}

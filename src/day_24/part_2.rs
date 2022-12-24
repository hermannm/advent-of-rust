use super::valley::Valley;

pub fn solve_puzzle(input: &str) -> Result<u32, String> {
    let mut valley = Valley::try_from(input)?;

    let first_trip_minutes = valley.find_fastest_path();
    valley.expedition.flip_start_and_goal_positions();
    let second_trip_minutes = valley.find_fastest_path();
    valley.expedition.flip_start_and_goal_positions();
    let third_trip_minutes = valley.find_fastest_path();

    Ok(first_trip_minutes + second_trip_minutes + third_trip_minutes)
}

use super::sensor_map::SensorMap;

pub fn solve_puzzle(input: &str, range_to_check: (i32, i32)) -> Result<i64, String> {
    let map = SensorMap::try_from(input)?;

    let beacon_position = map
        .find_first_possible_beacon(range_to_check, range_to_check)
        .ok_or_else(|| String::from("No possible beacon found"))?;

    Ok(beacon_position.tuning_signal())
}

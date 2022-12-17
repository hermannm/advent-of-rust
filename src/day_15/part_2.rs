use super::sensor_map::SensorMap;

pub fn solve_puzzle(input: &str) -> Result<i64, String> {
    let map = SensorMap::try_from(input)?;

    const RANGE_TO_CHECK: (i32, i32) = (0, 4_000_000);

    let beacon_position = map
        .find_first_possible_beacon(RANGE_TO_CHECK, RANGE_TO_CHECK)
        .ok_or_else(|| String::from("No possible beacon found"))?;

    Ok(beacon_position.tuning_signal())
}

use super::sensor_map::SensorMap;

pub fn solve_puzzle(input: &str, row_to_check: i32) -> Result<usize, String> {
    let map = SensorMap::try_from(input)?;

    let positions_to_check = map.get_positions_in_row(row_to_check)?;

    Ok(map.count_positions_without_beacons(positions_to_check))
}

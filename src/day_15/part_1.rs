use super::sensor_map::SensorMap;

pub fn solve_puzzle(input: &str) -> Result<usize, String> {
    let map = SensorMap::try_from(input)?;

    let positions_to_check = map.get_positions_in_row(2_000_000)?;

    Ok(map.count_positions_without_beacons(positions_to_check))
}

use super::valves::ValveMap;

pub fn solve_puzzle(input: &str) -> Result<u32, String> {
    let valve_map = ValveMap::try_from(input)?;

    let pressure_released = valve_map.release_maximum_pressure(30, "AA")?;

    Ok(pressure_released)
}

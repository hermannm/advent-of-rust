use super::valves::ValveMap;

pub fn solve_puzzle(input: &str) -> Result<i32, String> {
    let valve_map = ValveMap::try_from(input)?;

    println!("{valve_map:?}");

    Ok(0)
}

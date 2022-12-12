use super::height_map::HeightMap;

pub fn solve_puzzle(input: &str) -> Result<u32, String> {
    let height_map = HeightMap::try_from(input)?;

    let (_, steps) = height_map.find_shortest_path()?;

    Ok(steps)
}

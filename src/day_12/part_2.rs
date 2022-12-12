use super::height_map::HeightMap;

pub fn solve_puzzle(input: &str) -> Result<u32, String> {
    let mut height_map = HeightMap::try_from(input)?;

    let starting_positions = height_map.positions_at_lowest_elevation();
    let mut path_steps = Vec::<u32>::new();

    for starting_position in starting_positions {
        height_map.current_position = starting_position;
        if let Ok((_, steps)) = height_map.find_shortest_path() {
            path_steps.push(steps);
        }
    }

    let shortest_path = path_steps
        .iter()
        .min()
        .ok_or_else(|| String::from("No shortest path found"))?;

    Ok(*shortest_path)
}

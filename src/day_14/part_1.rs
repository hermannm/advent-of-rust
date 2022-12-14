use super::cave::Cave;

pub fn solve_puzzle(input: &str) -> Result<usize, String> {
    let mut cave = Cave::try_from(input)?;

    cave.fill_with_sand();

    Ok(cave.sand.len())
}

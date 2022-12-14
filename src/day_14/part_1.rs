use super::cave::Cave;

pub fn solve_puzzle(input: &str) -> Result<i32, String> {
    let cave = Cave::try_from(input)?;

    Ok(cave.lowest_rock)
}

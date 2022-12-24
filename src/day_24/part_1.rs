use super::valley::Valley;

pub fn solve_puzzle(input: &str) -> Result<u32, String> {
    let mut valley = Valley::try_from(input)?;

    Ok(valley.find_fastest_path())
}

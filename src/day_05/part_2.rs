use super::parsing::parse_input;

pub fn solve_puzzle(input: &str) -> Result<String, String> {
    let (mut crate_arrangement, move_operations) = parse_input(input)?;

    crate_arrangement.rearrange(&move_operations, false)?;

    Ok(crate_arrangement.get_top_crates_string())
}

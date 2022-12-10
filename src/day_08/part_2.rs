use super::forest::Forest;

pub fn solve_puzzle(input: &str) -> Result<usize, String> {
    let forest = Forest::try_from(input)?;

    Ok(forest.get_highest_scenic_score())
}

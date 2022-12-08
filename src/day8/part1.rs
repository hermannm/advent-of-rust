use super::forest::Forest;

pub fn solve_puzzle(input: &str) -> Result<u32, String> {
    let forest = Forest::try_from(input)?;

    Ok(forest.count_trees_visible_from_outside())
}

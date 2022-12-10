use super::ranges::RangePair;

pub fn solve_puzzle(input: &str) -> Result<i32, String> {
    let mut fully_contained_pairs = 0;

    for line in input.lines() {
        let pair = RangePair::try_from(line)?;

        if pair.range_1.fully_contains(&pair.range_2) || pair.range_2.fully_contains(&pair.range_1)
        {
            fully_contained_pairs += 1;
        }
    }

    Ok(fully_contained_pairs)
}

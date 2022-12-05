use super::shared::Pair;

pub fn solve_puzzle(input: &str) -> Result<i32, String> {
    let mut fully_contained_pairs = 0;

    for line in input.lines() {
        let pair = Pair::try_from(line)?;

        if pair.range1.fully_contains(&pair.range2) || pair.range2.fully_contains(&pair.range1) {
            fully_contained_pairs += 1;
        }
    }

    Ok(fully_contained_pairs)
}

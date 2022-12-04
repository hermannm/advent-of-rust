use super::shared::Pair;

#[allow(dead_code)]
pub fn solve_puzzle() -> Result<i32, String> {
    let input = include_str!("input.txt");

    let mut fully_contained_pairs = 0;

    for line in input.lines() {
        let pair = Pair::try_from(line)?;

        if pair.range1.fully_contains(&pair.range2) || pair.range2.fully_contains(&pair.range1) {
            fully_contained_pairs += 1;
        }
    }

    Ok(fully_contained_pairs)
}

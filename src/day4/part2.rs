use super::shared::Pair;

pub fn solve_puzzle(input: &str) -> Result<i32, String> {
    let mut overlapping_pairs = 0;

    for line in input.lines() {
        let pair = Pair::try_from(line)?;

        if pair.range1.overlaps(&pair.range2) {
            overlapping_pairs += 1;
        }
    }

    Ok(overlapping_pairs)
}

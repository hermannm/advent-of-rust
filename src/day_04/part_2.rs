use super::ranges::RangePair;

pub fn solve_puzzle(input: &str) -> Result<i32, String> {
    let mut overlapping_pairs = 0;

    for line in input.lines() {
        let pair = RangePair::try_from(line)?;

        if pair.range_1.overlaps(&pair.range_2) {
            overlapping_pairs += 1;
        }
    }

    Ok(overlapping_pairs)
}

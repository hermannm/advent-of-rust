use super::shared::{Pair, Range};

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

impl Range {
    pub fn overlaps(&self, other: &Range) -> bool {
        (self.min <= other.max && self.max >= other.min)
            || (other.min <= self.max && other.max >= self.min)
    }
}

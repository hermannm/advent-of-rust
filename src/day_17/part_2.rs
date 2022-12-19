use super::{falling_rocks::Chamber, parsing::jet_pattern_from_input};

pub fn solve_puzzle(input: &str) -> Result<usize, String> {
    let jet_pattern = jet_pattern_from_input(input)?;

    let mut chamber = Chamber::new(7);

    chamber.get_height_of_fallen_rocks(1_000_000_000_000, jet_pattern)
}

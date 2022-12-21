use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use super::blueprint::Blueprint;

pub fn solve_puzzle(input: &str) -> Result<u32, String> {
    let blueprints = input
        .lines()
        .map(Blueprint::try_from)
        .collect::<Result<Vec<Blueprint>, String>>()?;

    const MINUTES_TO_OPEN_GEODES: u16 = 24;

    let quality_level_sum = blueprints
        .par_iter()
        .map(|blueprint| blueprint.quality_level(MINUTES_TO_OPEN_GEODES))
        .sum::<u32>();

    Ok(quality_level_sum)
}

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use super::blueprint::Blueprint;

pub fn solve_puzzle(input: &str) -> Result<u32, String> {
    let blueprints = Blueprint::blueprints_from_input(input)?;

    const MINUTES_TO_OPEN_GEODES: u32 = 24;

    let quality_level_sum = blueprints
        .par_iter()
        .map(|blueprint| {
            let quality_level = blueprint.quality_level(MINUTES_TO_OPEN_GEODES);
            println!(
                "Blueprint {} quality level: {}",
                blueprint.id, quality_level
            );
            quality_level
        })
        .sum::<u32>();

    Ok(quality_level_sum)
}

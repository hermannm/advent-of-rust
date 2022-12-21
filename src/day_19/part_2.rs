use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use super::blueprint::Blueprint;

pub fn solve_puzzle(input: &str) -> Result<u16, String> {
    let blueprints = input
        .lines()
        .take(3)
        .map(Blueprint::try_from)
        .collect::<Result<Vec<Blueprint>, String>>()?;

    const MINUTES_TO_OPEN_GEODES: u16 = 32;

    let max_geodes_multiplied = blueprints
        .par_iter()
        .map(|blueprint| blueprint.max_geodes(MINUTES_TO_OPEN_GEODES))
        .product::<u16>();

    Ok(max_geodes_multiplied)
}

use std::{
    collections::HashMap,
    iter::{Cycle, Enumerate, Peekable},
    slice::Iter,
};

use super::falling_rocks::{Chamber, Movement, RockShape};

pub struct Cache {
    entries: HashMap<CacheKey, CacheValue>,
    computed_height: Option<usize>,
}

#[derive(PartialEq, Eq, Hash)]
struct CacheKey {
    rock_shape_index: usize,
    jet_index: usize,
    column_height_differences_from_chamber_height: Vec<usize>,
}

struct CacheValue {
    fallen_rocks_count: usize,
    chamber_height: usize,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            entries: HashMap::<CacheKey, CacheValue>::new(),
            computed_height: None,
        }
    }

    pub fn get_computed_height(&self) -> usize {
        self.computed_height.unwrap_or(0)
    }

    pub fn compute_height(
        &mut self,
        chamber: &Chamber,
        total_number_of_rocks: usize,
        fallen_rocks_count_mut: &mut usize,
        rock_shape_stream: &mut Peekable<Cycle<Enumerate<Iter<RockShape>>>>,
        jet_stream: &mut Peekable<Cycle<Enumerate<Iter<Movement>>>>,
    ) -> Result<(), String> {
        if self.computed_height.is_some() {
            return Ok(());
        }

        let &(next_rock_shape_index, _) = rock_shape_stream
            .peek()
            .ok_or_else(|| String::from("Rock shape stream stopped unexpectedly"))?;

        let &(next_jet_index, _) = jet_stream
            .peek()
            .ok_or_else(|| String::from("Rock shape stream stopped unexpectedly"))?;

        let chamber_height = chamber.height();

        let cache_key = CacheKey {
            rock_shape_index: next_rock_shape_index,
            jet_index: next_jet_index,
            column_height_differences_from_chamber_height: chamber
                .column_height_differences_from_chamber_height(),
        };

        let fallen_rocks_count = *fallen_rocks_count_mut;

        let cache_value = CacheValue {
            fallen_rocks_count,
            chamber_height,
        };

        if let Some(previous_value) = self.entries.insert(cache_key, cache_value) {
            let cycle_length = fallen_rocks_count - previous_value.fallen_rocks_count;
            let cycles_remaining = (total_number_of_rocks - fallen_rocks_count) / cycle_length;

            let rocks_to_skip = cycles_remaining * cycle_length;
            *fallen_rocks_count_mut += rocks_to_skip;

            self.computed_height =
                Some((chamber_height - previous_value.chamber_height) * cycles_remaining);
        }

        Ok(())
    }
}

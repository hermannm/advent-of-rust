use std::{cmp::Ordering, collections::HashSet};

use super::cave::{Cave, Position};

impl TryFrom<&str> for Cave {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut rocks = HashSet::<Position>::new();

        for line in input.lines() {
            let position_strings = line.split(" -> ");

            let mut previous_position: Option<Position> = None;

            for position_string in position_strings {
                let position = Position::try_from(position_string)?;

                rocks.insert(position.clone());

                if let Some(previous_position) = previous_position {
                    fill_rocks_inbetween(&mut rocks, &previous_position, &position);
                }

                previous_position = Some(position);
            }
        }

        let lowest_rock = rocks
            .iter()
            .map(|rock_position| rock_position.y)
            .max()
            .ok_or_else(|| String::from("Failed to find lowest rock in cave"))?;

        Ok(Cave {
            rocks,
            lowest_rock,
            has_floor: false,
            sand: HashSet::<Position>::new(),
        })
    }
}

impl TryFrom<&str> for Position {
    type Error = String;

    fn try_from(position_string: &str) -> Result<Self, Self::Error> {
        let (x_string, y_string) = position_string.split_once(',').ok_or_else(|| {
            format!("Failed to split position string '{position_string}' into x and y")
        })?;

        let x = x_string
            .parse::<i32>()
            .map_err(|_| format!("Failed to parse x value from string '{x_string}'"))?;

        let y = y_string
            .parse::<i32>()
            .map_err(|_| format!("Failed to parse y value from string '{y_string}'"))?;

        Ok(Position { x, y })
    }
}

fn fill_rocks_inbetween(rocks: &mut HashSet<Position>, rock_1: &Position, rock_2: &Position) {
    use Ordering::*;

    match (rock_1.x.cmp(&rock_2.x), rock_1.y.cmp(&rock_2.y)) {
        (Equal, y_comparison) => {
            {
                if y_comparison == Less {
                    rock_1.y..rock_2.y
                } else {
                    rock_2.y..rock_1.y
                }
            }
            .map(|y| Position { x: rock_2.x, y })
            .for_each(|position| {
                rocks.insert(position);
            });
        }
        (x_comparison, Equal) => {
            {
                if x_comparison == Less {
                    rock_1.x..rock_2.x
                } else {
                    rock_2.x..rock_1.x
                }
            }
            .map(|x| Position { x, y: rock_2.y })
            .for_each(|position| {
                rocks.insert(position);
            });
        }
        _ => {}
    }
}

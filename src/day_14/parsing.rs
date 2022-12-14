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
                let (x_string, y_string) = position_string.split_once(',').ok_or_else(|| {
                    format!("Failed to split position string '{position_string}' into x and y")
                })?;

                let x = x_string
                    .parse::<i32>()
                    .map_err(|_| format!("Failed to parse x value from string '{x_string}'"))?;

                let y = y_string
                    .parse::<i32>()
                    .map_err(|_| format!("Failed to parse x value from string '{y_string}'"))?;

                let position = Position { x, y };

                rocks.insert(position.clone());

                if let Some(previous_position) = previous_position {
                    use Ordering::*;

                    match (
                        previous_position.x.cmp(&position.x),
                        previous_position.y.cmp(&position.y),
                    ) {
                        (Equal, y_comparison) => {
                            {
                                if y_comparison == Less {
                                    previous_position.y..position.y
                                } else {
                                    position.y..previous_position.y
                                }
                            }
                            .map(|y| Position { x: position.x, y })
                            .for_each(|position| {
                                rocks.insert(position);
                            });
                        }
                        (x_comparison, Equal) => {
                            {
                                if x_comparison == Less {
                                    previous_position.x..position.x
                                } else {
                                    position.x..previous_position.x
                                }
                            }
                            .map(|x| Position { x, y: position.y })
                            .for_each(|position| {
                                rocks.insert(position);
                            });
                        }
                        _ => {}
                    }
                }

                previous_position = Some(position);
            }
        }

        let lowest_rock = rocks
            .iter()
            .map(|rock_position| rock_position.y)
            .min()
            .ok_or_else(|| String::from("Failed to find lowest rock in cave"))?;

        Ok(Cave {
            rocks,
            lowest_rock,
            sand: HashSet::<Position>::new(),
        })
    }
}

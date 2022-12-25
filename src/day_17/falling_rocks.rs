use std::{
    collections::HashSet,
    iter::{Cycle, Enumerate, Peekable},
    slice::Iter,
};

use super::cache::Cache;

pub(super) struct Chamber {
    width: usize,
    fallen_rocks: Vec<Rock>,
}

impl Chamber {
    pub fn new(width: usize) -> Self {
        Self {
            width,
            fallen_rocks: Vec::<Rock>::new(),
        }
    }

    pub fn get_height_of_fallen_rocks(
        &mut self,
        number_of_rocks: usize,
        jet_pattern: Vec<Movement>,
    ) -> Result<usize, String> {
        let rock_shapes = RockShape::default_shapes();
        let mut rock_shape_stream = rock_shapes.iter().enumerate().cycle().peekable();
        let mut jet_stream = jet_pattern.iter().enumerate().cycle().peekable();

        let mut fallen_rocks_count = 0;
        let mut cache = Cache::new();

        while fallen_rocks_count < number_of_rocks {
            fallen_rocks_count += 1;

            self.let_rock_fall(&mut rock_shape_stream, &mut jet_stream)?;

            cache.compute_height(
                self,
                number_of_rocks,
                &mut fallen_rocks_count,
                &mut rock_shape_stream,
                &mut jet_stream,
            )?;
        }

        Ok(self.height() + cache.get_computed_height())
    }

    fn let_rock_fall(
        &mut self,
        rock_shape_stream: &mut Peekable<Cycle<Enumerate<Iter<RockShape>>>>,
        jet_stream: &mut Peekable<Cycle<Enumerate<Iter<Movement>>>>,
    ) -> Result<(), String> {
        let (_, shape) = rock_shape_stream
            .next()
            .ok_or_else(|| String::from("Rock shape stream stopped unexpectedly"))?;

        let mut falling_rock = Rock {
            shape: shape.clone(),
            position: (2, self.height() + 3),
        };

        loop {
            let (_, jet) = jet_stream
                .next()
                .ok_or_else(|| String::from("Jet stream stopped unexpectedly"))?;

            falling_rock.attempt_move(jet, self);

            let has_landed = falling_rock.attempt_move(&Movement::Down, self);
            if has_landed {
                self.fallen_rocks.push(falling_rock);
                break;
            }
        }

        Ok(())
    }

    pub fn height(&self) -> usize {
        self.fallen_rocks
            .iter()
            .map(|rock| rock.position.1 + rock.shape.height)
            .max()
            .unwrap_or(0)
    }

    pub fn column_height_differences_from_chamber_height(&self) -> Vec<usize> {
        let height = self.height();

        (0..self.width)
            .map(|column| height - self.column_height(column))
            .collect::<Vec<usize>>()
    }

    fn column_height(&self, column: usize) -> usize {
        self.fallen_rocks
            .iter()
            .flat_map(|rock| rock.point_positions())
            .filter_map(|(x, y)| if x == column { Some(y) } else { None })
            .max()
            .unwrap_or(0)
    }

    fn fallen_rocks_overlap_positions(&self, positions: impl Iterator<Item = Position>) -> bool {
        let position_set = HashSet::<Position>::from_iter(positions);

        self.fallen_rocks.iter().any(|rock| {
            let rock_point_position_set = HashSet::<Position>::from_iter(rock.point_positions());

            rock_point_position_set.intersection(&position_set).count() != 0
        })
    }
}

pub(super) enum Movement {
    Left,
    Right,
    Down,
}

pub(super) struct Rock {
    shape: RockShape,
    position: Position,
}

/// Position in the chamber of falling rocks.
/// The first number is the distance from the left wall in the chamber,
/// while the second number is the distance from the bottom.
type Position = (usize, usize);

impl Rock {
    fn attempt_move(&mut self, movement: &Movement, chamber: &Chamber) -> bool {
        match movement {
            Movement::Left => {
                if self.position.0 == 0 {
                    return false;
                }

                let new_positions = self.point_positions().map(|(x, y)| (x - 1, y));

                if !chamber.fallen_rocks_overlap_positions(new_positions) {
                    self.position.0 -= 1;
                }

                false
            }
            Movement::Right => {
                if self.position.0 + self.shape.width == chamber.width {
                    return false;
                }

                let new_positions = self.point_positions().map(|(x, y)| (x + 1, y));

                if !chamber.fallen_rocks_overlap_positions(new_positions) {
                    self.position.0 += 1;
                }

                false
            }
            Movement::Down => {
                if self.position.1 == 0 {
                    return true;
                }

                let new_positions = self.point_positions().map(|(x, y)| (x, y - 1));

                let has_landed = chamber.fallen_rocks_overlap_positions(new_positions);

                if !has_landed {
                    self.position.1 -= 1;
                }

                has_landed
            }
        }
    }

    fn point_positions(&self) -> impl Iterator<Item = Position> + '_ {
        self.shape
            .grid
            .iter()
            .enumerate()
            .flat_map(move |(y_index, shape_row)| {
                shape_row
                    .iter()
                    .enumerate()
                    .flat_map(move |(x_index, &is_rock)| {
                        if is_rock {
                            Some((self.position.0 + x_index, self.position.1 + y_index))
                        } else {
                            None
                        }
                    })
            })
    }
}

#[derive(Clone)]
pub(super) struct RockShape {
    /// A 4x4 grid of booleans, where true == rock and false == air.
    /// If shape does not fill the whole grid, it starts from the top left [0, 0].
    grid: [[bool; 4]; 4],
    width: usize,
    height: usize,
}

impl RockShape {
    fn default_shapes() -> Vec<RockShape> {
        vec![
            RockShape {
                grid: [
                    [true, true, true, true],
                    [false, false, false, false],
                    [false, false, false, false],
                    [false, false, false, false],
                ],
                width: 4,
                height: 1,
            },
            RockShape {
                grid: [
                    [false, true, false, false],
                    [true, true, true, false],
                    [false, true, false, false],
                    [false, false, false, false],
                ],
                width: 3,
                height: 3,
            },
            RockShape {
                grid: [
                    [true, true, true, false],
                    [false, false, true, false],
                    [false, false, true, false],
                    [false, false, false, false],
                ],
                width: 3,
                height: 3,
            },
            RockShape {
                grid: [
                    [true, false, false, false],
                    [true, false, false, false],
                    [true, false, false, false],
                    [true, false, false, false],
                ],
                width: 1,
                height: 4,
            },
            RockShape {
                grid: [
                    [true, true, false, false],
                    [true, true, false, false],
                    [false, false, false, false],
                    [false, false, false, false],
                ],
                width: 2,
                height: 2,
            },
        ]
    }
}

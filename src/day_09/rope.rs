use std::collections::HashSet;

use super::movement::{Direction, Movement};

pub struct Rope {
    pub knots: Vec<Knot>,
}

impl Rope {
    pub fn new(number_of_knots: u32) -> Result<Self, String> {
        if number_of_knots < 2 {
            return Err("Rope must contain at least 2 knots".to_string());
        }

        let starting_position = Position { x: 0, y: 0 };
        let mut knots = Vec::<Knot>::new();

        for _ in 0..number_of_knots {
            knots.push(Knot::new(starting_position.clone()));
        }

        Ok(Self { knots })
    }

    pub fn tail(&self) -> &Knot {
        self.knots
            .last()
            .expect("Rope should always contain 2 knots for head and tail")
    }

    pub fn move_rope(&mut self, movement: &Movement) {
        let mut steps_left = movement.steps;

        while steps_left != 0 {
            let mut previous_knot_position: Option<Position> = None;

            for knot in self.knots.iter_mut() {
                let Position { x, y } = knot.position;

                match previous_knot_position {
                    None => {
                        match movement.direction {
                            Direction::Up => {
                                knot.move_to(Position { x, y: y + 1 });
                            }
                            Direction::Down => {
                                knot.move_to(Position { x, y: y - 1 });
                            }
                            Direction::Left => {
                                knot.move_to(Position { x: x - 1, y });
                            }
                            Direction::Right => {
                                knot.move_to(Position { x: x + 1, y });
                            }
                        };
                    }
                    Some(previous_knot_position) => {
                        if !knot
                            .position
                            .is_adjacent_or_overlaps(&previous_knot_position)
                        {
                            let Position {
                                x: prev_x,
                                y: prev_y,
                            } = previous_knot_position;

                            if x == prev_x && y < prev_y {
                                knot.move_to(Position { x, y: y + 1 });
                            } else if x == prev_x && y > prev_y {
                                knot.move_to(Position { x, y: y - 1 });
                            } else if y == prev_y && x < prev_x {
                                knot.move_to(Position { x: x + 1, y });
                            } else if y == prev_y && x > prev_x {
                                knot.move_to(Position { x: x - 1, y });
                            } else if x < prev_x && y < prev_y {
                                knot.move_to(Position { x: x + 1, y: y + 1 });
                            } else if x < prev_x && y > prev_y {
                                knot.move_to(Position { x: x + 1, y: y - 1 });
                            } else if x > prev_x && y > prev_y {
                                knot.move_to(Position { x: x - 1, y: y - 1 });
                            } else if x > prev_x && y < prev_y {
                                knot.move_to(Position { x: x - 1, y: y + 1 });
                            }
                        }
                    }
                }

                previous_knot_position = Some(knot.position);
            }

            steps_left -= 1;
        }

        dbg!(&self.tail().position);
    }
}

pub struct Knot {
    pub position: Position,
    pub visited_positions: HashSet<Position>,
}

impl Knot {
    fn new(starting_position: Position) -> Self {
        let mut visited_positions = HashSet::<Position>::new();
        visited_positions.insert(starting_position);

        Self {
            position: starting_position,
            visited_positions,
        }
    }

    fn move_to(&mut self, position: Position) {
        self.position = position;
        self.visited_positions.insert(position);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn is_adjacent_or_overlaps(&self, other: &Position) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }
}

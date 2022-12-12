use std::{cmp::Ordering, collections::HashSet};

use super::movement::{Direction, Movement};

pub struct Rope {
    pub knots: Vec<Knot>,
}

impl Rope {
    pub fn new(number_of_knots: u32) -> Result<Self, String> {
        if number_of_knots < 2 {
            return Err(String::from("Rope must contain at least 2 knots"));
        }

        let starting_position = Position { x: 0, y: 0 };
        let mut knots = Vec::<Knot>::new();

        for _ in 0..number_of_knots {
            knots.push(Knot::new(starting_position));
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
                match previous_knot_position {
                    None => {
                        knot.move_in_direction(&movement.direction);
                    }
                    Some(previous_knot_position) => {
                        knot.follow_previous_knot(&previous_knot_position);
                    }
                }

                previous_knot_position = Some(knot.position);
            }

            steps_left -= 1;
        }
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

    fn move_in_direction(&mut self, direction: &Direction) {
        let Position { x, y } = self.position;

        use Direction::*;

        let new_position = match direction {
            Up => Position { x, y: y + 1 },
            Down => Position { x, y: y - 1 },
            Left => Position { x: x - 1, y },
            Right => Position { x: x + 1, y },
        };

        self.move_to(new_position);
    }

    fn follow_previous_knot(&mut self, previous_knot_position: &Position) {
        if self.position.is_adjacent(previous_knot_position) {
            return;
        }

        let Position { x, y } = self.position;

        use Ordering::*;

        let new_position = match (
            x.cmp(&previous_knot_position.x),
            y.cmp(&previous_knot_position.y),
        ) {
            (Equal, Less) => Position { x, y: y + 1 },
            (Equal, Greater) => Position { x, y: y - 1 },
            (Less, Equal) => Position { x: x + 1, y },
            (Greater, Equal) => Position { x: x - 1, y },
            (Less, Less) => Position { x: x + 1, y: y + 1 },
            (Less, Greater) => Position { x: x + 1, y: y - 1 },
            (Greater, Greater) => Position { x: x - 1, y: y - 1 },
            (Greater, Less) => Position { x: x - 1, y: y + 1 },
            (Equal, Equal) => return,
        };

        self.move_to(new_position);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn is_adjacent(&self, other: &Position) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }
}

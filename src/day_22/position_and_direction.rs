#[derive(Clone, PartialEq, Eq, Hash)]
pub(super) struct Position {
    pub row: i64,
    pub column: i64,
}

impl Position {
    pub fn other_position_in_direction(&self, direction: &Direction, steps: i64) -> Position {
        use Direction::*;

        let mut neighbor_position = self.clone();

        match direction {
            North => {
                neighbor_position.row -= steps;
            }
            East => {
                neighbor_position.column += steps;
            }
            South => {
                neighbor_position.row += steps;
            }
            West => {
                neighbor_position.column -= steps;
            }
        }

        neighbor_position
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub(super) enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn turn_right(&mut self) {
        use Direction::*;

        match self {
            North => {
                *self = East;
            }
            East => {
                *self = South;
            }
            South => {
                *self = West;
            }
            West => {
                *self = North;
            }
        }
    }

    pub fn turn_left(&mut self) {
        use Direction::*;

        match self {
            North => {
                *self = West;
            }
            East => {
                *self = North;
            }
            South => {
                *self = East;
            }
            West => {
                *self = South;
            }
        }
    }

    pub fn opposite(&self) -> Direction {
        use Direction::*;

        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}

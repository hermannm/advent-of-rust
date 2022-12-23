use super::{
    map::{Map, Tile},
    position_and_direction::{Direction, Position},
};

pub enum Instruction {
    Forward { steps: i64 },
    Right,
    Left,
}

pub struct Mover {
    position: Position,
    direction: Direction,
}

impl Mover {
    pub fn new(map: &Map) -> Result<Self, String> {
        let position = map.top_left_position()?;

        Ok(Self {
            position,
            direction: Direction::East,
        })
    }

    pub fn follow_instruction(
        &mut self,
        instruction: &Instruction,
        map: &Map,
    ) -> Result<(), String> {
        use Instruction::*;

        match instruction {
            Forward { steps } => {
                self.move_forward(*steps, map)?;
            }
            Right => {
                self.direction.turn_right();
            }
            Left => {
                self.direction.turn_left();
            }
        }

        Ok(())
    }

    fn move_forward(&mut self, steps: i64, map: &Map) -> Result<(), String> {
        let mut steps_completed: i64 = 0;

        while steps_completed < steps {
            steps_completed += 1;

            let (tile, position, direction) =
                map.next_tile_in_direction(&self.position, &self.direction)?;

            match tile {
                Tile::Open => {
                    self.position = position;
                    self.direction = direction;
                }
                Tile::Wall => {
                    break;
                }
            }
        }

        Ok(())
    }

    pub fn get_password(&self) -> i64 {
        use Direction::*;

        let row_score = 1000 * (self.position.row + 1);

        let column_score = 4 * (self.position.column + 1);

        let direction_score = match self.direction {
            North => 3,
            East => 0,
            South => 1,
            West => 2,
        };

        row_score + column_score + direction_score
    }
}

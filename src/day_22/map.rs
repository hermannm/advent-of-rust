pub struct Map {
    tiles: Tiles,
    map_type: MapType,
}

enum MapType {
    Flat,
    Cube { side_length: u16 },
}

pub type Tiles = Vec<Vec<Option<Tile>>>;

#[derive(Clone)]
pub enum Tile {
    Open,
    Wall,
}

impl Map {
    pub fn new(tiles: Tiles, cube_side_length: Option<u16>) -> Self {
        use MapType::*;

        let map_type = match cube_side_length {
            Some(side_length) => Cube { side_length },
            None => Flat,
        };

        Self { tiles, map_type }
    }

    fn get(&self, row: i32, column: i32) -> Option<Tile> {
        let row_index = usize::try_from(row).ok()?;
        let column_index = usize::try_from(column).ok()?;

        self.tiles.get(row_index)?.get(column_index)?.to_owned()
    }

    fn top_left_position(&self) -> Result<Position, String> {
        for (row_index, rows) in self.tiles.iter().enumerate() {
            for (column_index, tile) in rows.iter().enumerate() {
                if let Some(Tile::Open) = tile {
                    let row = i32::try_from(row_index).map_err(|_| {
                        format!("Failed to convert index '{row_index}' to signed integer")
                    })?;

                    let column = i32::try_from(column_index).map_err(|_| {
                        format!("Failed to convert index '{column_index}' to signed integer")
                    })?;

                    return Ok(Position { row, column });
                }
            }
        }

        Err(String::from("Failed to find top-left position in map"))
    }

    fn next_tile_in_direction(
        &self,
        current_position: &Position,
        direction: &Direction,
    ) -> (Tile, Position, Direction) {
        let next_position = current_position.get_neighbor(direction);

        if let Some(tile) = self.get(next_position.row, next_position.column) {
            return (tile, next_position, direction.clone());
        }

        use MapType::*;

        match self.map_type {
            Flat => {
                let (tile, next_position) = self.wrap_flat(current_position, direction);
                (tile, next_position, direction.clone())
            }
            Cube { side_length } => self.wrap_cube(current_position, direction, side_length),
        }
    }

    fn wrap_flat(&self, current_position: &Position, direction: &Direction) -> (Tile, Position) {
        let mut next_position = current_position.clone();

        let mut tile_on_other_side = self
            .get(current_position.row, current_position.column)
            .expect("Current position should always be an open tile");

        let opposite_direction = direction.opposite();

        loop {
            let neighbor = next_position.get_neighbor(&opposite_direction);

            if let Some(tile) = self.get(neighbor.row, neighbor.column) {
                tile_on_other_side = tile;
                next_position = neighbor;
            } else {
                break;
            }
        }

        (tile_on_other_side, next_position)
    }

    fn wrap_cube(
        &self,
        current_position: &Position,
        current_direction: &Direction,
        cube_side_length: u16,
    ) -> (Tile, Position, Direction) {
        todo!()
    }
}

pub enum Instruction {
    Forward { steps: i32 },
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

    pub fn follow_instruction(&mut self, instruction: &Instruction, map: &Map) {
        use Instruction::*;

        match instruction {
            Forward { steps } => {
                self.move_forward(*steps, map);
            }
            Right => {
                self.direction.turn_right();
            }
            Left => {
                self.direction.turn_left();
            }
        }
    }

    fn move_forward(&mut self, steps: i32, map: &Map) {
        let mut steps_completed: i32 = 0;

        while steps_completed < steps {
            steps_completed += 1;

            let (tile, position, direction) =
                map.next_tile_in_direction(&self.position, &self.direction);

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
    }

    pub fn get_password(&self) -> i32 {
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

#[derive(Clone)]
struct Position {
    row: i32,
    column: i32,
}

impl Position {
    fn get_neighbor(&self, direction: &Direction) -> Position {
        use Direction::*;

        let mut neighbor_position = self.clone();

        match direction {
            North => {
                neighbor_position.row -= 1;
            }
            East => {
                neighbor_position.column += 1;
            }
            South => {
                neighbor_position.row += 1;
            }
            West => {
                neighbor_position.column -= 1;
            }
        }

        neighbor_position
    }
}

#[derive(Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(&mut self) {
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

    fn turn_left(&mut self) {
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

    fn opposite(&self) -> Direction {
        use Direction::*;

        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}

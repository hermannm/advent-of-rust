use std::collections::HashSet;

pub(super) struct Cave {
    pub rocks: HashSet<Position>,
    pub lowest_rock: i32,
    pub has_floor: bool,

    pub sand: HashSet<Position>,
}

impl Cave {
    pub fn fill_with_sand(&mut self) {
        let sand_start_position = Position { x: 500, y: 0 };

        let mut current_sand = sand_start_position.clone();

        'sand_loop: loop {
            if current_sand.y == self.lowest_rock || self.sand.contains(&sand_start_position) {
                return;
            }

            if !self.has_floor || current_sand.y != self.lowest_rock - 1 {
                for position in [
                    current_sand.below(),
                    current_sand.below_left(),
                    current_sand.below_right(),
                ] {
                    if !self.rocks.contains(&position) && !self.sand.contains(&position) {
                        current_sand = position;
                        continue 'sand_loop;
                    }
                }
            }

            self.sand.insert(current_sand);
            current_sand = sand_start_position.clone();
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub(super) struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    fn below(&self) -> Position {
        Position {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn below_left(&self) -> Position {
        Position {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    fn below_right(&self) -> Position {
        Position {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}

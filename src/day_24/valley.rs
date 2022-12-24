use std::collections::HashSet;

pub struct Valley {
    pub expedition: Expedition,
    pub blizzards: Vec<Blizzard>,
    pub wall_positions: HashSet<Position>,

    pub width: usize,
    pub height: usize,
}

impl Valley {
    pub fn find_fastest_path(&mut self) -> u32 {
        let mut minutes_passed: u32 = 0;
        let goal_position = self.expedition.goal_position.clone();

        let mut possible_expedition_positions = HashSet::<Position>::new();
        possible_expedition_positions.insert(self.expedition.start_position.clone());

        loop {
            minutes_passed += 1;

            self.move_blizzards();

            let occupied_positions = self.get_occupied_positions();

            possible_expedition_positions = Expedition::next_possible_positions(
                possible_expedition_positions,
                &occupied_positions,
            );

            if possible_expedition_positions.contains(&goal_position) {
                return minutes_passed;
            }
        }
    }

    fn move_blizzards(&mut self) {
        for blizzard in self.blizzards.iter_mut() {
            blizzard.move_in_valley(&self.wall_positions, self.width, self.height);
        }
    }

    fn get_occupied_positions(&self) -> HashSet<&Position> {
        let mut occupied_positions =
            HashSet::<&Position>::with_capacity(self.wall_positions.len() + self.blizzards.len());

        for wall_position in &self.wall_positions {
            occupied_positions.insert(wall_position);
        }

        for blizzard in &self.blizzards {
            occupied_positions.insert(&blizzard.position);
        }

        occupied_positions
    }
}

pub struct Expedition {
    pub start_position: Position,
    pub goal_position: Position,
}

impl Expedition {
    fn next_possible_positions(
        previous_positions: HashSet<Position>,
        occupied_positions: &HashSet<&Position>,
    ) -> HashSet<Position> {
        let mut next_positions = HashSet::<Position>::with_capacity(previous_positions.len() * 2);

        for previous_position in previous_positions {
            use Direction::*;

            let open_adjacent_positions = [
                previous_position.new_position_in_direction(&North),
                previous_position.new_position_in_direction(&East),
                previous_position.new_position_in_direction(&South),
                previous_position.new_position_in_direction(&West),
                previous_position,
            ]
            .into_iter()
            .filter(|position| !occupied_positions.contains(position));

            next_positions.extend(open_adjacent_positions);
        }

        next_positions
    }

    pub fn flip_start_and_goal_positions(&mut self) {
        let new_goal_position = self.start_position.clone();
        self.start_position = self.goal_position.clone();
        self.goal_position = new_goal_position;
    }
}

pub struct Blizzard {
    pub position: Position,
    pub direction: Direction,
}

impl Blizzard {
    fn move_in_valley(
        &mut self,
        wall_positions: &HashSet<Position>,
        valley_width: usize,
        valley_height: usize,
    ) {
        let new_position = self.position.new_position_in_direction(&self.direction);

        if wall_positions.contains(&new_position) {
            self.move_to_other_side_of_valley(valley_width, valley_height);
        } else {
            self.position = new_position;
        }
    }

    fn move_to_other_side_of_valley(&mut self, valley_width: usize, valley_height: usize) {
        match self.direction {
            Direction::North => {
                self.position.y = valley_height - 2;
            }
            Direction::East => {
                self.position.x = 1;
            }
            Direction::South => {
                self.position.y = 2;
            }
            Direction::West => {
                self.position.x = valley_width - 2;
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Position {
    fn new_position_in_direction(&self, direction: &Direction) -> Position {
        use Direction::*;

        let &Position { x, y } = self;

        match direction {
            North => Position { x, y: y - 1 },
            East => Position { x: x + 1, y },
            South => Position { x, y: y + 1 },
            West => Position { x: x - 1, y },
        }
    }
}

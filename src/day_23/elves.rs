use std::collections::HashMap;

pub struct Elves(pub Vec<Elf>);

impl Elves {
    pub fn move_to_open_ground(&mut self, rounds: u32) {
        let mut current_round: u32 = 1;

        while current_round <= rounds {
            let proposed_moves = self.propose_moves(current_round);
            let filtered_moves = Elves::filter_moves(&proposed_moves);
            self.execute_moves(filtered_moves);

            current_round += 1;
        }
    }

    pub fn find_round_where_no_elf_moved(&mut self) -> u32 {
        let mut current_round: u32 = 1;

        loop {
            let proposed_moves = self.propose_moves(current_round);
            let filtered_moves = Elves::filter_moves(&proposed_moves);

            if filtered_moves.is_empty() {
                return current_round;
            }

            self.execute_moves(filtered_moves);

            current_round += 1;
        }
    }

    fn propose_moves(&self, round: u32) -> HashMap<Elf, Option<Position>> {
        let mut proposed_moves = HashMap::<Elf, Option<Position>>::new();

        for elf in &self.0 {
            let proposed_move = elf.propose_move(round, self.all_except(elf));
            proposed_moves.insert(elf.clone(), proposed_move);
        }

        proposed_moves
    }

    fn filter_moves(proposed_moves: &HashMap<Elf, Option<Position>>) -> HashMap<&Position, &Elf> {
        let mut filtered_moves = HashMap::<&Position, &Elf>::new();
        let mut contested_moves = Vec::<&Position>::new();

        for (elf, proposed_move) in proposed_moves {
            let Some(proposed_move) = proposed_move else {
                continue;
            };

            if let Some(_already_inserted) = filtered_moves.insert(proposed_move, elf) {
                contested_moves.push(proposed_move);
            }
        }

        for contested in contested_moves {
            filtered_moves.remove(contested);
        }

        filtered_moves
    }

    fn execute_moves(&mut self, filtered_moves: HashMap<&Position, &Elf>) {
        for (new_position, elf) in filtered_moves {
            let elf = self
                .0
                .iter_mut()
                .find(|other_elf| other_elf == &elf)
                .expect("Expected to find elf from filtered moves in main elf list");

            elf.position = *new_position;
        }
    }

    pub fn count_empty_ground(&self) -> Result<usize, String> {
        let x_iterator = self.0.iter().map(|elf| elf.position.x);
        let y_iterator = self.0.iter().map(|elf| elf.position.y);

        let min_x = x_iterator.clone().min();
        let max_x = x_iterator.max();
        let min_y = y_iterator.clone().min();
        let max_y = y_iterator.max();

        let (
            Some(min_x),
            Some(max_x),
            Some(min_y),
            Some(max_y),
        ) = (min_x, max_x, min_y, max_y) else {
            return Err(String::from("Expected elves to not be empty when counting ground"));
        };

        let width = usize::try_from(max_x - min_x + 1)
            .map_err(|_| String::from("Failed to convert width to unsigned integer"))?;

        let height = usize::try_from(max_y - min_y + 1)
            .map_err(|_| String::from("Failed to convert width to unsigned integer"))?;

        let elves = self.0.len();

        Ok(width * height - elves)
    }

    fn all_except<'a>(&'a self, elf: &'a Elf) -> impl Iterator<Item = &'a Elf> + Clone {
        self.0.iter().filter(move |&other_elf| other_elf != elf)
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Elf {
    position: Position,
}

impl Elf {
    pub fn new(x: i64, y: i64) -> Self {
        Self {
            position: Position { x, y },
        }
    }

    fn propose_move<'a>(
        &self,
        round: u32,
        other_elves: impl Iterator<Item = &'a Elf> + Clone,
    ) -> Option<Position> {
        let adjacent_positions = self.position.adjacent();

        for check in adjacent_positions.position_checks_in_order(round) {
            if !other_elves
                .clone()
                .any(|other_elf| check.positions_to_check.contains(&&other_elf.position))
            {
                return check.position_to_propose.copied();
            }
        }

        None
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn adjacent(&self) -> AdjacentPositions {
        let &Position { x, y } = self;

        AdjacentPositions {
            north: Position { x, y: y + 1 },
            north_east: Position { x: x + 1, y: y + 1 },
            east: Position { x: x + 1, y },
            south_east: Position { x: x + 1, y: y - 1 },
            south: Position { x, y: y - 1 },
            south_west: Position { x: x - 1, y: y - 1 },
            west: Position { x: x - 1, y },
            north_west: Position { x: x - 1, y: y + 1 },
        }
    }
}

struct AdjacentPositions {
    north: Position,
    north_east: Position,
    east: Position,
    south_east: Position,
    south: Position,
    south_west: Position,
    west: Position,
    north_west: Position,
}

struct PositionCheck<'a> {
    positions_to_check: Vec<&'a Position>,
    position_to_propose: Option<&'a Position>,
}

impl AdjacentPositions {
    pub fn position_checks_in_order(&self, round: u32) -> [PositionCheck; 5] {
        let all = self.all_check();
        let north = self.north_check();
        let south = self.south_check();
        let west = self.west_check();
        let east = self.east_check();

        match (round - 1) % 4 {
            0 => [all, north, south, west, east],
            1 => [all, south, west, east, north],
            2 => [all, west, east, north, south],
            3 => [all, east, north, south, west],
            _ => unreachable!("Operation '% 4' should only give 0, 1, 2 or 3"),
        }
    }

    fn all_check(&self) -> PositionCheck {
        PositionCheck {
            positions_to_check: vec![
                &self.north,
                &self.north_east,
                &self.east,
                &self.south_east,
                &self.south,
                &self.south_west,
                &self.west,
                &self.north_west,
            ],
            position_to_propose: None,
        }
    }

    fn north_check(&self) -> PositionCheck {
        PositionCheck {
            positions_to_check: vec![&self.north, &self.north_east, &self.north_west],
            position_to_propose: Some(&self.north),
        }
    }

    fn south_check(&self) -> PositionCheck {
        PositionCheck {
            positions_to_check: vec![&self.south, &self.south_east, &self.south_west],
            position_to_propose: Some(&self.south),
        }
    }

    fn west_check(&self) -> PositionCheck {
        PositionCheck {
            positions_to_check: vec![&self.west, &self.north_west, &self.south_west],
            position_to_propose: Some(&self.west),
        }
    }

    fn east_check(&self) -> PositionCheck {
        PositionCheck {
            positions_to_check: vec![&self.east, &self.north_east, &self.south_east],
            position_to_propose: Some(&self.east),
        }
    }
}

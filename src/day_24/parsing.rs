use std::collections::HashSet;

use super::valley::{Blizzard, Direction, Expedition, Position, Valley};

impl TryFrom<&str> for Valley {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut blizzards = Vec::<Blizzard>::new();
        let mut wall_positions = HashSet::<Position>::new();
        let mut start_position: Option<Position> = None;
        let mut goal_position: Option<Position> = None;

        let height: usize = input.lines().count() + 1;
        let mut width: Option<usize> = None;

        for (y, line) in input
            .lines()
            .enumerate()
            .map(|(index, line)| (index + 1, line))
        {
            if y == 1 {
                width = Some(line.len());
            }

            for (x, character) in line.chars().enumerate() {
                match character {
                    '.' => {
                        if y == 1 {
                            start_position = Some(Position { x, y });
                        } else if y == height - 1 {
                            goal_position = Some(Position { x, y });
                        }
                    }
                    '#' => {
                        wall_positions.insert(Position { x, y });
                    }
                    _ => {
                        let direction = Direction::try_from(character)?;
                        let blizzard = Blizzard {
                            direction,
                            position: Position { x, y },
                        };
                        blizzards.push(blizzard);
                    }
                }
            }
        }

        let Some(width) = width else {
            return Err(String::from("Expected input to not be empty"));
        };

        let Some(start_position) = start_position else {
            return Err(String::from("Expected to find valid start position for expedition in input"));
        };

        wall_positions.insert(Position {
            x: start_position.x,
            y: start_position.y - 1,
        });

        let Some(goal_position) = goal_position else {
            return Err(String::from("Expected to find valid goal position in input"));
        };

        wall_positions.insert(Position {
            x: goal_position.x,
            y: goal_position.y + 1,
        });

        let expedition = Expedition {
            start_position,
            goal_position,
        };

        Ok(Valley {
            expedition,
            blizzards,
            wall_positions,
            width,
            height,
        })
    }
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(character: char) -> Result<Self, Self::Error> {
        use Direction::*;

        let direction = match character {
            '^' => North,
            '>' => East,
            'v' => South,
            '<' => West,
            _ => {
                return Err(format!(
                    "Could not parse character '{character}' to direction"
                ));
            }
        };

        Ok(direction)
    }
}

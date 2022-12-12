use super::height_map::{HeightMap, Position};

impl TryFrom<&str> for HeightMap {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut heights = Vec::<Vec<usize>>::new();
        let mut current_position: Option<Position> = None;
        let mut goal_position: Option<Position> = None;

        let mut rows = 0usize;
        let mut columns = 0usize;

        for (row, line) in input.lines().enumerate() {
            rows += 1;

            let mut height_row = Vec::<usize>::new();

            for (col, char) in line.chars().enumerate() {
                if row == 0 {
                    columns += 1;
                }

                let height_char = match char {
                    'S' => {
                        current_position = Some((row, col));
                        'a'
                    }
                    'E' => {
                        goal_position = Some((row, col));
                        'z'
                    }
                    char => char,
                };

                let height = height_from_char(height_char)?;
                height_row.push(height);
            }

            heights.push(height_row);
        }

        match (current_position, goal_position) {
            (Some(current_position), Some(goal_position)) => Ok(Self {
                heights,
                rows,
                columns,
                current_position,
                goal_position,
            }),
            _ => Err(String::from(
                "Did not find current position and/or goal position in input",
            )),
        }
    }
}

fn height_from_char(character: char) -> Result<usize, String> {
    ('a'..='z')
        .position(|other_character| character == other_character)
        .ok_or_else(|| format!("Invalid height character: {character}"))
}

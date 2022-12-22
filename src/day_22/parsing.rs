use super::map::{Instruction, Map, Tile, Tiles};

pub fn tiles_and_instructions_from_input(input: &str) -> Result<(Tiles, Vec<Instruction>), String> {
    let (map_input, instructions_input) = input
        .split_once("\n\n")
        .ok_or_else(|| String::from("Expected to find double newline in input"))?;

    let map = Map::tiles_from_input(map_input)?;
    let instructions = Instruction::instructions_from_line(instructions_input)?;

    Ok((map, instructions))
}

impl Map {
    fn tiles_from_input(input: &str) -> Result<Tiles, String> {
        let mut tiles = Tiles::new();

        for line in input.lines() {
            let mut row = Vec::<Option<Tile>>::new();

            for character in line.chars() {
                let tile = Tile::from_character(character)?;
                row.push(tile);
            }

            tiles.push(row);
        }

        Ok(tiles)
    }
}

impl Tile {
    fn from_character(character: char) -> Result<Option<Tile>, String> {
        use Tile::*;

        let tile = match character {
            '.' => Some(Open),
            '#' => Some(Wall),
            ' ' => None,
            _ => {
                return Err(format!("Unrecognized map input character '{character}'"));
            }
        };

        Ok(tile)
    }
}

impl Instruction {
    fn instructions_from_line(line: &str) -> Result<Vec<Instruction>, String> {
        use Instruction::*;

        let mut instructions = Vec::<Instruction>::new();
        let mut current_number = String::new();

        let get_forward_instruction = |number: &String| -> Result<Instruction, String> {
            let steps = number
                .parse::<i32>()
                .map_err(|_| format!("Failed to parse '{number}' to integer"))?;

            Ok(Forward { steps })
        };

        for character in line.chars() {
            if character.is_ascii_digit() {
                current_number.push(character);
            } else {
                let forward = get_forward_instruction(&current_number)?;
                instructions.push(forward);
                current_number.clear();

                let rotation = match character {
                    'R' => Right,
                    'L' => Left,
                    _ => {
                        return Err(format!(
                            "Unrecognized instruction character '{character}' in input"
                        ));
                    }
                };

                instructions.push(rotation);
            }
        }

        if !current_number.is_empty() {
            let forward = get_forward_instruction(&current_number)?;
            instructions.push(forward);
        }

        Ok(instructions)
    }
}

use super::{
    map::{Map, Tile},
    mover::Instruction,
};

pub(super) type TilesWithRowsAndColumns = (Vec<Vec<Option<Tile>>>, u32, u32);

pub(super) fn tiles_and_instructions_from_input(
    input: &str,
) -> Result<(TilesWithRowsAndColumns, Vec<Instruction>), String> {
    let (map_input, instructions_input) = input
        .split_once("\n\n")
        .ok_or_else(|| String::from("Expected to find double newline in input"))?;

    let tiles = Map::tiles_from_input(map_input)?;
    let instructions = Instruction::instructions_from_line(instructions_input)?;

    Ok((tiles, instructions))
}

impl Map {
    fn tiles_from_input(input: &str) -> Result<TilesWithRowsAndColumns, String> {
        let mut tiles = Vec::<Vec<Option<Tile>>>::new();

        let mut rows: u32 = 0;
        let mut columns: u32 = 0;

        for line in input.lines() {
            rows += 1;

            let mut row = Vec::<Option<Tile>>::new();
            let mut columns_in_this_row: u32 = 0;

            for character in line.chars() {
                columns_in_this_row += 1;

                let tile = Tile::from_character(character)?;
                row.push(tile);
            }

            tiles.push(row);

            if columns_in_this_row > columns {
                columns = columns_in_this_row;
            }
        }

        Ok((tiles, rows, columns))
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
                .parse::<i64>()
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

use std::collections::HashMap;

use regex::Regex;

use super::arrangement::{CrateArrangement, MoveOperation};

pub fn parse_input(input: &str) -> Result<(CrateArrangement, Vec<MoveOperation>), String> {
    let mut crate_arrangement_lines = Vec::<&str>::new();
    let mut move_operation_lines = Vec::<&str>::new();

    let mut is_crate_arrangement_line = true;
    for line in input.lines() {
        if line == "" {
            is_crate_arrangement_line = false;
            continue;
        }

        if is_crate_arrangement_line {
            crate_arrangement_lines.push(line);
        } else {
            move_operation_lines.push(line);
        }
    }

    let crate_arrangement = CrateArrangement::try_from(crate_arrangement_lines)?;
    let move_operations = move_operation_lines
        .iter()
        .map(|line| MoveOperation::try_from(*line))
        .collect::<Result<Vec<MoveOperation>, String>>()?;

    Ok((crate_arrangement, move_operations))
}

impl TryFrom<Vec<&str>> for CrateArrangement {
    type Error = String;

    fn try_from(input_lines: Vec<&str>) -> Result<Self, Self::Error> {
        let mut stacks = HashMap::<i32, Vec<char>>::new();

        let stack_number_line = input_lines
            .last()
            .ok_or("Empty input passed to crate arrangement".to_string())?;

        for i in (1..stack_number_line.len() - 1).step_by(4) {
            let stack_number: i32 = stack_number_line
                .chars()
                .nth(i)
                .ok_or("Stack number line too short".to_string())?
                .to_digit(10)
                .ok_or(format!(
                    "Failed to parse stack number from input line: {stack_number_line}"
                ))?
                .try_into()
                .map_err(|_| "Could not convert stack number to 32-bit integer".to_string())?;

            let stack: Vec<char> = input_lines
                .iter()
                .rev()
                .skip(1)
                .flat_map(|line| match line.chars().nth(i) {
                    Some(char) => {
                        if char == ' ' {
                            None
                        } else {
                            Some(char)
                        }
                    }
                    None => None,
                })
                .collect();

            stacks.insert(stack_number, stack);
        }

        Ok(Self { stacks })
    }
}

impl TryFrom<&str> for MoveOperation {
    type Error = String;

    fn try_from(input_line: &str) -> Result<Self, Self::Error> {
        let re =
            Regex::new(r"move | from | to ").map_err(|_| "Failed to compile regex".to_string())?;

        let numbers = re
            .split(input_line)
            .skip(1)
            .map(|number_string| {
                number_string
                    .parse::<i32>()
                    .map_err(|_| "Failed to parse number in move operation line".to_string())
            })
            .collect::<Result<Vec<i32>, String>>()?;

        if numbers.len() < 3 {
            return Err("Did not find enough numbers in move operation line".to_string());
        }

        Ok(Self {
            crates_to_move: numbers[0],
            from_stack: numbers[1],
            to_stack: numbers[2],
        })
    }
}

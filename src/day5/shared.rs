use std::collections::HashMap;

use regex::Regex;

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

    let crate_arrangement = (&mut crate_arrangement_lines).try_into()?;
    let move_operations = move_operation_lines
        .into_iter()
        .map(|line| line.try_into())
        .collect::<Result<Vec<MoveOperation>, String>>()?;

    Ok((crate_arrangement, move_operations))
}

pub struct CrateArrangement {
    pub stacks: HashMap<i32, Vec<char>>,
}

impl CrateArrangement {
    pub fn get_top_crates(&self) -> HashMap<i32, Option<char>> {
        let mut top_crates = HashMap::<i32, Option<char>>::new();

        for (stack_number, crates) in self.stacks.clone() {
            top_crates.insert(stack_number, crates.last().copied());
        }

        top_crates
    }
}

pub fn get_top_crates_string(top_crates: HashMap<i32, Option<char>>) -> String {
    let mut top_crates_vec = top_crates.into_iter().collect::<Vec<(i32, Option<char>)>>();

    top_crates_vec.sort_by_key(|(stack_number, _)| *stack_number);

    top_crates_vec
        .into_iter()
        .map(|(_, top_crate)| match top_crate {
            Some(top_crate) => top_crate,
            None => ' ',
        })
        .collect::<String>()
}

impl TryFrom<&mut Vec<&str>> for CrateArrangement {
    type Error = String;

    fn try_from(input_lines: &mut Vec<&str>) -> Result<Self, Self::Error> {
        let mut stacks = HashMap::<i32, Vec<char>>::new();

        let stack_number_line = input_lines
            .pop()
            .ok_or("Empty input passed to crate arrangement")?;

        for i in (1..stack_number_line.len() - 1).step_by(4) {
            let stack_number: i32 = stack_number_line
                .chars()
                .nth(i)
                .ok_or("Stack number line too short")?
                .to_digit(10)
                .ok_or(format!(
                    "Failed to parse stack number from input line: {stack_number_line}"
                ))?
                .try_into()
                .map_err(|_| "Could not convert stack number to 32-bit integer")?;

            let stack: Vec<char> = input_lines
                .into_iter()
                .rev()
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

pub struct MoveOperation {
    pub crates_to_move: i32,
    pub from_stack: i32,
    pub to_stack: i32,
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

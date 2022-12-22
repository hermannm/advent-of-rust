use std::collections::HashMap;

use super::operation_tree::{Operand, Operation, Operator};

const ROOT_MONKEY_NAME: &str = "root";
const VARIABLE_NAME: &str = "humn";

impl Operation {
    pub fn root_from_input(input: &str) -> Result<Self, String> {
        let mut input_map = HashMap::<&str, &str>::new();

        for line in input.lines() {
            let Some((monkey_name, operation_string)) = line.split_once(": ") else {
                return Err(format!("Expected to find ':' in input line '{line}'"));
            };

            input_map.insert(monkey_name, operation_string);
        }

        let Some(root_operation) = input_map.get(ROOT_MONKEY_NAME) else {
            return Err(format!("Failed to find monkey with name '{ROOT_MONKEY_NAME}' in input map"));
        };

        let (operand_1, _, operand_2) = Operation::split_operation_string(root_operation)?;

        Operation::new(operand_1, '=', operand_2, &input_map)
    }

    fn new(
        operand_1: &str,
        operator_character: char,
        operand_2: &str,
        input_map: &HashMap<&str, &str>,
    ) -> Result<Self, String> {
        let operand_1 = Operand::from_input_map(operand_1, input_map)?;
        let operator = Operator::try_from(operator_character)?;
        let operand_2 = Operand::from_input_map(operand_2, input_map)?;

        Ok(Operation {
            operator,
            operands: [operand_1, operand_2],
        })
    }

    fn split_operation_string(operation_string: &str) -> Result<(&str, char, &str), String> {
        let operation_split = operation_string.split(' ').collect::<Vec<&str>>();
        if operation_split.len() != 3 {
            return Err(format!(
                "Expected operation string '{operation_string}' to be splittable into 3"
            ));
        }

        let Some(operator_character) = operation_split[1].chars().next() else {
            return Err(format!(
                "Failed to get first character from string '{}'", operation_split[1]
            ));
        };

        Ok((operation_split[0], operator_character, operation_split[2]))
    }
}

impl Operand {
    fn from_input_map(
        operand_string: &str,
        input_map: &HashMap<&str, &str>,
    ) -> Result<Self, String> {
        use Operand::*;

        if operand_string == VARIABLE_NAME {
            println!("{operand_string}");
            return Ok(Variable);
        }

        let Some(operand_value) = input_map.get(operand_string) else {
            return Err(format!("Failed to find operand '{operand_string}' in input map"));
        };

        let operand = match operand_value.parse::<i64>() {
            Ok(number) => Number(number),
            Err(_) => {
                let (operand_1, operator, operand_2) =
                    Operation::split_operation_string(operand_value)?;

                let operation = Operation::new(operand_1, operator, operand_2, input_map)?;

                Nested(Box::new(operation))
            }
        };

        Ok(operand)
    }
}

impl TryFrom<char> for Operator {
    type Error = String;

    fn try_from(operator_character: char) -> Result<Self, Self::Error> {
        use Operator::*;

        let operator = match operator_character {
            '+' => Add,
            '-' => Subtract,
            '*' => Multiply,
            '/' => Divide,
            '=' => Equals,
            _ => {
                return Err(format!(
                    "Unexpected operator character '{operator_character}'"
                ))
            }
        };

        Ok(operator)
    }
}

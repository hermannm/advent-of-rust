use super::monkeys::{MathOperation, Monkeys, Operand, Operator, ReadyMonkey, WaitingMonkey};

impl TryFrom<&str> for Monkeys {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut ready_monkeys = Vec::<ReadyMonkey>::new();
        let mut waiting_monkeys = Vec::<WaitingMonkey>::new();

        for line in input.lines() {
            use MonkeyParseResult::*;

            match MonkeyParseResult::from(line) {
                Ready(monkey) => {
                    ready_monkeys.push(monkey);
                }
                Waiting(monkey) => {
                    waiting_monkeys.push(monkey);
                }
                ParseError(error) => {
                    return Err(error);
                }
            }
        }

        Ok(Monkeys {
            ready_monkeys,
            waiting_monkeys,
        })
    }
}

enum MonkeyParseResult {
    Ready(ReadyMonkey),
    Waiting(WaitingMonkey),
    ParseError(String),
}

impl MonkeyParseResult {
    fn from(input_line: &str) -> MonkeyParseResult {
        use MonkeyParseResult::*;

        let Some((name, rest_of_line)) = input_line.split_once(": ") else {
            return ParseError(format!("Expected to find ':' in input line '{input_line}'"));
        };

        let name = String::from(name);

        let line_split = rest_of_line.split(' ').collect::<Vec<&str>>();

        match line_split.len() {
            1 => {
                let Ok(shout) = line_split[0].parse::<i64>() else {
                    return ParseError(format!("Failed to parse string '{}' to integer", line_split[0]));
                };

                Ready(ReadyMonkey { name, shout })
            }
            3 => {
                let Some(operator_character) = line_split[1]
                    .chars()
                    .last() else {
                        return ParseError(format!(
                            "Failed to parse middle of math operation input split '{line_split:?}' to character"
                        ));
                    };

                let shout = match MathOperation::try_from((
                    line_split[0],
                    operator_character,
                    line_split[2],
                )) {
                    Ok(shout) => shout,
                    Err(error) => return ParseError(error),
                };

                Waiting(WaitingMonkey {
                    name,
                    shout,
                    shout_result: None,
                })
            }
            _ => ParseError(format!(
                "Expected input string '{rest_of_line}' to be splittable into either 1 or 3"
            )),
        }
    }
}

impl TryFrom<(&str, char, &str)> for MathOperation {
    type Error = String;

    fn try_from(input_split: (&str, char, &str)) -> Result<Self, Self::Error> {
        let operand_1 = Operand::from(input_split.0);
        let operand_2 = Operand::from(input_split.2);
        let operator = Operator::try_from(input_split.1)?;

        Ok(MathOperation {
            operands: [operand_1, operand_2],
            operator,
        })
    }
}

impl From<&str> for Operand {
    fn from(monkey_name: &str) -> Self {
        Operand {
            monkey_name: String::from(monkey_name),
            value: None,
        }
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
            _ => {
                return Err(format!(
                    "Unexpected operator character '{operator_character}'"
                ))
            }
        };

        Ok(operator)
    }
}

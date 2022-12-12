use std::str::FromStr;

use super::monkey_in_the_middle::{Item, Monkey, WorryLevelOperation, WorryLevelTest};

impl Monkey {
    pub fn monkeys_from_input_lines(input: &str) -> Result<Vec<Monkey>, String> {
        input
            .split("\n\n")
            .map(|line_group| {
                let lines_vec = line_group.lines().collect::<Vec<&str>>();

                let monkey_lines = <[&str; 6]>::try_from(lines_vec).map_err(|_| {
                    "Failed to convert line group to array of 6 lines for monkey input"
                })?;

                Monkey::try_from(monkey_lines)
            })
            .collect::<Result<Vec<Monkey>, String>>()
    }
}

impl TryFrom<[&str; 6]> for Monkey {
    type Error = String;

    fn try_from(monkey_lines: [&str; 6]) -> Result<Self, Self::Error> {
        let [monkey_id_line, starting_items_line, operation_line, test_line_1, test_line_2, test_line_3] =
            monkey_lines;

        let id = Monkey::get_id_from_line(monkey_id_line)?;
        let items = Item::starting_items_from_line(starting_items_line)?;
        let operation = WorryLevelOperation::try_from(operation_line)?;
        let test = WorryLevelTest::try_from([test_line_1, test_line_2, test_line_3])?;

        Ok(Self {
            id,
            items,
            operation,
            test,
            item_inspection_count: 0,
        })
    }
}

impl Monkey {
    fn get_id_from_line(line: &str) -> Result<i32, String> {
        let (_, rest_of_line) = line
            .split_once("Monkey ")
            .ok_or_else(|| String::from("Monkey ID input line did not contain 'Monkey '"))?;

        let (id_string, _) = rest_of_line
            .split_once(':')
            .ok_or_else(|| String::from("Monkey ID input line did not contain ':'"))?;

        id_string
            .parse::<i32>()
            .map_err(|_| String::from("Failed to parse monkey ID string to integer"))
    }
}

impl Item {
    fn starting_items_from_line(line: &str) -> Result<Vec<Item>, String> {
        let (_, items_string) = line.split_once("Starting items: ").ok_or_else(|| {
            String::from("Starting items input line did not contain 'Starting items: '")
        })?;

        items_string
            .split(", ")
            .map(Item::try_from)
            .collect::<Result<Vec<Item>, String>>()
    }
}

impl TryFrom<&str> for Item {
    type Error = String;

    fn try_from(worry_level_string: &str) -> Result<Self, Self::Error> {
        let worry_level = worry_level_string.parse::<u64>().map_err(|_| {
            format!("Failed to parse worry level string '{worry_level_string}' to integer")
        })?;

        Ok(Self { worry_level })
    }
}

impl TryFrom<&str> for WorryLevelOperation {
    type Error = String;

    fn try_from(operation_string: &str) -> Result<Self, Self::Error> {
        if let Some((_, addend_string)) = operation_string.split_once(" + ") {
            let addend = addend_string.parse::<u64>().map_err(|_| {
                String::from("Failed to parse addend in worry level operation to integer")
            })?;

            Ok(WorryLevelOperation::Add(addend))
        } else if let Some((_, factor_string)) = operation_string.split_once(" * ") {
            if factor_string == "old" {
                Ok(WorryLevelOperation::MultiplyWithSelf)
            } else {
                let factor = factor_string.parse::<u64>().map_err(|_| {
                    String::from(
                        "Failed to parse multiplication factor in worry level operation to integer",
                    )
                })?;

                Ok(WorryLevelOperation::MultiplyWith(factor))
            }
        } else {
            Err(format!("Failed to recognize any operation in worry level operation string: {operation_string}"))
        }
    }
}

impl TryFrom<[&str; 3]> for WorryLevelTest {
    type Error = String;

    fn try_from(input_lines: [&str; 3]) -> Result<Self, Self::Error> {
        let divisible_by = {
            let line = input_lines[0];

            if !line.contains("divisible by") {
                return Err(String::from(
                    "Expected first input line for worry level test to contain 'divisible by'",
                ));
            }

            get_last_integer_in_line::<u64>(line)?
        };

        let target_monkey_if_true = WorryLevelTest::get_target_monkey_from_line(input_lines[1])?;
        let target_monkey_if_false = WorryLevelTest::get_target_monkey_from_line(input_lines[2])?;

        Ok(Self {
            divisible_by,
            target_monkey_if_true,
            target_monkey_if_false,
        })
    }
}

impl WorryLevelTest {
    fn get_target_monkey_from_line(line: &str) -> Result<i32, String> {
        if !line.contains("throw to monkey") {
            return Err(String::from(
                "Expected input line for target monkey to contain 'throw to monkey'",
            ));
        }

        get_last_integer_in_line::<i32>(line)
    }
}

fn get_last_integer_in_line<T: FromStr>(line: &str) -> Result<T, String> {
    let last_integer_string = line
        .split(' ')
        .last()
        .ok_or_else(|| String::from("Cannot get integer from empty line"))?;

    last_integer_string
        .parse::<T>()
        .map_err(|_| String::from("Failed to parse last part of line to integer"))
}

use super::operations::Operation;

const VARIABLE_NAME: &str = "humn";

pub fn solve_puzzle(input: &str) -> Result<i64, String> {
    let root_operation = Operation::root_from_input(input, Some(VARIABLE_NAME), true)?;

    let variable_value = root_operation.calculate_variable(None)?;

    Ok(variable_value)
}

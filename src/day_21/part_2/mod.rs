use self::operation_tree::Operation;

mod operation_tree;
mod parsing;

pub fn solve_puzzle(input: &str) -> Result<i64, String> {
    let root_operation = Operation::root_from_input(input)?;

    let variable_value = root_operation.calculate_variable(None)?;

    Ok(variable_value)
}

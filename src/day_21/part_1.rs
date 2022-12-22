use super::operations::{Operation, ValueResult};

pub fn solve_puzzle(input: &str) -> Result<i64, String> {
    let root_operation = Operation::root_from_input(input, None, false)?;

    use ValueResult::*;

    match root_operation.get_value() {
        Value(root_value) => Ok(root_value),
        ContainsVariable => Err(String::from("Operation tree contained unexpected variable")),
    }
}

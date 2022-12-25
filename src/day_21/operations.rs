pub(super) struct Operation {
    pub operator: Operator,
    pub operands: [Operand; 2],
}

pub(super) enum ValueResult {
    Value(i64),
    ContainsVariable,
}

impl Operation {
    pub fn get_value(&self) -> ValueResult {
        use ValueResult::*;

        let [operand_1, operand_2] = &self.operands;

        match (operand_1.get_value(), operand_2.get_value()) {
            (Value(value_1), Value(value_2)) => Value(self.operator.apply(value_1, value_2)),
            _ => ContainsVariable,
        }
    }

    pub fn calculate_variable(&self, operand_value_to_reach: Option<i64>) -> Result<i64, String> {
        use ValueResult::*;

        let [operand_1, operand_2] = &self.operands;

        let variable_value = match (operand_1.get_value(), operand_2.get_value()) {
            (Value(operand1_value), ContainsVariable) => {
                let operand_value_to_reach = self.operator.get_other_operand_value(
                    Some(operand1_value),
                    None,
                    operand_value_to_reach,
                )?;

                operand_2.calculate_variable(operand_value_to_reach)?
            }
            (ContainsVariable, Value(operand2_value)) => {
                let operand_value_to_reach = self.operator.get_other_operand_value(
                    None,
                    Some(operand2_value),
                    operand_value_to_reach,
                )?;

                operand_1.calculate_variable(operand_value_to_reach)?
            }
            (ContainsVariable, ContainsVariable) => {
                return Err(String::from(
                    "Found 2 variables in operation tree, when there should only be 1",
                ));
            }
            (Value(_), Value(_)) => {
                return Err(String::from(
                    "Expected one side of the operation tree to contain a variable, but found none",
                ));
            }
        };

        Ok(variable_value)
    }
}

pub(super) enum Operand {
    Variable,
    Number(i64),
    Nested(Box<Operation>),
}

impl Operand {
    fn get_value(&self) -> ValueResult {
        use Operand::*;
        use ValueResult::*;

        match self {
            Variable => ContainsVariable,
            Number(number) => Value(*number),
            Nested(operation) => operation.get_value(),
        }
    }

    fn calculate_variable(&self, operand_value_to_reach: i64) -> Result<i64, String> {
        use Operand::*;

        let value = match self {
            Variable => operand_value_to_reach,
            Nested(operation) => operation.calculate_variable(Some(operand_value_to_reach))?,
            Number(_) => {
                return Err(String::from(
                    "Tried to calculate variable from concrete value operand",
                ));
            }
        };

        Ok(value)
    }
}

pub(super) enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equals,
}

impl Operator {
    fn apply(&self, value_1: i64, value_2: i64) -> i64 {
        use Operator::*;

        match self {
            Add => value_1 + value_2,
            Subtract => value_1 - value_2,
            Multiply => value_1 * value_2,
            Divide => value_1 / value_2,
            Equals => value_1,
        }
    }

    fn get_other_operand_value(
        &self,
        first_operand_value: Option<i64>,
        second_operand_value: Option<i64>,
        result_to_reach: Option<i64>,
    ) -> Result<i64, String> {
        use Operator::*;

        let second_operand_value = match (self, first_operand_value, second_operand_value, result_to_reach) {
            (Equals, Some(value), None, _) | (Equals, None, Some(value), _) => {
                value
            }
            (Add, Some(value), None, Some(result)) | (Add, None, Some(value), Some(result)) => {
                result - value
            }
            (Subtract, Some(value_1), None, Some(result)) => {
                value_1 - result
            }
            (Subtract, None, Some(value_2), Some(result)) => {
                value_2 + result
            }
            (Multiply, Some(value), None, Some(result)) | (Multiply, None, Some(value), Some(result)) => {
                result / value
            }
            (Divide, Some(value_1), None, Some(result)) => {
                value_1 / result
            }
            (Divide, None, Some(value_2), Some(result)) => {
                value_2 * result
            }
            _ => {
                return Err(format!(
                    "Unexpected input when trying to get other operand value: first operand '{first_operand_value:?}', second operand '{second_operand_value:?}', result to reach '{result_to_reach:?}'"
                ))
            }
        };

        Ok(second_operand_value)
    }
}

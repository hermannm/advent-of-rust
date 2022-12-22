pub struct Monkeys {
    pub ready_monkeys: Vec<ReadyMonkey>,
    pub waiting_monkeys: Vec<WaitingMonkey>,
}

pub struct ReadyMonkey {
    pub name: String,
    pub shout: i64,
}

pub struct WaitingMonkey {
    pub name: String,
    pub shout: MathOperation,
    pub shout_result: Option<i64>,
}

impl Monkeys {
    pub fn get_number_yelled_by(&mut self, monkey_name: &str) -> Option<i64> {
        for monkey in &self.ready_monkeys {
            Monkeys::shout_value(&monkey.name, monkey.shout, &mut self.waiting_monkeys);
        }

        for monkey in &self.waiting_monkeys {
            if monkey.name.as_str() == monkey_name {
                return monkey.shout_result;
            }
        }

        None
    }

    fn shout_value(monkey_name: &String, value: i64, waiting_monkeys: &mut Vec<WaitingMonkey>) {
        let mut monkeys_to_shout = Vec::<(String, i64)>::new();

        for waiting_monkey in waiting_monkeys.iter_mut() {
            if waiting_monkey.shout_result.is_some() {
                continue;
            }

            for operand in waiting_monkey.shout.operands.iter_mut() {
                if &operand.monkey_name == monkey_name {
                    operand.value = Some(value);
                }
            }

            if let Some(value) = waiting_monkey.shout.try_calculate() {
                waiting_monkey.shout_result = Some(value);
                monkeys_to_shout.push((waiting_monkey.name.clone(), value));
            }
        }

        for (monkey, value) in monkeys_to_shout {
            Monkeys::shout_value(&monkey, value, waiting_monkeys);
        }
    }
}

pub struct MathOperation {
    pub operands: [Operand; 2],
    pub operator: Operator,
}

pub struct Operand {
    pub monkey_name: String,
    pub value: Option<i64>,
}

pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl MathOperation {
    fn try_calculate(&self) -> Option<i64> {
        let operand_values = [self.operands[0].value, self.operands[1].value];

        match operand_values {
            [Some(value_1), Some(value_2)] => {
                use Operator::*;

                let result = match self.operator {
                    Add => value_1 + value_2,
                    Subtract => value_1 - value_2,
                    Multiply => value_1 * value_2,
                    Divide => value_1 / value_2,
                };

                Some(result)
            }
            _ => None,
        }
    }
}

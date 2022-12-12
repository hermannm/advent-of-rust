pub enum Instruction {
    Addx(i64),
    Noop,
}

impl Instruction {
    pub fn cycles_to_complete(&self) -> u32 {
        match self {
            Instruction::Addx(_) => 2,
            Instruction::Noop => 1,
        }
    }
}

impl TryFrom<&str> for Instruction {
    type Error = String;

    fn try_from(input_line: &str) -> Result<Self, Self::Error> {
        match input_line.split_once(' ') {
            Some((command_string, value_string)) => {
                if command_string != "addx" {
                    return Err(format!(
                        "Expected input line with space to be an addx instruction, but got: {input_line}"
                    ));
                }

                let value = value_string
                    .parse::<i64>()
                    .map_err(|_| "Failed to parse addx value to integer".to_string())?;

                Ok(Instruction::Addx(value))
            }
            None => {
                if input_line != "noop" {
                    return Err(format!(
                        "Expected input line without space to be a noop instruction, but got: {input_line}"
                    ));
                }

                Ok(Instruction::Noop)
            }
        }
    }
}

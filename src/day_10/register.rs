use super::instruction::Instruction;

pub(super) struct Register {
    pub value: i64,
    cycles_completed: u32,
}

impl Register {
    pub fn new(start_value: i64) -> Self {
        Self {
            value: start_value,
            cycles_completed: 0,
        }
    }

    pub fn apply_instructions(&mut self, instructions: &mut Vec<Instruction>, until_cycle: u32) {
        loop {
            if instructions.is_empty() {
                return;
            }

            let cycles_to_complete = instructions[0].cycles_to_complete();

            if self.cycles_completed + cycles_to_complete >= until_cycle {
                return;
            }

            let instruction = instructions.remove(0);

            match instruction {
                Instruction::Addx(value) => {
                    self.value += value;
                }
                Instruction::Noop => {}
            }

            self.cycles_completed += cycles_to_complete;
        }
    }

    pub fn get_signal_strength(&self, current_cycle: u32) -> i64 {
        self.value * i64::from(current_cycle)
    }
}

use super::{instruction::Instruction, register::Register};

pub fn solve_puzzle(input: &str) -> Result<i64, String> {
    let mut instructions = input
        .lines()
        .map(Instruction::try_from)
        .collect::<Result<Vec<Instruction>, String>>()?;

    let mut register = Register::new(1);

    let mut signal_strengths = Vec::<i64>::new();

    for cycle in [20, 60, 100, 140, 180, 220] {
        register.apply_instructions(&mut instructions, cycle);
        signal_strengths.push(register.get_signal_strength(cycle));
    }

    Ok(signal_strengths.iter().sum())
}

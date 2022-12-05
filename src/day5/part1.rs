use super::shared::{get_top_crates_string, parse_input, CrateArrangement, MoveOperation};

#[allow(dead_code)]
pub fn solve_puzzle() -> Result<String, String> {
    let input = include_str!("input.txt");

    let (mut crate_arrangement, move_operations) = parse_input(input)?;

    rearrange_crates(&mut crate_arrangement, &move_operations)?;

    let top_crates_string = get_top_crates_string(crate_arrangement.get_top_crates());

    Ok(top_crates_string)
}

fn rearrange_crates(
    crate_arrangement: &mut CrateArrangement,
    move_operations: &Vec<MoveOperation>,
) -> Result<(), String> {
    for move_operation in move_operations {
        for _ in 0..move_operation.crates_to_move {
            let moved_crate = crate_arrangement
                .stacks
                .get_mut(&move_operation.from_stack)
                .ok_or(format!(
                    "No crate stack found with number {}",
                    move_operation.from_stack
                ))?
                .pop()
                .ok_or("Attempted to move crate from empty stack")?;

            crate_arrangement
                .stacks
                .get_mut(&move_operation.to_stack)
                .ok_or(format!(
                    "No crate stack found with number {}",
                    move_operation.to_stack
                ))?
                .push(moved_crate);
        }
    }

    Ok(())
}

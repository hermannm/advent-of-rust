use super::shared::{get_top_crates_string, parse_input, CrateArrangement, MoveOperation};

pub fn solve_puzzle(input: &str) -> Result<String, String> {
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
        let mut moved_crates = {
            let stack = crate_arrangement
                .stacks
                .get_mut(&move_operation.from_stack)
                .ok_or(format!(
                    "No crate stack found with number {}",
                    move_operation.from_stack
                ))?;

            let crates_to_move: usize = move_operation.crates_to_move.try_into().map_err(|_| {
                "Could not convert crates to move in move operation to usize".to_string()
            })?;

            stack.split_off(stack.len() - crates_to_move)
        };

        crate_arrangement
            .stacks
            .get_mut(&move_operation.to_stack)
            .ok_or(format!(
                "No crate stack found with number {}",
                move_operation.to_stack
            ))?
            .append(&mut moved_crates);
    }

    Ok(())
}

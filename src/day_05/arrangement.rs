use std::collections::HashMap;

pub struct MoveOperation {
    pub crates_to_move: i32,
    pub from_stack: i32,
    pub to_stack: i32,
}

pub struct CrateArrangement {
    pub stacks: HashMap<i32, Vec<char>>,
}

impl CrateArrangement {
    pub fn rearrange(
        &mut self,
        move_operations: &Vec<MoveOperation>,
        reverse_moves: bool,
    ) -> Result<(), String> {
        for move_operation in move_operations {
            let mut moved_crates = {
                let stack = self
                    .stacks
                    .get_mut(&move_operation.from_stack)
                    .ok_or_else(|| {
                        format!(
                            "No crate stack found with number {}",
                            move_operation.from_stack
                        )
                    })?;

                let crates_to_move =
                    usize::try_from(move_operation.crates_to_move).map_err(|_| {
                        String::from("Could not convert crates to move in move operation to usize")
                    })?;

                let mut moved_crates = stack.split_off(stack.len() - crates_to_move);
                if reverse_moves {
                    moved_crates.reverse();
                }

                moved_crates
            };

            self.stacks
                .get_mut(&move_operation.to_stack)
                .ok_or_else(|| {
                    format!(
                        "No crate stack found with number {}",
                        move_operation.to_stack
                    )
                })?
                .append(&mut moved_crates);
        }

        Ok(())
    }

    pub fn get_top_crates_string(&self) -> String {
        let mut top_crates_vec = self.stacks.iter().collect::<Vec<(&i32, &Vec<char>)>>();

        top_crates_vec.sort_by_key(|(stack_number, _)| *stack_number);

        top_crates_vec
            .iter()
            .map(|(_, stack)| match stack.last() {
                Some(top_crate) => *top_crate,
                None => ' ',
            })
            .collect::<String>()
    }
}

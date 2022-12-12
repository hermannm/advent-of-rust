use super::{movement::Movement, rope::Rope};

pub fn solve_puzzle(input: &str) -> Result<usize, String> {
    let movements = input
        .lines()
        .map(Movement::try_from)
        .collect::<Result<Vec<Movement>, String>>()?;

    let mut rope = Rope::new(2)?;

    for movement in movements {
        rope.move_rope(&movement);
    }

    Ok(rope.tail().visited_positions.len())
}

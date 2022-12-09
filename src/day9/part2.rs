use super::{movement::Movement, rope::Rope};

pub fn solve_puzzle(input: &str) -> Result<usize, String> {
    let movements = input
        .lines()
        .map(|line| Movement::try_from(line))
        .collect::<Result<Vec<Movement>, String>>()?;

    let mut rope = Rope::new(10)?;

    for movement in movements {
        rope.move_rope(&movement);
    }

    Ok(rope.tail().visited_positions.len())
}

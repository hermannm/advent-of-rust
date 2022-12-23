use super::{map::Map, mover::Mover, parsing::tiles_and_instructions_from_input};

pub fn solve_puzzle(input: &str, cube_side_length: u32) -> Result<i64, String> {
    let ((tiles, rows, columns), instructions) = tiles_and_instructions_from_input(input)?;

    let mut map = Map::new(tiles, rows, columns);
    map.set_to_cube(cube_side_length);

    let mut mover = Mover::new(&map)?;

    for instruction in &instructions {
        mover.follow_instruction(instruction, &map)?;
    }

    Ok(mover.get_password())
}

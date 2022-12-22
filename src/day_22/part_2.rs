use super::{
    map::{Map, Mover},
    parsing::tiles_and_instructions_from_input,
};

pub fn solve_puzzle(input: &str, cube_side_length: u16) -> Result<i32, String> {
    let (tiles, instructions) = tiles_and_instructions_from_input(input)?;

    let map = Map::new(tiles, Some(cube_side_length));

    let mut mover = Mover::new(&map)?;

    for instruction in &instructions {
        mover.follow_instruction(instruction, &map);
    }

    Ok(mover.get_password())
}

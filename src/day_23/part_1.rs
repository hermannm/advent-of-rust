use super::elves::Elves;

pub fn solve_puzzle(input: &str) -> Result<usize, String> {
    let mut elves = Elves::from(input);

    elves.move_to_open_ground(10);
    let empty_ground = elves.count_empty_ground()?;

    Ok(empty_ground)
}

// - More empty ground in every direction
// - Rounds, in which each elf alternates between:
//     - considering where to move
//         - considers 8 adjacent positions
//             - if no adjacent elf:
//                 do nothing
//             - else:
//                 - if no elf in N, NE, NW:
//                     propose move N one step
//                 - else if no elf in S, SE, SW:
//                     propose move S one step
//                 - else if no elf in W, NW, SW:
//                     propose move W one step
//                 - else if no elf in E, NE, SE:
//                     propose move E one step
//                 (order of checked directions
//                 ROTATES BY ONE every round)
//     - actually moving
//         - for every proposed move:
//             - if only elf to propose move here:
//                 make move
//             - else:
//                 do nothing
//

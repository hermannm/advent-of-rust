use super::elves::Elves;

pub fn solve_puzzle(input: &str) -> u32 {
    let mut elves = Elves::from(input);

    elves.find_round_where_no_elf_moved()
}

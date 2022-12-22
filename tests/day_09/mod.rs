use advent_of_rust::day_09::{part_1, part_2};

const PART1_INPUT: &str = include_str!("part1_input.txt");

#[test]
fn part_1() {
    let part1_result = part_1::solve_puzzle(PART1_INPUT.trim_end());
    assert_eq!(part1_result, Ok(13))
}

const PART2_INPUT: &str = include_str!("part2_input.txt");

#[test]
fn part_2() {
    let part2_result = part_2::solve_puzzle(PART2_INPUT.trim_end());
    assert_eq!(part2_result, Ok(36))
}

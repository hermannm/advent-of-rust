use advent_of_rust::day_19::{part_1, part_2};

const INPUT: &str = include_str!("input.txt");

#[test]
fn part_1() {
    let part1_result = part_1::solve_puzzle(INPUT.trim());
    assert_eq!(part1_result, Ok(33))
}

#[test]
fn part_2() {
    let part2_result = part_2::solve_puzzle(INPUT.trim());
    assert_eq!(part2_result, Ok(3_472))
}
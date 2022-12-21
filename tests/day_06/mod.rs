use advent_of_rust::day_06::{part_1, part_2};

const INPUT: &str = include_str!("input.txt");

#[test]
fn part_1() {
    let input = include_str!("input.txt").trim();
    let part1_result = part_1::solve_puzzle(input);
    assert_eq!(part1_result, Ok(7))
}

#[test]
fn part_2() {
    let part2_result = part_2::solve_puzzle(INPUT.trim());
    assert_eq!(part2_result, Ok(19))
}

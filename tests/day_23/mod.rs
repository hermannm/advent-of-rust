use advent_of_rust::day_23::{part_1, part_2};

const INPUT: &str = include_str!("input.txt");

#[test]
fn part_1() {
    let part1_result = part_1::solve_puzzle(INPUT.trim_end());
    assert_eq!(part1_result, Ok(110))
}

#[test]
fn part_2() {
    let part2_result = part_2::solve_puzzle(INPUT.trim_end());
    assert_eq!(part2_result, 20)
}

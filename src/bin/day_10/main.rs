use advent_of_rust::day_10::{part_1, part_2};

fn main() -> Result<(), String> {
    let input = include_str!("input.txt").trim_end();

    println!("Solutions to Advent of Code 2022, day 10");

    let part1_solution = part_1::solve_puzzle(input)?;
    println!("Part 1: {part1_solution}");

    let part2_solution = part_2::solve_puzzle(input)?;
    println!("Part 2: {part2_solution}");

    Ok(())
}

use advent_of_rust::day3::{part1, part2};

fn main() -> Result<(), String> {
    let input = include_str!("input.txt");

    let part1_solution = part1::solve_puzzle(input)?;
    let part2_solution = part2::solve_puzzle(input)?;

    println!("Solutions to Advent of Code 2022, day 3");
    println!("Part 1: {part1_solution}");
    println!("Part 2: {part2_solution}");

    Ok(())
}

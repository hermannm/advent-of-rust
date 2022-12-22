use advent_of_rust::day_15::{part_1, part_2};

fn main() -> Result<(), String> {
    let input = include_str!("input.txt").trim_end();

    println!("Solutions to Advent of Code 2022, day 15");

    let part1_solution = part_1::solve_puzzle(input, 2_000_000)?;
    println!("Part 1: {part1_solution}");

    let part2_solution = part_2::solve_puzzle(input, (0, 4_000_000))?;
    println!("Part 2: {part2_solution}");

    Ok(())
}

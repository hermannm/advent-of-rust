use advent_of_rust::day_25;

fn main() -> Result<(), String> {
    let input = include_str!("input.txt").trim_end();

    let solution = day_25::solve_puzzle(input)?;
    println!("Solution to Advent of Code 2022, day 25: {solution}");

    Ok(())
}

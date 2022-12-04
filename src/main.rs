mod day1;
mod day2;
mod day3;
mod day4;

fn main() -> Result<(), String> {
    let solution = day4::part2::solve_puzzle()?;
    println!("Solution: {solution}");

    Ok(())
}

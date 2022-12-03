mod day1;
mod day2;
mod day3;

fn main() -> Result<(), String> {
    let solution = day3::part2::solve_puzzle()?;
    println!("Solution: {solution}");

    Ok(())
}

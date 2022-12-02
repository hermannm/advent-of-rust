mod day1;
mod day2;

fn main() -> Result<(), String> {
    let solution = day2::part2::solve_puzzle()?;
    println!("Solution: {solution}");

    Ok(())
}

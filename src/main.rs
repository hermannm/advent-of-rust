mod day1;

fn main() -> Result<(), day1::part1::PuzzleError> {
    let solution = day1::part1::solve_puzzle()?;
    println!("Solution: {solution}");

    Ok(())
}

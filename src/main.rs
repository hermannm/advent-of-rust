mod day1;

fn main() -> Result<(), day1::part2::PuzzleError> {
    let solution = day1::part2::solve_puzzle()?;
    println!("Solution: {solution}");

    Ok(())
}

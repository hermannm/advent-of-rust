use self::snafu::SnafuNumber;

mod parsing;
mod snafu;

pub fn solve_puzzle(input: &str) -> Result<String, String> {
    let snafu_numbers = input
        .lines()
        .map(SnafuNumber::try_from)
        .collect::<Result<Vec<SnafuNumber>, String>>()?;

    let sum = snafu_numbers.iter().map(i64::from).sum::<i64>();

    Ok(SnafuNumber::from(sum).to_string())
}

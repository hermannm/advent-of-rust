use super::{directory_sizes::DirectorySizes, file_system::Directory, parsing::InputLine};

pub fn solve_puzzle(input: &str) -> Result<u64, String> {
    let parsed_lines = InputLine::parse_lines(input)?;

    let root_directory = Directory::try_from(parsed_lines)?;

    let directory_sizes = DirectorySizes::from(root_directory);

    let size_sum = directory_sizes
        .into_iter()
        .filter(|&&size| size <= 100_000u64)
        .map(|size| *size)
        .sum();

    Ok(size_sum)
}

use super::{directory_sizes::DirectorySizes, file_system::Directory, parsing::InputLine};

pub fn solve_puzzle(input: &str) -> Result<u64, String> {
    let parsed_lines = InputLine::parse_lines(input)?;

    let root_directory = Directory::try_from(parsed_lines)?;

    let directory_sizes = DirectorySizes::from(root_directory);

    const TOTAL_DISK_SPACE: u64 = 70_000_000;
    const SPACE_NEEDED_FOR_UPDATE: u64 = 30_000_000;

    let available_disk_space = TOTAL_DISK_SPACE - directory_sizes.root_size;
    if available_disk_space > SPACE_NEEDED_FOR_UPDATE {
        return Err("No space needed to be freed".to_string());
    }

    let space_to_free = SPACE_NEEDED_FOR_UPDATE - available_disk_space;

    let size_of_directory_to_delete = directory_sizes
        .into_iter()
        .filter(|&&size| size >= space_to_free)
        .map(|size| *size)
        .min()
        .ok_or("Found no directories big enough to free up space for update".to_string())?;

    Ok(size_of_directory_to_delete)
}

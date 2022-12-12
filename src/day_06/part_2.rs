use super::contiguity::end_index_of_contiguous_different_characters;

pub fn solve_puzzle(input: &str) -> Result<usize, String> {
    end_index_of_contiguous_different_characters(input, 14)
        .ok_or_else(|| String::from("No fourteen contiguous different characters found"))
}

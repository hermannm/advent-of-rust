use super::contiguity::end_index_of_contiguous_different_characters;

pub fn solve_puzzle(input: &str) -> Result<usize, String> {
    end_index_of_contiguous_different_characters(input, 4)
        .ok_or("No four contiguous different characters found".to_string())
}

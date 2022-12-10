use super::shared::end_index_of_contiguous_different_characters;

pub fn solve_puzzle(input: &str) -> Result<usize, String> {
    end_index_of_contiguous_different_characters(input, 14)
        .ok_or("No fourteen contiguous different characters found".to_string())
}

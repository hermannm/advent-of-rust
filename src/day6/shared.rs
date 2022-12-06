pub fn end_index_of_contiguous_different_characters(
    input: &str,
    number_of_different_characters: usize,
) -> Option<usize> {
    'outer: for end_index in number_of_different_characters..(input.len() - 1) {
        let start_index = end_index - number_of_different_characters;

        let four_characters = &input[start_index..end_index];
        let mut checked_characters = Vec::<char>::new();

        for character in four_characters.chars() {
            if checked_characters.contains(&character) {
                continue 'outer;
            } else {
                checked_characters.push(character);
            }
        }

        return Some(end_index);
    }

    None
}

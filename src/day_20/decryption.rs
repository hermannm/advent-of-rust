pub(super) struct EncryptedFile {
    pub number_sequence: Vec<i64>,
}

impl EncryptedFile {
    pub fn mix_n_times(&mut self, n: u32) -> Result<(), String> {
        let mut numbers_with_original_indices = self
            .number_sequence
            .iter()
            .copied()
            .enumerate()
            .collect::<Vec<(usize, i64)>>();

        for _ in 0..n {
            self.mix(&mut numbers_with_original_indices)?;
        }

        self.number_sequence = numbers_with_original_indices
            .into_iter()
            .map(|(_, number)| number)
            .collect::<Vec<i64>>();

        Ok(())
    }

    fn mix(&self, numbers_with_original_indices: &mut Vec<(usize, i64)>) -> Result<(), String> {
        for original_index in 0..self.number_sequence.len() {
            let current_index = numbers_with_original_indices
                .iter()
                .position(|&(index, _)| index == original_index)
                .ok_or_else(|| format!("Failed to find number with index {original_index}"))?;

            let (_, number) = numbers_with_original_indices.remove(current_index);

            let current_index_signed = i64::try_from(current_index).map_err(|_| {
                format!("Failed to convert index '{current_index}' to signed integer")
            })?;

            let target_index = self.wrap_index(
                current_index_signed + number,
                numbers_with_original_indices.len(),
            )?;

            numbers_with_original_indices.insert(target_index, (original_index, number));
        }

        Ok(())
    }

    fn wrap_index(&self, mut index_to_wrap: i64, sequence_length: usize) -> Result<usize, String> {
        let sequence_length_signed = i64::try_from(sequence_length).map_err(|_| {
            format!("Failed to convert sequence length {sequence_length} to signed integer")
        })?;

        if index_to_wrap <= 0 || index_to_wrap >= sequence_length_signed {
            index_to_wrap = index_to_wrap.rem_euclid(sequence_length_signed);
        }

        usize::try_from(index_to_wrap)
            .map_err(|_| format!("Failed to convert index '{index_to_wrap}' to unsigned integer"))
    }

    pub fn get_with_wrapping_index(&self, index: i64) -> Result<i64, String> {
        let index = self.wrap_index(index, self.number_sequence.len())?;

        let number = self
            .number_sequence
            .get(index)
            .ok_or_else(|| format!("No number found at index '{index}'"))?;

        Ok(*number)
    }

    pub fn index_of_number(&self, number: &i64) -> Option<usize> {
        self.number_sequence
            .iter()
            .position(|other_number| other_number == number)
    }

    pub fn apply_decryption_key(&mut self, key: i64) {
        for number in self.number_sequence.iter_mut() {
            *number *= key;
        }
    }
}

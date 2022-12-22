use super::decryption::EncryptedFile;

pub fn solve_puzzle(input: &str) -> Result<i64, String> {
    let mut file = EncryptedFile::try_from(input)?;

    const DECRYPTION_KEY: i64 = 811589153;
    file.apply_decryption_key(DECRYPTION_KEY);
    file.mix_n_times(10)?;

    let index_of_zero = file
        .index_of_number(&0)
        .ok_or_else(|| String::from("Failed to find '0' in number sequence"))?;

    let mut sum = 0;
    for index in [1000, 2000, 3000] {
        sum += file.get_with_wrapping_index(index_of_zero as i64 + index)?;
    }

    Ok(sum)
}

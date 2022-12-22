use super::decryption::EncryptedFile;

impl TryFrom<&str> for EncryptedFile {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let number_sequence = input
            .lines()
            .map(|line| {
                line.parse::<i64>()
                    .map_err(|_| format!("Failed to parse '{line}' to integer"))
            })
            .collect::<Result<Vec<i64>, String>>()?;

        Ok(Self { number_sequence })
    }
}

use super::snafu::{SnafuDigit, SnafuNumber};

impl TryFrom<&str> for SnafuNumber {
    type Error = String;

    fn try_from(input_line: &str) -> Result<Self, Self::Error> {
        let digits_in_reverse_order = input_line
            .chars()
            .rev()
            .map(SnafuDigit::try_from)
            .collect::<Result<Vec<SnafuDigit>, String>>()?;

        Ok(Self {
            digits_in_reverse_order,
        })
    }
}

impl TryFrom<char> for SnafuDigit {
    type Error = String;

    fn try_from(character: char) -> Result<Self, Self::Error> {
        use SnafuDigit::*;

        let digit = match character {
            '=' => DoubleMinus,
            '-' => Minus,
            '0' => Zero,
            '1' => One,
            '2' => Two,
            _ => {
                return Err(format!(
                    "Input character not unrecognized as SNAFU digit: {character}"
                ));
            }
        };

        Ok(digit)
    }
}

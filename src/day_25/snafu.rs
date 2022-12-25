#[derive(Debug)]
pub struct SnafuNumber {
    pub digits_in_reverse_order: Vec<SnafuDigit>,
}

impl From<&SnafuNumber> for i64 {
    fn from(snafu: &SnafuNumber) -> Self {
        let mut decimal: i64 = 0;

        for (index, digit) in (0u32..).zip(snafu.digits_in_reverse_order.iter()) {
            decimal += i64::from(digit) * 5i64.pow(index);
        }

        decimal
    }
}

impl From<i64> for SnafuNumber {
    fn from(mut decimal_number: i64) -> Self {
        let mut snafu_digits = Vec::<SnafuDigit>::new();
        let mut current_place = 1;

        while decimal_number != 0 {
            use SnafuDigit::*;

            for (snafu_digit, positive_remainder_to_check, negative_remainder_to_check) in [
                (Zero, 0, 0),
                (One, 1, -4),
                (Two, 2, -3),
                (DoubleMinus, 3, -2),
                (Minus, 4, -1),
            ] {
                let remainder_to_check = if decimal_number >= 0 {
                    positive_remainder_to_check
                } else {
                    negative_remainder_to_check
                };

                if decimal_number % (current_place * 5) == (remainder_to_check * current_place) {
                    decimal_number -= i64::from(&snafu_digit) * current_place;

                    snafu_digits.push(snafu_digit);
                }
            }

            current_place *= 5;
        }

        SnafuNumber {
            digits_in_reverse_order: snafu_digits,
        }
    }
}

impl ToString for SnafuNumber {
    fn to_string(&self) -> String {
        let string = self
            .digits_in_reverse_order
            .iter()
            .rev()
            .map(SnafuDigit::to_char)
            .collect::<String>();

        if string.is_empty() {
            String::from("0")
        } else {
            string
        }
    }
}

#[derive(Debug)]
pub enum SnafuDigit {
    DoubleMinus,
    Minus,
    Zero,
    One,
    Two,
}

impl From<&SnafuDigit> for i64 {
    fn from(snafu: &SnafuDigit) -> Self {
        use SnafuDigit::*;

        match snafu {
            DoubleMinus => -2,
            Minus => -1,
            Zero => 0,
            One => 1,
            Two => 2,
        }
    }
}

impl SnafuDigit {
    fn to_char(&self) -> char {
        use SnafuDigit::*;

        match self {
            DoubleMinus => '=',
            Minus => '-',
            Zero => '0',
            One => '1',
            Two => '2',
        }
    }
}

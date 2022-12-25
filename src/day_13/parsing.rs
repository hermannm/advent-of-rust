use std::iter::once;

use super::integers_and_lists::{IntegerOrList, Pair};

impl TryFrom<(&str, &str)> for Pair {
    type Error = String;

    fn try_from(line_pair: (&str, &str)) -> Result<Self, Self::Error> {
        let left = Vec::<IntegerOrList>::from_list_string(line_pair.0).map_err(String::from)?;
        let right = Vec::<IntegerOrList>::from_list_string(line_pair.1).map_err(String::from)?;

        Ok(Self { left, right })
    }
}

pub(super) trait FromListString: Sized {
    fn from_list_string(list_string: &str) -> Result<Self, FromListStringError>;
}

impl FromListString for Vec<IntegerOrList> {
    fn from_list_string(mut list_string: &str) -> Result<Self, FromListStringError> {
        if !(list_string.starts_with('[') && list_string.ends_with(']')) {
            return Err(FromListStringError::NotListString);
        }

        list_string = &list_string[1..(list_string.len() - 1)];

        let mut list = Vec::<IntegerOrList>::new();
        let mut open_brackets = 0i32;
        let mut current_element = String::new();

        for char in list_string.chars().chain(once(',')) {
            match char {
                ',' => {
                    if open_brackets == 0 {
                        if !current_element.is_empty() {
                            match IntegerOrList::try_from(current_element.as_str()) {
                                Ok(integer_or_list) => {
                                    list.push(integer_or_list);
                                    current_element.clear();
                                }
                                Err(error) => {
                                    return Err(FromListStringError::ParseError(error));
                                }
                            }
                        }
                    } else {
                        current_element.push(char);
                    }
                }
                '[' => {
                    open_brackets += 1;
                    current_element.push(char);
                }
                ']' => {
                    open_brackets -= 1;
                    current_element.push(char);
                }
                char => {
                    current_element.push(char);
                }
            }
        }

        Ok(list)
    }
}

pub(super) enum FromListStringError {
    NotListString,
    ParseError(String),
}

impl From<FromListStringError> for String {
    fn from(error: FromListStringError) -> Self {
        match error {
            FromListStringError::NotListString => String::from("Input line was not a list"),
            FromListStringError::ParseError(error) => error,
        }
    }
}

impl TryFrom<&str> for IntegerOrList {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        use IntegerOrList::*;

        match Vec::<IntegerOrList>::from_list_string(input) {
            Ok(list) => Ok(List(list)),
            Err(FromListStringError::ParseError(error)) => Err(error),
            Err(FromListStringError::NotListString) => {
                let integer = input
                    .parse::<i32>()
                    .map_err(|_| format!("Failed to parse input '{input}' to integer"))?;

                Ok(Integer(integer))
            }
        }
    }
}

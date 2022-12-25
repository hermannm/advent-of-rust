use super::falling_rocks::Movement;

pub(super) fn jet_pattern_from_input(input: &str) -> Result<Vec<Movement>, String> {
    input
        .chars()
        .map(Movement::try_from)
        .collect::<Result<Vec<Movement>, String>>()
}

impl TryFrom<char> for Movement {
    type Error = String;

    fn try_from(character: char) -> Result<Self, Self::Error> {
        match character {
            '<' => Ok(Movement::Left),
            '>' => Ok(Movement::Right),
            _ => Err(format!("Unrecognized character: {character}")),
        }
    }
}

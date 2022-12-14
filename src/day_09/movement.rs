pub(super) struct Movement {
    pub direction: Direction,
    pub steps: u32,
}

impl TryFrom<&str> for Movement {
    type Error = String;

    fn try_from(input_line: &str) -> Result<Self, Self::Error> {
        let (direction_str, steps_str) = input_line
            .split_once(' ')
            .ok_or_else(|| String::from("Input line did not contain space"))?;

        let direction = Direction::try_from(direction_str)?;

        let steps = steps_str
            .parse::<u32>()
            .map_err(|_| String::from("Failed to parse second part of input line as digit"))?;

        Ok(Self { direction, steps })
    }
}

pub(super) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<&str> for Direction {
    type Error = String;

    fn try_from(direction_str: &str) -> Result<Self, Self::Error> {
        use Direction::*;

        match direction_str {
            "U" => Ok(Up),
            "D" => Ok(Down),
            "L" => Ok(Left),
            "R" => Ok(Right),
            _ => Err(format!("Unrecognized direction: {direction_str}")),
        }
    }
}

pub struct Movement {
    pub direction: Direction,
    pub steps: u32,
}

impl TryFrom<&str> for Movement {
    type Error = String;

    fn try_from(input_line: &str) -> Result<Self, Self::Error> {
        let (direction_str, steps_str) = input_line
            .split_once(' ')
            .ok_or("Input line did not contain space".to_string())?;

        let direction = Direction::try_from(direction_str)?;

        let steps = steps_str
            .parse::<u32>()
            .map_err(|_| "Failed to parse second part of input line as digit".to_string())?;

        Ok(Self { direction, steps })
    }
}

pub enum Direction {
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

pub struct Pair {
    pub range1: Range,
    pub range2: Range,
}

impl TryFrom<&str> for Pair {
    type Error = String;

    fn try_from(input_string: &str) -> Result<Self, Self::Error> {
        let pair_strings: Vec<&str> = input_string.split(',').collect();
        if pair_strings.len() != 2 {
            return Err("Input line did not contain 2 pairs".to_string());
        }

        let range1 = Range::try_from(pair_strings[0])?;
        let range2 = Range::try_from(pair_strings[1])?;

        Ok(Self { range1, range2 })
    }
}

pub struct Range {
    min: i32,
    max: i32,
}

impl Range {
    pub fn fully_contains(&self, other: &Range) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        (self.min <= other.max && self.max >= other.min)
            || (other.min <= self.max && other.max >= self.min)
    }
}

impl TryFrom<&str> for Range {
    type Error = String;

    fn try_from(input_string: &str) -> Result<Self, Self::Error> {
        let range_strings: Vec<&str> = input_string.split('-').collect();
        if range_strings.len() != 2 {
            return Err("Pair range did not contain 2 parts separated by '-'".to_string());
        }

        let min = range_strings[0]
            .parse::<i32>()
            .map_err(|_| "Failed to parse first range number".to_string())?;
        let max = range_strings[1]
            .parse::<i32>()
            .map_err(|_| "Failed to parse second range number".to_string())?;

        Ok(Self { min, max })
    }
}

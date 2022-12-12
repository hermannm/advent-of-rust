pub struct Range {
    pub min: i32,
    pub max: i32,
}

impl Range {
    pub fn fully_contains(&self, other: &Range) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        self.min <= other.max && self.max >= other.min
    }
}

impl TryFrom<&str> for Range {
    type Error = String;

    fn try_from(input_string: &str) -> Result<Self, Self::Error> {
        let range_strings: Vec<&str> = input_string.split('-').collect();
        if range_strings.len() != 2 {
            return Err(String::from(
                "Pair range did not contain 2 parts separated by '-'",
            ));
        }

        let min = range_strings[0]
            .parse::<i32>()
            .map_err(|_| String::from("Failed to parse first range number"))?;
        let max = range_strings[1]
            .parse::<i32>()
            .map_err(|_| String::from("Failed to parse second range number"))?;

        Ok(Self { min, max })
    }
}

pub struct RangePair {
    pub range_1: Range,
    pub range_2: Range,
}

impl TryFrom<&str> for RangePair {
    type Error = String;

    fn try_from(input_string: &str) -> Result<Self, Self::Error> {
        let pair_strings: Vec<&str> = input_string.split(',').collect();
        if pair_strings.len() != 2 {
            return Err(String::from("Input line did not contain 2 pairs"));
        }

        let range_1 = Range::try_from(pair_strings[0])?;
        let range_2 = Range::try_from(pair_strings[1])?;

        Ok(Self { range_1, range_2 })
    }
}

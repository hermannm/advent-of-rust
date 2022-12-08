use super::forest::{Forest, Tree};

impl TryFrom<&str> for Forest {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let rows = input.lines().count();
        let columns = input.lines().next().unwrap_or("").len();

        let mut trees = Vec::<Vec<Tree>>::new();

        for line in input.lines() {
            let treeline = line
                .chars()
                .map(|height_char| Tree::try_from(height_char))
                .collect::<Result<Vec<Tree>, String>>()?;

            if treeline.len() != columns {
                return Err("Treeline did match number of treelines in the forest".to_string());
            }

            trees.push(treeline);
        }

        if trees.len() != rows {
            return Err(
                "Number of treelines in the forest did not match expected value".to_string(),
            );
        }

        Ok(Self {
            trees,
            rows,
            columns,
        })
    }
}

impl TryFrom<char> for Tree {
    type Error = String;

    fn try_from(height_char: char) -> Result<Self, Self::Error> {
        let height = height_char
            .to_digit(10)
            .ok_or("Failed to parse input height character to integer".to_string())?;

        Ok(Self { height })
    }
}

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
                .map(Tree::try_from)
                .collect::<Result<Vec<Tree>, String>>()?;

            if treeline.len() != columns {
                return Err(String::from(
                    "Treeline did match number of treelines in the forest",
                ));
            }

            trees.push(treeline);
        }

        if trees.len() != rows {
            return Err(String::from(
                "Number of treelines in the forest did not match expected value",
            ));
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
            .ok_or_else(|| String::from("Failed to parse input height character to integer"))?;

        Ok(Self { height })
    }
}

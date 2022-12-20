use super::blueprint::{Blueprint, OreAndClayCost, OreAndObsidianCost, OreCost};

impl Blueprint {
    pub fn blueprints_from_input(input: &str) -> Result<Vec<Blueprint>, String> {
        input
            .lines()
            .map(Blueprint::try_from)
            .collect::<Result<Vec<Blueprint>, String>>()
    }
}

impl TryFrom<&str> for Blueprint {
    type Error = String;

    fn try_from(input_line: &str) -> Result<Self, Self::Error> {
        let (_, rest_of_line) = input_line
            .split_once("Blueprint ")
            .ok_or_else(|| format!("Expected to find 'Blueprint' in line: '{input_line}'"))?;

        let (id_string, rest_of_line) = rest_of_line
            .split_once(':')
            .ok_or_else(|| format!("Expected to find ':' in line: '{rest_of_line}'"))?;

        let id = id_string
            .parse::<u32>()
            .map_err(|_| format!("Failed to parse ID string '{id_string}' to integer"))?;

        let cost_strings = rest_of_line.split('.').collect::<Vec<&str>>();

        if cost_strings.len() < 4 {
            return Err(format!(
                "Expected 4 robot cost strings in line: {rest_of_line}"
            ));
        }

        let ore_robot_cost = OreCost::try_from(cost_strings[0])?;
        let clay_robot_cost = OreCost::try_from(cost_strings[1])?;
        let obsidian_robot_cost = OreAndClayCost::try_from(cost_strings[2])?;
        let geode_robot_cost = OreAndObsidianCost::try_from(cost_strings[3])?;

        Ok(Self {
            id,
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,
        })
    }
}

impl TryFrom<&str> for OreCost {
    type Error = String;

    fn try_from(cost_string: &str) -> Result<Self, Self::Error> {
        let (_, ore_cost_string) = cost_string
            .split_once("Each ore robot costs ")
            .or_else(|| cost_string.split_once("Each clay robot costs "))
            .ok_or_else(|| {
                format!(
                    "Expected to find 'Each ore robot costs' or 'Each clay robot costs' in cost string '{cost_string}'"
                )
            })?;

        let ore = parse_number_string_with_suffix(ore_cost_string, " ore")?;

        Ok(Self { ore })
    }
}

impl TryFrom<&str> for OreAndClayCost {
    type Error = String;

    fn try_from(cost_string: &str) -> Result<Self, Self::Error> {
        let (_, ore_and_clay_cost_string) = cost_string
            .split_once("Each obsidian robot costs ")
            .ok_or_else(|| {
                format!(
                    "Expected to find 'Each obsidian robot costs' in cost string '{cost_string}'"
                )
            })?;

        let (ore, clay) = parse_two_number_strings_with_suffixes(
            ore_and_clay_cost_string,
            " ore",
            " and ",
            " clay",
        )?;

        Ok(Self { ore, clay })
    }
}

impl TryFrom<&str> for OreAndObsidianCost {
    type Error = String;

    fn try_from(cost_string: &str) -> Result<Self, Self::Error> {
        let (_, ore_and_obsidian_cost_string) = cost_string
            .split_once("Each geode robot costs ")
            .ok_or_else(|| {
            format!("Expected to find 'Each geode robot costs' in cost string '{cost_string}'")
        })?;

        let (ore, obsidian) = parse_two_number_strings_with_suffixes(
            ore_and_obsidian_cost_string,
            " ore",
            " and ",
            " obsidian",
        )?;

        Ok(Self { ore, obsidian })
    }
}

fn parse_number_string_with_suffix(string: &str, suffix: &str) -> Result<u32, String> {
    let (number_string, _) = string
        .split_once(suffix)
        .ok_or_else(|| format!("Expected to find '{suffix}' in string '{string}'"))?;

    let number = number_string
        .parse::<u32>()
        .map_err(|_| format!("Failed to parse string '{number_string}' to integer"))?;

    Ok(number)
}

fn parse_two_number_strings_with_suffixes(
    string: &str,
    suffix_1: &str,
    separator: &str,
    suffix_2: &str,
) -> Result<(u32, u32), String> {
    let (suffixed_string_1, suffixed_string_2) = string
        .split_once(separator)
        .ok_or_else(|| format!("Expected to find '{separator}' in string '{string}'"))?;

    let number_1 = parse_number_string_with_suffix(suffixed_string_1, suffix_1)?;
    let number_2 = parse_number_string_with_suffix(suffixed_string_2, suffix_2)?;

    Ok((number_1, number_2))
}

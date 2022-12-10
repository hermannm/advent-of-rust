use super::rucksack::{Item, Priority, Rucksack};

pub fn solve_puzzle(input: &str) -> Result<i32, String> {
    let lines = input.lines().collect();
    let groups = split_into_groups(lines)?;

    let mut priority_sum = 0;

    for group in groups {
        let badge_item = find_badge_item(&group).ok_or("Badge item not found".to_string())?;
        let priority = badge_item.get_priority()?;
        priority_sum += priority;
    }

    Ok(priority_sum)
}

fn split_into_groups(input_lines: Vec<&str>) -> Result<Vec<[Rucksack; 3]>, String> {
    let mut groups = Vec::<[Rucksack; 3]>::new();

    let input_line_count = input_lines.len();

    if input_line_count % 3 != 0 {
        return Err("Input lines not divisible into groups of 3".to_string());
    }

    for line_index in (0..(input_line_count - 1)).step_by(3) {
        groups.push([
            Rucksack::try_from(input_lines[line_index])?,
            Rucksack::try_from(input_lines[line_index + 1])?,
            Rucksack::try_from(input_lines[line_index + 2])?,
        ])
    }

    Ok(groups)
}

fn find_badge_item(group: &[Rucksack; 3]) -> Option<Item> {
    group[0]
        .contents
        .iter()
        .find(|item| group[1].contents.contains(&item) && group[2].contents.contains(&item))
        .copied()
}

use super::shared::{Item, Priority};

#[allow(dead_code)]
pub fn solve_puzzle() -> Result<i32, String> {
    let input = include_str!("input.txt");

    let lines = input.lines().collect();
    let groups = split_into_groups(lines)?;

    let mut priority_sum = 0;

    for group in groups {
        let badge_item = find_badge_item(group).ok_or("Badge item not found".to_string())?;
        let priority = badge_item.get_priority()?;
        priority_sum += priority;
    }

    Ok(priority_sum)
}

fn split_into_groups(input_lines: Vec<&str>) -> Result<Vec<[String; 3]>, String> {
    let mut groups = Vec::<[String; 3]>::new();

    let input_line_count = input_lines.len();

    if input_line_count % 3 != 0 {
        return Err("Input lines not divisible into groups of 3".to_string());
    }

    for line_index in (0..(input_line_count - 1)).step_by(3) {
        groups.push([
            input_lines[line_index].to_string(),
            input_lines[line_index + 1].to_string(),
            input_lines[line_index + 2].to_string(),
        ])
    }

    Ok(groups)
}

fn find_badge_item(group: [String; 3]) -> Option<Item> {
    for item in group[0].chars() {
        if group[1].contains(|other_item| item == other_item)
            && group[2].contains(|other_item| item == other_item)
        {
            return Some(item);
        }
    }

    None
}

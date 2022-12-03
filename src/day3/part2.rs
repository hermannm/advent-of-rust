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

type Item = char;

trait Priority {
    fn get_priority(&self) -> Result<i32, String>;
}

impl Priority for Item {
    fn get_priority(&self) -> Result<i32, String> {
        let priority = ITEM_TYPES
            .iter()
            .position(|item| item == self)
            .ok_or("Invalid item type".to_string())?;

        i32::try_from(priority + 1)
            .map_err(|_| "Could not convert priority to 32-bit integer".to_string())
    }
}

static ITEM_TYPES: [Item; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

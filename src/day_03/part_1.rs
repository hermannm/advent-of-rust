use super::rucksack::{Priority, Rucksack};

pub fn solve_puzzle(input: &str) -> Result<i32, String> {
    let mut priority_sum = 0;

    for line in input.lines() {
        let rucksack = Rucksack::try_from(line)?;
        let shared_item = rucksack.find_shared_item_in_compartments()?;
        let priority = shared_item.get_priority()?;
        priority_sum += priority;
    }

    Ok(priority_sum)
}

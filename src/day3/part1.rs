use super::shared::{Item, Priority, Rucksack};

pub fn solve_puzzle(input: &str) -> Result<i32, String> {
    let mut priority_sum = 0;

    for line in input.lines() {
        let rucksack = Rucksack::new(line)?;
        let shared_item = rucksack.find_shared_item_in_compartments()?;
        let priority = shared_item.get_priority()?;
        priority_sum += priority;
    }

    Ok(priority_sum)
}

impl Rucksack {
    fn find_shared_item_in_compartments(&self) -> Result<Item, String> {
        let [compartment1, compartment2] = self.compartment_contents();

        for item in compartment1 {
            if compartment2.contains(&item) {
                return Ok(item);
            }
        }

        Err("No shared item type found between the two compartments".to_string())
    }

    fn compartment_contents(&self) -> [Vec<Item>; 2] {
        let (compartment1, compartment2) = self.contents.split_at(self.contents.len() / 2);

        [
            Vec::<Item>::from(compartment1),
            Vec::<Item>::from(compartment2),
        ]
    }
}

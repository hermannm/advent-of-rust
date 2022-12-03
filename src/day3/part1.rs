#[allow(dead_code)]
pub fn solve_puzzle() -> Result<i32, String> {
    let input = include_str!("input.txt");

    let mut priority_sum = 0;

    for line in input.lines() {
        let rucksack = Rucksack::new(line)?;
        let shared_item = rucksack.find_shared_item()?;
        let priority = shared_item.get_priority()?;
        priority_sum += priority;
    }

    Ok(priority_sum)
}

struct Rucksack {
    compartments: [String; 2],
}

impl Rucksack {
    fn new(input_string: &str) -> Result<Self, String> {
        let input_length = input_string.len();
        if input_length % 2 != 0 {
            return Err("Rucksack input string has odd number of characters".to_string());
        }

        let (compartment1, compartment2) = input_string.split_at(input_length / 2);

        Ok(Self {
            compartments: [compartment1.to_string(), compartment2.to_string()],
        })
    }
}

impl Rucksack {
    fn find_shared_item(&self) -> Result<Item, String> {
        for item_type in self.compartments[0].chars() {
            if self.compartments[1].contains(|other_item_type| item_type == other_item_type) {
                return Ok(item_type);
            }
        }

        Err("No shared item type found between the two compartments".to_string())
    }
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

pub(super) struct Rucksack {
    pub contents: Vec<Item>,
}

impl Rucksack {
    pub fn compartment_contents(&self) -> [Vec<Item>; 2] {
        let (compartment_1, compartment_2) = self.contents.split_at(self.contents.len() / 2);

        [
            Vec::<Item>::from(compartment_1),
            Vec::<Item>::from(compartment_2),
        ]
    }

    pub fn find_shared_item_in_compartments(&self) -> Result<Item, String> {
        let [compartment_1, compartment_2] = self.compartment_contents();

        for item in compartment_1 {
            if compartment_2.contains(&item) {
                return Ok(item);
            }
        }

        Err(String::from(
            "No shared item type found between the two compartments",
        ))
    }
}

impl TryFrom<&str> for Rucksack {
    type Error = String;

    fn try_from(content_string: &str) -> Result<Self, Self::Error> {
        if content_string.len() % 2 != 0 {
            return Err(String::from(
                "Rucksack input string has odd number of characters",
            ));
        }

        let contents = content_string.chars().collect::<Vec<Item>>();

        Ok(Self { contents })
    }
}

pub(super) type Item = char;

pub(super) trait Priority {
    fn get_priority(&self) -> Result<i32, String>;
}

impl Priority for Item {
    fn get_priority(&self) -> Result<i32, String> {
        let priority = ITEM_TYPES
            .iter()
            .position(|item| item == self)
            .ok_or_else(|| String::from("Invalid item type"))?;

        i32::try_from(priority + 1)
            .map_err(|_| String::from("Could not convert priority to 32-bit integer"))
    }
}

static ITEM_TYPES: [Item; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

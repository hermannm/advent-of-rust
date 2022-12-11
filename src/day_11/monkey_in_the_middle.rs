use std::collections::HashMap;

pub fn play_monkey_in_the_middle(
    monkeys: &mut Vec<Monkey>,
    until_round: u32,
    relief_after_inspection: bool,
) -> Result<(), String> {
    let monkey_tests_common_denominator = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible_by)
        .product::<u64>();

    let mut round = 0u32;

    while round < until_round {
        round += 1;

        for index in 0..monkeys.len() {
            let mut thrown_items = HashMap::<i32, Vec<Item>>::new();

            let monkey = &mut monkeys[index];

            loop {
                if monkey.items.is_empty() {
                    break;
                }

                let mut item = monkey.items.remove(0);

                monkey.inspect_item(&mut item, relief_after_inspection);

                item.worry_level %= monkey_tests_common_denominator;

                monkey.throw_item(item, &mut thrown_items);
            }

            for (monkey_id, items) in thrown_items.iter_mut() {
                let monkey = monkeys
                    .iter_mut()
                    .find(|monkey| monkey.id == *monkey_id)
                    .ok_or(format!("Monkey with ID '{monkey_id}' not found"))?;

                monkey.items.append(items);
            }
        }
    }

    Ok(())
}

pub struct Monkey {
    pub id: i32,
    pub items: Vec<Item>,
    pub operation: WorryLevelOperation,
    pub test: WorryLevelTest,
    pub item_inspection_count: u64,
}

impl Monkey {
    fn inspect_item(&mut self, item: &mut Item, relief_after_inspection: bool) {
        use WorryLevelOperation::*;

        match &self.operation {
            Add(value) => {
                item.worry_level += value;
            }
            MultiplyWith(value) => {
                item.worry_level *= value;
            }
            MultiplyWithSelf => {
                item.worry_level *= item.worry_level;
            }
        }

        if relief_after_inspection {
            item.worry_level /= 3;
        }

        self.item_inspection_count += 1;
    }

    fn throw_item(&self, item: Item, thrown_items: &mut HashMap<i32, Vec<Item>>) {
        let target_monkey = {
            if &item.worry_level % &self.test.divisible_by == 0 {
                self.test.target_monkey_if_true
            } else {
                self.test.target_monkey_if_false
            }
        };

        let target_monkey_items = thrown_items
            .entry(target_monkey)
            .or_insert(Vec::<Item>::new());

        target_monkey_items.push(item);
    }
}

pub struct Item {
    pub worry_level: u64,
}

pub enum WorryLevelOperation {
    Add(u64),
    MultiplyWith(u64),
    MultiplyWithSelf,
}

pub struct WorryLevelTest {
    pub divisible_by: u64,
    pub target_monkey_if_true: i32,
    pub target_monkey_if_false: i32,
}

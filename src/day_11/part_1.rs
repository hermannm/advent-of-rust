use super::monkey_in_the_middle::{play_monkey_in_the_middle, Monkey};

pub fn solve_puzzle(input: &str) -> Result<u64, String> {
    let mut monkeys = Monkey::monkeys_from_input_lines(input)?;

    play_monkey_in_the_middle(&mut monkeys, 20, true)?;

    monkeys.sort_by(|monkey_1, monkey_2| {
        monkey_2
            .item_inspection_count
            .cmp(&monkey_1.item_inspection_count)
    });

    let monkey_business = monkeys[0].item_inspection_count * monkeys[1].item_inspection_count;

    Ok(monkey_business)
}

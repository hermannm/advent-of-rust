use super::monkeys::Monkeys;

pub fn solve_puzzle(input: &str) -> Result<i64, String> {
    let mut monkeys = Monkeys::try_from(input)?;

    let root_shout = monkeys
        .get_number_yelled_by("root")
        .ok_or_else(|| String::from("Failed to find value shouted by 'root'"))?;

    Ok(root_shout)
}

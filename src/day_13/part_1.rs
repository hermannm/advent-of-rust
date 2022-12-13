use super::integers_and_lists::Pair;

pub fn solve_puzzle(input: &str) -> Result<usize, String> {
    let pairs = input
        .split("\n\n")
        .map(|line_pair_string| {
            let line_pair = line_pair_string
                .split_once('\n')
                .ok_or_else(|| String::from("Failed to split line pair at newline"))?;

            Pair::try_from(line_pair)
        })
        .collect::<Result<Vec<Pair>, String>>()?;

    let right_order_pair_index_sum = pairs
        .iter()
        .enumerate()
        .flat_map(|(index, pair)| {
            if pair.is_in_right_order() {
                Some(index + 1)
            } else {
                None
            }
        })
        .sum::<usize>();

    Ok(right_order_pair_index_sum)
}

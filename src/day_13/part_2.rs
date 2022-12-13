use std::cmp::Ordering;

use super::{
    integers_and_lists::{CheckOrder, IntegerOrList, Order},
    parsing::{FromListString, FromListStringError},
};

pub fn solve_puzzle(input: &str) -> Result<usize, String> {
    let mut packets = input
        .replace("\n\n", "\n")
        .lines()
        .map(Vec::<IntegerOrList>::from_list_string)
        .collect::<Result<Vec<Vec<IntegerOrList>>, FromListStringError>>()
        .map_err(String::from)?;

    let dividers = ["[[2]]", "[[6]]"]
        .iter()
        .map(|divider_string| {
            let divider =
                Vec::<IntegerOrList>::from_list_string(divider_string).map_err(String::from)?;
            packets.push(divider.clone());
            Ok(divider)
        })
        .collect::<Result<Vec<Vec<IntegerOrList>>, String>>()?;

    packets.sort_by(
        |left, right| match Vec::<IntegerOrList>::check_order(left, right) {
            Order::Right => Ordering::Less,
            Order::Wrong => Ordering::Greater,
            Order::Indecisive => Ordering::Equal,
        },
    );

    let divider_index_product = packets
        .iter()
        .enumerate()
        .flat_map(|(index, packet)| {
            for divider in &dividers {
                if packet == divider {
                    return Some(index + 1);
                }
            }

            None
        })
        .product::<usize>();

    Ok(divider_index_product)
}

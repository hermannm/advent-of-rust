use std::cmp::Ordering;

pub struct Pair {
    pub left: Vec<IntegerOrList>,
    pub right: Vec<IntegerOrList>,
}

#[derive(PartialEq, Clone)]
pub enum IntegerOrList {
    Integer(i32),
    List(Vec<IntegerOrList>),
}

impl Pair {
    pub fn is_in_right_order(&self) -> bool {
        matches!(
            Vec::<IntegerOrList>::check_order(&self.left, &self.right),
            Order::Right
        )
    }
}

pub enum Order {
    Right,
    Wrong,
    Indecisive,
}

pub trait CheckOrder {
    fn check_order(left: &Self, right: &Self) -> Order;
}

impl CheckOrder for Vec<IntegerOrList> {
    fn check_order(left: &Vec<IntegerOrList>, right: &Vec<IntegerOrList>) -> Order {
        for (index, left_value) in left.iter().enumerate() {
            if let Some(right_value) = right.get(index) {
                let order = IntegerOrList::check_order(left_value, right_value);

                if let Order::Indecisive = order {
                    continue;
                } else {
                    return order;
                }
            } else {
                return Order::Wrong;
            }
        }

        if left.len() < right.len() {
            Order::Right
        } else {
            Order::Indecisive
        }
    }
}

impl CheckOrder for IntegerOrList {
    fn check_order(left: &IntegerOrList, right: &IntegerOrList) -> Order {
        use IntegerOrList::*;

        match (left, right) {
            (Integer(left), Integer(right)) => {
                use Ordering::*;

                match left.cmp(right) {
                    Less => Order::Right,
                    Equal => Order::Indecisive,
                    Greater => Order::Wrong,
                }
            }
            (Integer(left), List(right)) => {
                let left_list = vec![Integer(*left)];
                Vec::<IntegerOrList>::check_order(&left_list, right)
            }
            (List(left), Integer(right)) => {
                let right_list = vec![Integer(*right)];
                Vec::<IntegerOrList>::check_order(left, &right_list)
            }
            (List(left), List(right)) => Vec::<IntegerOrList>::check_order(left, right),
        }
    }
}

use std::collections::HashSet;

pub struct Cave {
    pub rocks: HashSet<Position>,
    pub lowest_rock: i32,

    pub sand: HashSet<Position>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

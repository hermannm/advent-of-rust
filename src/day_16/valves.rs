use std::collections::HashMap;

#[derive(Debug)]
pub struct ValveMap(pub HashMap<String, Valve>);

#[derive(Debug)]
pub struct Valve {
    pub flow_rate: u32,
    pub connected_valves: Vec<String>,
}

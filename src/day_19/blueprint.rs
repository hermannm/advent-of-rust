use std::collections::HashMap;

use super::{resources::Resource, robots::Robot, search_state::SearchState};

pub struct Blueprint {
    pub id: u32,
    pub robot_costs: HashMap<Robot, Cost>,
}

pub type Cost = HashMap<Resource, u16>;

impl Blueprint {
    pub fn quality_level(&self, minutes_to_open_geodes: u16) -> u32 {
        self.id * u32::from(self.max_geodes(minutes_to_open_geodes))
    }

    pub fn max_geodes(&self, minutes_to_open_geodes: u16) -> u16 {
        SearchState::new(minutes_to_open_geodes).recursively_find_max_geodes(self)
    }

    pub fn cost_of_robot(&self, robot: &Robot) -> &Cost {
        self.robot_costs
            .get(robot)
            .expect("Blueprint should always have costs for every robot")
    }

    pub fn max_cost_of_resource(&self, resource: &Resource) -> u16 {
        self.robot_costs
            .iter()
            .flat_map(|(_, cost)| cost.iter())
            .filter_map(|(other_resource, resource_cost)| {
                if other_resource == resource {
                    Some(*resource_cost)
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(0)
    }
}

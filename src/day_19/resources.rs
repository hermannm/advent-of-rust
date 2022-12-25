use std::collections::HashMap;

use super::{
    blueprint::Cost,
    robots::{Robot, Robots, NUMBER_OF_ROBOTS},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Clone)]
pub(super) struct Resources {
    map: HashMap<Resource, u16>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            map: HashMap::<Resource, u16>::with_capacity(NUMBER_OF_ROBOTS),
        }
    }

    pub fn get(&self, resource: &Resource) -> u16 {
        *self.map.get(resource).unwrap_or(&0)
    }

    pub fn try_pay(&self, cost: &Cost) -> Option<Resources> {
        let mut new_resources = self.clone();

        for (resource, resource_cost) in cost {
            let new_resource_amount = self.get(resource).checked_sub(*resource_cost)?;

            new_resources.map.insert(*resource, new_resource_amount);
        }

        Some(new_resources)
    }

    pub fn collect_from_robots(&mut self, robots: &Robots) {
        for (robot, robot_count) in robots.into_iter() {
            let resource_count = self.map.entry(Resource::from(robot)).or_insert(0);

            *resource_count += robot_count
        }
    }
}

impl From<&Robot> for Resource {
    fn from(robot: &Robot) -> Self {
        use Resource::*;
        use Robot::*;

        match robot {
            OreCollector => Ore,
            ClayCollector => Clay,
            ObsidianCollector => Obsidian,
            GeodeCracker => Geode,
        }
    }
}

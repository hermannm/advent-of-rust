use std::collections::HashSet;

use super::{
    blueprint::Blueprint,
    resources::{Resource, Resources},
    robots::{Robot, Robots, NUMBER_OF_ROBOTS},
};

#[derive(Clone)]
pub(super) struct SearchState {
    minutes_left: u16,
    resources: Resources,
    robots: Robots,
    skipped_robots: HashSet<Robot>,
}

impl SearchState {
    pub fn new(minutes_to_open_geodes: u16) -> Self {
        Self {
            minutes_left: minutes_to_open_geodes,
            resources: Resources::new(),
            robots: Robots::default(),
            skipped_robots: HashSet::<Robot>::with_capacity(NUMBER_OF_ROBOTS),
        }
    }

    pub fn recursively_find_max_geodes(&self, blueprint: &Blueprint) -> u16 {
        if self.minutes_left == 0 {
            return self.resources.get(&Resource::Geode);
        }

        self.next_states(blueprint)
            .iter()
            .map(|next_state| next_state.recursively_find_max_geodes(blueprint))
            .max()
            .unwrap_or(0)
    }

    fn next_states(&self, blueprint: &Blueprint) -> Vec<SearchState> {
        use Robot::*;

        let mut states = Vec::<SearchState>::new();
        let mut robots_skipped_if_waiting = self.skipped_robots.clone();

        for robot in [GeodeCracker, ObsidianCollector, ClayCollector, OreCollector] {
            if let Some(next_state) = self.try_buy_robot(robot.clone(), blueprint) {
                states.push(next_state);
                robots_skipped_if_waiting.insert(robot);
            }
        }

        if !robots_skipped_if_waiting.contains(&GeodeCracker) {
            states.push(self.state_after_waiting(robots_skipped_if_waiting));
        }

        states
    }

    fn try_buy_robot(&self, robot: Robot, blueprint: &Blueprint) -> Option<SearchState> {
        if self.should_skip_robot(&robot, blueprint) {
            return None;
        }

        let cost = blueprint.cost_of_robot(&robot);
        let new_resources = self.resources.try_pay(cost)?;

        let mut new_state = SearchState {
            resources: new_resources,
            ..self.new_state_one_minute_later()
        };

        new_state.resources.collect_from_robots(&new_state.robots);

        new_state.robots.add(robot);

        Some(new_state)
    }

    fn should_skip_robot(&self, robot: &Robot, blueprint: &Blueprint) -> bool {
        if robot == &Robot::GeodeCracker {
            return false;
        }

        if self.skipped_robots.contains(robot) {
            return true;
        }

        let robots_creating_resource = self.robots.get(robot);

        let robot_resource = Resource::from(robot);
        let resource_in_storage = self.resources.get(&robot_resource);
        let max_cost_of_resource = blueprint.max_cost_of_resource(&robot_resource);

        robots_creating_resource * self.minutes_left + resource_in_storage
            >= max_cost_of_resource * self.minutes_left
    }

    fn new_state_one_minute_later(&self) -> SearchState {
        SearchState {
            minutes_left: self.minutes_left - 1,
            skipped_robots: HashSet::<Robot>::with_capacity(NUMBER_OF_ROBOTS),
            ..self.clone()
        }
    }

    fn state_after_waiting(&self, skipped_robots: HashSet<Robot>) -> SearchState {
        let mut new_state = SearchState {
            skipped_robots,
            ..self.new_state_one_minute_later()
        };

        new_state.resources.collect_from_robots(&new_state.robots);

        new_state
    }
}

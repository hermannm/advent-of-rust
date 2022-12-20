pub struct Blueprint {
    pub id: u32,
    pub ore_robot_cost: OreCost,
    pub clay_robot_cost: OreCost,
    pub obsidian_robot_cost: OreAndClayCost,
    pub geode_robot_cost: OreAndObsidianCost,
}

impl Blueprint {
    pub fn quality_level(&self, minutes_to_open_geodes: u32) -> u32 {
        self.id * self.max_opened_geode_count(minutes_to_open_geodes)
    }

    fn max_opened_geode_count(&self, minutes_to_open_geodes: u32) -> u32 {
        self.recursively_find_max_geodes(
            minutes_to_open_geodes,
            &Resources::new(),
            &Robot::starting_robots(),
        )
    }

    fn recursively_find_max_geodes(
        &self,
        minutes_left: u32,
        resources: &Resources,
        robots: &[Robot],
    ) -> u32 {
        if minutes_left == 0 {
            return resources.geodes;
        }

        let mut max_geodes = 0u32;

        for action in Action::actions_to_test(resources, self) {
            let mut resources = resources.clone();
            let mut robots = robots.to_vec();

            resources.collect_from_robots(&robots);

            if let Action::BuyRobot((robot, cost)) = action {
                resources.pay_cost(&cost);
                robots.push(robot);
            }

            let max_geodes_after_action =
                self.recursively_find_max_geodes(minutes_left - 1, &resources, &robots);

            if max_geodes_after_action > max_geodes {
                max_geodes = max_geodes_after_action;
            }
        }

        max_geodes
    }

    fn cost_of_robot(&self, robot: &Robot) -> Cost {
        use Cost::*;
        use Robot::*;

        match robot {
            OreCollector => Ore(self.ore_robot_cost.clone()),
            ClayCollector => Ore(self.clay_robot_cost.clone()),
            ObsidianCollector => OreAndClay(self.obsidian_robot_cost.clone()),
            GeodeCracker => OreAndObsidian(self.geode_robot_cost.clone()),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct OreCost {
    pub ore: u32,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct OreAndClayCost {
    pub ore: u32,
    pub clay: u32,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct OreAndObsidianCost {
    pub ore: u32,
    pub obsidian: u32,
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum Cost {
    Ore(OreCost),
    OreAndClay(OreAndClayCost),
    OreAndObsidian(OreAndObsidianCost),
}

enum Action {
    BuyRobot((Robot, Cost)),
    Wait,
}

impl Action {
    fn actions_to_test(resources: &Resources, blueprint: &Blueprint) -> Vec<Action> {
        use Robot::*;

        if let Some(buy_geode_cracker) = Action::try_buy_robot(GeodeCracker, resources, blueprint) {
            return vec![buy_geode_cracker];
        }

        let mut actions = vec![Action::Wait];

        if let Some(buy_ore_collector) = Action::try_buy_robot(OreCollector, resources, blueprint) {
            actions.push(buy_ore_collector);
        }

        if let Some(buy_obsidian_collector) =
            Action::try_buy_robot(ObsidianCollector, resources, blueprint)
        {
            actions.push(buy_obsidian_collector);
        } else if let Some(buy_clay_collector) =
            Action::try_buy_robot(ClayCollector, resources, blueprint)
        {
            actions.push(buy_clay_collector);
        }

        actions
    }

    fn try_buy_robot(robot: Robot, resources: &Resources, blueprint: &Blueprint) -> Option<Action> {
        let cost = blueprint.cost_of_robot(&robot);

        if resources.can_afford(&cost) {
            Some(Action::BuyRobot((robot, cost)))
        } else {
            None
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum Robot {
    OreCollector,
    ClayCollector,
    ObsidianCollector,
    GeodeCracker,
}

impl Robot {
    fn starting_robots() -> Vec<Robot> {
        vec![Robot::OreCollector]
    }
}

#[derive(Clone)]
struct Resources {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,
}

impl Resources {
    fn new() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
        }
    }

    fn collect_from_robots(&mut self, robots: &[Robot]) {
        use Robot::*;

        for robot in robots {
            match robot {
                OreCollector => {
                    self.ore += 1;
                }
                ClayCollector => {
                    self.clay += 1;
                }
                ObsidianCollector => {
                    self.obsidian += 1;
                }
                GeodeCracker => {
                    self.geodes += 1;
                }
            }
        }
    }

    fn can_afford(&self, cost: &Cost) -> bool {
        use Cost::*;

        match cost {
            Ore(cost) => self.ore >= cost.ore,
            OreAndClay(cost) => self.ore >= cost.ore && self.clay >= cost.clay,
            OreAndObsidian(cost) => self.ore >= cost.ore && self.obsidian >= cost.obsidian,
        }
    }

    fn with_cost_applied(&self, cost: &Cost) -> Resources {
        let &Resources {
            ore,
            clay,
            obsidian,
            geodes,
        } = self;

        use Cost::*;

        match cost {
            Ore(cost) => Resources {
                ore: ore - cost.ore,
                clay,
                obsidian,
                geodes,
            },
            OreAndClay(cost) => Resources {
                ore: ore - cost.ore,
                clay: clay - cost.clay,
                obsidian,
                geodes,
            },
            OreAndObsidian(cost) => Resources {
                ore: ore - cost.ore,
                clay,
                obsidian: obsidian - cost.obsidian,
                geodes,
            },
        }
    }

    fn pay_cost(&mut self, cost: &Cost) {
        *self = self.with_cost_applied(cost);
    }
}

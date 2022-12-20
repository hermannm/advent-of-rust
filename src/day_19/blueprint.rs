use std::collections::HashSet;

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
            Action::Wait,
            Robot::starting_robots(),
            Resources::new(),
        )
    }

    fn recursively_find_max_geodes(
        &self,
        minutes_left: u32,
        action: Action,
        mut robots: Vec<Robot>,
        mut resources: Resources,
    ) -> u32 {
        if minutes_left == 0 {
            return resources.geodes;
        }

        resources.collect_from_robots(&robots);

        action.execute(&mut robots, &mut resources);

        let mut max_geodes = 0u32;

        for action in Action::all_available_actions(&resources, self) {
            let max_geodes_after_action = self.recursively_find_max_geodes(
                minutes_left - 1,
                action,
                robots.clone(),
                resources.clone(),
            );

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
    BuyRobots(Vec<(Robot, Cost)>),
    Wait,
}

impl Action {
    fn all_available_actions(resources: &Resources, blueprint: &Blueprint) -> Vec<Action> {
        let mut first_robot_purchase_alternatives = HashSet::<Vec<(Robot, Cost)>>::new();

        let (affordable_robots, can_buy_geode_cracker) =
            Robot::all_affordable_or_geode_cracker(resources, blueprint);

        for robot_with_cost in affordable_robots {
            first_robot_purchase_alternatives.insert(vec![robot_with_cost]);
        }

        let mut robot_purchase_alternatives = vec![first_robot_purchase_alternatives];

        loop {
            let mut purchase_alternatives_to_add = HashSet::<Vec<(Robot, Cost)>>::new();

            for purchase_alternative in robot_purchase_alternatives.last().unwrap() {
                let mut resources_after_purchase = resources.clone();

                for (_, cost) in purchase_alternative {
                    resources_after_purchase = resources_after_purchase.with_cost_applied(cost);
                }

                for robot_with_cost in
                    Robot::all_affordable_or_geode_cracker(&resources_after_purchase, blueprint).0
                {
                    let mut new_purchase_alternative = purchase_alternative.clone();
                    new_purchase_alternative.push(robot_with_cost);
                    purchase_alternatives_to_add.insert(new_purchase_alternative);
                }
            }

            if purchase_alternatives_to_add.is_empty() {
                break;
            }

            robot_purchase_alternatives.push(purchase_alternatives_to_add);
        }

        let mut actions = robot_purchase_alternatives
            .iter()
            .flat_map(|purchase_set| purchase_set.iter())
            .map(|robot_purchases| Action::BuyRobots(robot_purchases.clone()))
            .collect::<Vec<Action>>();

        if !can_buy_geode_cracker {
            actions.push(Action::Wait);
        }

        actions
    }

    fn execute(self, robots: &mut Vec<Robot>, resources: &mut Resources) {
        match self {
            Action::Wait => {}
            Action::BuyRobots(robots_with_cost) => {
                for (robot, cost) in robots_with_cost {
                    resources.pay_cost(&cost);
                    robots.push(robot);
                }
            }
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

    /// Returns a list of all robots we can afford with the given resources and blueprint.
    /// If we can afford geode cracker, returns only that, since we always want to buy that.
    /// Also returns a boolean flag for whether we could afford the geode cracker.
    fn all_affordable_or_geode_cracker(
        resources: &Resources,
        blueprint: &Blueprint,
    ) -> (Vec<(Robot, Cost)>, bool) {
        use Robot::*;

        let mut affordable_robots = Vec::<(Robot, Cost)>::new();

        for robot in [OreCollector, ClayCollector, ObsidianCollector, GeodeCracker] {
            let cost = blueprint.cost_of_robot(&robot);

            if resources.can_afford(&cost) {
                if robot == GeodeCracker {
                    return (vec![(robot, cost)], true);
                }

                affordable_robots.push((robot, cost));
            }
        }

        (affordable_robots, false)
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

    fn collect_from_robots(&mut self, robots: &Vec<Robot>) {
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

pub struct Blueprint {
    pub id: u32,
    pub ore_collector_cost: OreCost,
    pub clay_collector_cost: OreCost,
    pub obsidian_collector_cost: OreAndClayCost,
    pub geode_cracker_cost: OreAndObsidianCost,
}

pub struct OreCost {
    pub ore: u16,
}

pub struct OreAndClayCost {
    pub ore: u16,
    pub clay: u16,
}

pub struct OreAndObsidianCost {
    pub ore: u16,
    pub obsidian: u16,
}

impl Blueprint {
    pub fn quality_level(&self, minutes_to_open_geodes: u16) -> u32 {
        self.id * u32::from(self.max_geodes(minutes_to_open_geodes))
    }

    pub fn max_geodes(&self, minutes_to_open_geodes: u16) -> u16 {
        SearchState::new(minutes_to_open_geodes).recursively_find_max_geodes(self)
    }
}

#[derive(Clone)]
struct SearchState {
    minutes_left: u16,
    resources: Resources,
    robots: Robots,
    robots_skipped: RobotsSkipped,
}

impl SearchState {
    fn new(minutes_to_open_geodes: u16) -> SearchState {
        SearchState {
            minutes_left: minutes_to_open_geodes,
            resources: Resources::default(),
            robots: Robots::default(),
            robots_skipped: RobotsSkipped::default(),
        }
    }

    fn recursively_find_max_geodes(&self, blueprint: &Blueprint) -> u16 {
        if self.minutes_left == 0 {
            return self.resources.geodes;
        }

        self.next_states(blueprint)
            .iter()
            .map(|next_state| next_state.recursively_find_max_geodes(blueprint))
            .max()
            .unwrap_or(0)
    }

    fn next_states(&self, blueprint: &Blueprint) -> Vec<SearchState> {
        let mut states = Vec::<SearchState>::new();
        let mut robots_skipped_if_waiting = self.robots_skipped.clone();

        if let Some(geode_cracker_state) = self.try_buy_geode_cracker(blueprint) {
            states.push(geode_cracker_state);
            robots_skipped_if_waiting.geode_cracker |= true;
        }

        if let Some(obsidian_collector_state) = self.try_buy_obsidian_collector(blueprint) {
            states.push(obsidian_collector_state);
            robots_skipped_if_waiting.obsidian_collector |= true;
        }

        if let Some(clay_collector_state) = self.try_buy_clay_collector(blueprint) {
            states.push(clay_collector_state);
            robots_skipped_if_waiting.clay_collector |= true;
        }

        if let Some(ore_collector_state) = self.try_buy_ore_collector(blueprint) {
            states.push(ore_collector_state);
            robots_skipped_if_waiting.ore_collector |= true;
        }

        if !robots_skipped_if_waiting.geode_cracker {
            states.push(self.state_after_waiting(robots_skipped_if_waiting));
        }

        states
    }

    fn new_state_one_minute_later(&self) -> SearchState {
        SearchState {
            minutes_left: self.minutes_left - 1,
            resources: self.resources.clone(),
            robots: self.robots.clone(),
            robots_skipped: RobotsSkipped::default(),
        }
    }

    fn state_after_waiting(&self, robots_skipped: RobotsSkipped) -> SearchState {
        let mut state = self.new_state_one_minute_later();
        state.resources.collect_from_robots(&state.robots);
        state.robots_skipped = robots_skipped;
        state
    }

    fn try_buy_geode_cracker(&self, blueprint: &Blueprint) -> Option<SearchState> {
        let new_ore = self
            .resources
            .ore
            .checked_sub(blueprint.geode_cracker_cost.ore)?;

        let new_obsidian = self
            .resources
            .obsidian
            .checked_sub(blueprint.geode_cracker_cost.obsidian)?;

        let mut state = self.new_state_one_minute_later();

        state.resources.ore = new_ore;
        state.resources.obsidian = new_obsidian;
        state.resources.collect_from_robots(&state.robots);

        state.robots.geode_crackers += 1;

        Some(state)
    }

    fn try_buy_obsidian_collector(&self, blueprint: &Blueprint) -> Option<SearchState> {
        if self.robots_skipped.obsidian_collector
            || self.should_skip_robot(
                self.robots.obsidian_collectors,
                self.resources.obsidian,
                blueprint.geode_cracker_cost.obsidian,
            )
        {
            return None;
        }

        let new_ore = self
            .resources
            .ore
            .checked_sub(blueprint.obsidian_collector_cost.ore)?;

        let new_clay = self
            .resources
            .clay
            .checked_sub(blueprint.obsidian_collector_cost.clay)?;

        let mut state = self.new_state_one_minute_later();

        state.resources.ore = new_ore;
        state.resources.clay = new_clay;
        state.resources.collect_from_robots(&state.robots);

        state.robots.obsidian_collectors += 1;

        Some(state)
    }

    fn try_buy_clay_collector(&self, blueprint: &Blueprint) -> Option<SearchState> {
        if self.robots_skipped.clay_collector
            || self.should_skip_robot(
                self.robots.clay_collectors,
                self.resources.clay,
                blueprint.obsidian_collector_cost.clay,
            )
        {
            return None;
        }

        let new_ore = self
            .resources
            .ore
            .checked_sub(blueprint.clay_collector_cost.ore)?;

        let mut state = self.new_state_one_minute_later();

        state.resources.ore = new_ore;
        state.resources.collect_from_robots(&state.robots);

        state.robots.clay_collectors += 1;

        Some(state)
    }

    fn try_buy_ore_collector(&self, blueprint: &Blueprint) -> Option<SearchState> {
        if self.robots_skipped.ore_collector
            || self.should_skip_robot(
                self.robots.ore_collectors,
                self.resources.ore,
                blueprint
                    .ore_collector_cost
                    .ore
                    .max(blueprint.clay_collector_cost.ore)
                    .max(blueprint.obsidian_collector_cost.ore)
                    .max(blueprint.geode_cracker_cost.ore),
            )
        {
            return None;
        }

        let new_ore = self
            .resources
            .ore
            .checked_sub(blueprint.ore_collector_cost.ore)?;

        let mut state = self.new_state_one_minute_later();

        state.resources.ore = new_ore;
        state.resources.collect_from_robots(&state.robots);

        state.robots.ore_collectors += 1;

        Some(state)
    }

    fn should_skip_robot(
        &self,
        robots_creating_resource: u16,
        resource_in_storage: u16,
        max_cost_of_resource: u16,
    ) -> bool {
        robots_creating_resource * self.minutes_left + resource_in_storage
            >= max_cost_of_resource * self.minutes_left
    }
}

#[derive(Default, Clone)]
struct Resources {
    ore: u16,
    clay: u16,
    obsidian: u16,
    geodes: u16,
}

impl Resources {
    fn collect_from_robots(&mut self, robots: &Robots) {
        self.ore += robots.ore_collectors;
        self.clay += robots.clay_collectors;
        self.obsidian += robots.obsidian_collectors;
        self.geodes += robots.geode_crackers;
    }
}

#[derive(Clone)]
struct Robots {
    ore_collectors: u16,
    clay_collectors: u16,
    obsidian_collectors: u16,
    geode_crackers: u16,
}

impl Default for Robots {
    fn default() -> Self {
        Self {
            ore_collectors: 1,
            clay_collectors: 0,
            obsidian_collectors: 0,
            geode_crackers: 0,
        }
    }
}

#[derive(Default, Clone)]
struct RobotsSkipped {
    ore_collector: bool,
    clay_collector: bool,
    obsidian_collector: bool,
    geode_cracker: bool,
}

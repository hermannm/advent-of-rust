use std::collections::HashMap;

pub struct ValveMap(pub HashMap<String, Valve>);

pub struct Valve {
    pub flow_rate: u32,
    pub connected_valves: Vec<String>,
}

impl ValveMap {
    pub fn release_maximum_pressure(
        &self,
        in_minutes: u32,
        starting_valve_id: &str,
    ) -> Result<u32, String> {
        let shortest_paths_between_valves =
            self.shortest_paths_between_valves(starting_valve_id)?;

        self.find_releasable_pressure(
            &String::from(starting_valve_id),
            in_minutes,
            0u32,
            Vec::<&String>::new(),
            &shortest_paths_between_valves,
        )
        .ok_or_else(|| String::from("Failed to find releasable pressure from valves"))
    }

    fn find_releasable_pressure(
        &self,
        current_valve_id: &String,
        minutes_left: u32,
        released_pressure: u32,
        opened_valves: Vec<&String>,
        shortest_paths_between_valves: &HashMap<String, HashMap<String, u32>>,
    ) -> Option<u32> {
        let paths_from_current_valve = shortest_paths_between_valves.get(current_valve_id)?;

        let other_valves_releasable_pressure = paths_from_current_valve
            .iter()
            .filter(|(_, &minutes_required)| minutes_required < minutes_left)
            .flat_map(|(valve_id, &minutes_required)| {
                let flow_rate = if opened_valves.contains(&valve_id) {
                    0u32
                } else {
                    let valve = self.0.get(valve_id)?;
                    valve.flow_rate
                };

                let new_minutes_left = minutes_left - minutes_required;
                let releasable_pressure = released_pressure + flow_rate * new_minutes_left;

                let mut new_opened_valves = opened_valves.clone();
                new_opened_valves.push(valve_id);

                self.find_releasable_pressure(
                    valve_id,
                    new_minutes_left,
                    releasable_pressure,
                    new_opened_valves,
                    shortest_paths_between_valves,
                )
            })
            .max();

        if let Some(other_valves_releasable_pressure) = other_valves_releasable_pressure {
            if other_valves_releasable_pressure > released_pressure {
                return Some(other_valves_releasable_pressure);
            }
        }

        Some(released_pressure)
    }
}

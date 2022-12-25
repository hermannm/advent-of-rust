use std::collections::{HashMap, HashSet};

pub(super) struct ValveMap(pub HashMap<String, Valve>);

pub(super) struct Valve {
    pub flow_rate: u32,
    pub connected_valves: Vec<String>,
}

impl ValveMap {
    pub fn release_maximum_pressure(
        &self,
        in_minutes: u32,
        starting_valve_id: &String,
    ) -> Result<u32, String> {
        let shortest_paths_between_valves =
            self.shortest_paths_between_valves(starting_valve_id)?;

        self.find_releasable_pressure(
            starting_valve_id,
            in_minutes,
            0u32,
            Vec::<&String>::new(),
            &mut HashMap::<Vec<&String>, u32>::new(),
            &shortest_paths_between_valves,
        )
    }

    pub fn release_maximum_pressure_with_elephant(
        &self,
        in_minutes: u32,
        starting_valve_id: &String,
    ) -> Result<u32, String> {
        let mut opened_valves_pressure_map = HashMap::<Vec<&String>, u32>::new();

        let shortest_paths_between_valves =
            self.shortest_paths_between_valves(starting_valve_id)?;

        self.find_releasable_pressure(
            starting_valve_id,
            in_minutes,
            0u32,
            Vec::<&String>::new(),
            &mut opened_valves_pressure_map,
            &shortest_paths_between_valves,
        )?;

        let mut opened_valves_pressure_vec = opened_valves_pressure_map
            .into_iter()
            .collect::<Vec<(Vec<&String>, u32)>>();

        opened_valves_pressure_vec.sort_by(|(_, released_pressure_1), (_, released_pressure_2)| {
            released_pressure_2.cmp(released_pressure_1)
        });

        let mut max_pressure = 0u32;

        for (index_1, (opened_valves_1, released_pressure_1)) in
            opened_valves_pressure_vec.iter().enumerate()
        {
            let opened_valves_set_1 = HashSet::<&&String>::from_iter(opened_valves_1.iter());

            for (opened_valves_2, released_pressure_2) in
                opened_valves_pressure_vec.iter().skip(index_1 + 1)
            {
                let pressure_sum = released_pressure_1 + released_pressure_2;

                if pressure_sum <= max_pressure {
                    break;
                }

                let opened_valves_set_2 = HashSet::<&&String>::from_iter(opened_valves_2.iter());

                if opened_valves_set_1
                    .intersection(&opened_valves_set_2)
                    .count()
                    == 0
                    && pressure_sum > max_pressure
                {
                    max_pressure = pressure_sum;
                }
            }
        }

        Ok(max_pressure)
    }

    fn find_releasable_pressure<'a: 'b, 'b>(
        &self,
        current_valve_id: &'a String,
        minutes_left: u32,
        released_pressure: u32,
        opened_valves: Vec<&'b String>,
        opened_valves_pressure_map: &mut HashMap<Vec<&'b String>, u32>,
        shortest_paths_between_valves: &'a HashMap<String, HashMap<String, u32>>,
    ) -> Result<u32, String> {
        let paths_from_current_valve = shortest_paths_between_valves
            .get(current_valve_id)
            .ok_or_else(|| {
                format!("Failed to find shortest paths for valve with ID '{current_valve_id}'")
            })?;

        let other_valves_releasable_pressure = paths_from_current_valve
            .iter()
            .filter(|(_, &minutes_required)| minutes_required < minutes_left)
            .flat_map(|(valve_id, &minutes_required)| {
                let flow_rate = if opened_valves.contains(&valve_id) {
                    0u32
                } else {
                    let valve = self
                        .0
                        .get(valve_id)
                        .ok_or_else(|| format!("Failed to find valve with ID '{valve_id}'"))?;

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
                    opened_valves_pressure_map,
                    shortest_paths_between_valves,
                )
            })
            .max();

        if let Some(other_valves_releasable_pressure) = other_valves_releasable_pressure {
            if other_valves_releasable_pressure > released_pressure {
                return Ok(other_valves_releasable_pressure);
            }
        }

        opened_valves_pressure_map.insert(opened_valves, released_pressure);

        Ok(released_pressure)
    }
}

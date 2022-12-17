use std::collections::HashMap;

use super::valves::{Valve, ValveMap};

impl TryFrom<&str> for ValveMap {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut valve_map = HashMap::<String, Valve>::new();

        for line in input.lines() {
            let (_, rest_of_line) = line
                .split_once("Valve ")
                .ok_or_else(|| format!("Expected to find 'Valve' in input line '{line}'"))?;

            let (valve_id_string, rest_of_line) =
                rest_of_line.split_once(" has flow rate=").ok_or_else(|| {
                    format!("Expected to find 'has flow rate' in input line '{line}'")
                })?;

            let valve_id = String::from(valve_id_string);
            let valve = Valve::try_from(rest_of_line)?;

            valve_map.insert(valve_id, valve);
        }

        Ok(ValveMap(valve_map))
    }
}

impl TryFrom<&str> for Valve {
    type Error = String;

    fn try_from(line_starting_with_flow_rate: &str) -> Result<Self, Self::Error> {
        let (flow_rate_string, tunnels_string) = line_starting_with_flow_rate
            .split_once("; ")
            .ok_or_else(|| {
                format!("Expected to find ';' in input line '{line_starting_with_flow_rate}'")
            })?;

        let flow_rate = flow_rate_string.parse::<u32>().map_err(|_| {
            format!("Failed to parse flow rate input '{flow_rate_string}' to integer")
        })?;

        let connected_valves = {
            if let Some((_, connected_valves_string)) =
                tunnels_string.split_once("tunnels lead to valves ")
            {
                connected_valves_string
                    .split(", ")
                    .map(String::from)
                    .collect::<Vec<String>>()
            } else if let Some((_, connected_valve_string)) =
                tunnels_string.split_once("tunnel leads to valve ")
            {
                vec![String::from(connected_valve_string)]
            } else {
                return Err(format!(
                    "Failed to parse connected valves from input string '{tunnels_string}'"
                ));
            }
        };

        Ok(Valve {
            flow_rate,
            connected_valves,
        })
    }
}

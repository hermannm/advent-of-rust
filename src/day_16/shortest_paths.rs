use std::{collections::HashMap, ops::Index};

use petgraph::{
    algo::floyd_warshall,
    prelude::{Graph, NodeIndex},
    Directed,
};

use super::valves::ValveMap;

#[derive(PartialEq, Eq, Debug)]
enum ValveGraphNode {
    Valve(String),
    Connector(String),
}

impl ValveMap {
    pub fn shortest_paths_between_valves(
        &self,
    ) -> Result<HashMap<String, HashMap<String, u32>>, String> {
        let graph = self.to_graph()?;

        let result = floyd_warshall(&graph, |edge| *edge.weight()).map_err(|_| {
            String::from("Failed to run Floyd-Warshall algorithm: Negative cycle detected")
        })?;

        let mut shortest_paths = HashMap::<String, HashMap<String, u32>>::new();

        for (node_indices, shortest_path_length) in result {
            let nodes = (graph.index(node_indices.0), graph.index(node_indices.1));

            use ValveGraphNode::*;

            if let (Valve(valve_1), Valve(valve_2)) = nodes {
                let valve_1_paths = shortest_paths
                    .entry(valve_1.clone())
                    .or_insert_with(HashMap::<String, u32>::new);

                valve_1_paths.insert(valve_2.clone(), shortest_path_length);

                let valve_2_paths = shortest_paths
                    .entry(valve_2.clone())
                    .or_insert_with(HashMap::<String, u32>::new);

                valve_2_paths.insert(valve_1.clone(), shortest_path_length);
            }
        }

        Ok(shortest_paths)
    }

    fn to_graph(&self) -> Result<Graph<ValveGraphNode, u32, Directed>, String> {
        use ValveGraphNode::*;

        struct ValveIndices {
            valve_index: NodeIndex,
            connector_index: NodeIndex,
        }

        let mut valve_indices = HashMap::<String, ValveIndices>::new();
        let mut graph = Graph::new();

        for valve_id in self.0.keys() {
            let valve_index = graph.add_node(Valve(valve_id.clone()));
            let connector_index = graph.add_node(Connector(valve_id.clone()));

            valve_indices.insert(
                valve_id.clone(),
                ValveIndices {
                    valve_index,
                    connector_index,
                },
            );
        }

        for (valve_id, valve) in &self.0 {
            let &ValveIndices {
                valve_index,
                connector_index,
            } = valve_indices
                .get(valve_id)
                .ok_or_else(|| format!("Failed to get graph indices for valve ID '{valve_id}'"))?;

            graph.add_edge(connector_index, valve_index, 1u32);
            graph.add_edge(valve_index, connector_index, 0u32);

            for connected_valve in &valve.connected_valves {
                let &ValveIndices {
                    connector_index: connected_index,
                    ..
                } = valve_indices.get(connected_valve).ok_or_else(|| {
                    format!("Failed to get graph indices for valve ID '{connected_valve}'")
                })?;

                graph.add_edge(connector_index, connected_index, 1u32);
            }
        }

        Ok(graph)
    }
}

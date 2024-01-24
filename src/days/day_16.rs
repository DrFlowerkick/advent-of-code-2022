//!day_16.rs

use anyhow::Result;
use petgraph::algo::floyd_warshall;
use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::HashMap;

struct ValveNetwork<'a> {
    valves: UnGraph<(&'a str, u32), u32>,
    initial_node_id: NodeIndex<u32>,
    pair_distance: HashMap<(NodeIndex<u32>, NodeIndex<u32>), u32>,
}

impl<'a> From<&'a str> for ValveNetwork<'a> {
    fn from(value: &'a str) -> Self {
        let base_capacity = value.lines().count();
        let mut valves: UnGraph<(&str, u32), u32> =
            UnGraph::with_capacity(base_capacity, base_capacity);
        let mut labels: HashMap<&str, NodeIndex<u32>> = HashMap::new();
        // add nodes
        for line in value.lines() {
            let mut key_word_iter = line.split_whitespace();
            let valve_label = key_word_iter.nth(1).unwrap();
            let valve_value = key_word_iter
                .nth(2)
                .map(|v| {
                    v.strip_prefix("rate=")
                        .unwrap()
                        .strip_suffix(';')
                        .unwrap()
                        .parse::<u32>()
                        .expect("bad input")
                })
                .unwrap();
            let node_id = valves.add_node((valve_label, valve_value));
            labels.insert(valve_label, node_id);
        }
        // add edges
        let mut count_edge_update = 0;
        for line in value.lines() {
            let mut key_word_iter = line.split_whitespace();
            let valve_label = key_word_iter.nth(1).unwrap();
            let valve_id: &NodeIndex<u32> = labels.get(valve_label).unwrap();
            for edge_valve in key_word_iter.skip(7).map(|e| &e[0..2]) {
                let edge_valve_id: &NodeIndex<u32> = labels.get(edge_valve).unwrap();
                valves.update_edge(*valve_id, *edge_valve_id, 1);
                count_edge_update += 1;
            }
        }
        assert_eq!(valves.edge_count(), count_edge_update / 2);
        let pair_distance = floyd_warshall(&valves, |e| *e.weight()).expect("bad floyd warshall");
        ValveNetwork {
            valves,
            initial_node_id: *labels.get("AA").unwrap(),
            pair_distance,
        }
    }
}

impl<'a> ValveNetwork<'a> {
    fn iter_pair_distance(
        &self,
        node: NodeIndex<u32>,
        remaining_minutes: u32,
        minimum_valve_value: u32,
    ) -> impl Iterator<Item = (NodeIndex<u32>, u32)> + '_ {
        self.pair_distance
            .iter()
            .filter(move |((n1, n2), d)| {
                **d > 0 && **d < remaining_minutes - 1 && (*n1 == node || *n2 == node)
            })
            .filter_map(move |((n1, n2), d)| {
                let next_node = if node == *n1 { *n2 } else { *n1 };
                if self.valves.node_weight(next_node).unwrap().1 >= minimum_valve_value {
                    Some((next_node, *d))
                } else {
                    None
                }
            })
    }
    fn best_pressure_release(&self, minimum_valve_value: u32) -> u32 {
        let minutes: u32 = 30;
        let mut seen_nodes: Vec<NodeIndex<u32>> = Vec::with_capacity(self.valves.node_count());
        let mut max_pressure = 0;
        for (next_node, distance) in
            self.iter_pair_distance(self.initial_node_id, minutes, minimum_valve_value)
        {
            max_pressure = max_pressure.max(self.pressure_release_recursive(
                next_node,
                minutes - distance,
                &mut seen_nodes,
                minimum_valve_value,
            ));
        }
        max_pressure
    }
    fn pressure_release_recursive(
        &self,
        current_node: NodeIndex<u32>,
        mut remaining_minutes: u32,
        seen_nodes: &mut Vec<NodeIndex<u32>>,
        minimum_valve_value: u32,
    ) -> u32 {
        remaining_minutes -= 1;
        let current_pressure = self.valves.node_weight(current_node).unwrap().1 * remaining_minutes;
        if remaining_minutes <= 1 {
            return current_pressure;
        }
        seen_nodes.push(current_node);
        let mut max_next_pressure = 0;
        for (next_node, distance) in
            self.iter_pair_distance(current_node, remaining_minutes, minimum_valve_value)
        {
            if !seen_nodes.contains(&next_node) {
                max_next_pressure = max_next_pressure.max(self.pressure_release_recursive(
                    next_node,
                    remaining_minutes - distance,
                    seen_nodes,
                    minimum_valve_value,
                ));
            }
        }
        seen_nodes.pop();
        current_pressure + max_next_pressure
    }
    fn best_pressure_release_pair_working(&self, minimum_valve_value: u32) -> u32 {
        let minutes: u32 = 26;
        let mut seen_nodes: Vec<NodeIndex<u32>> = Vec::with_capacity(self.valves.node_count());
        let mut max_pressure = 0;
        for (i, (my_next_node, my_distance)) in self
            .iter_pair_distance(self.initial_node_id, minutes, minimum_valve_value)
            .enumerate()
        {
            for (elephant_next_node, elephant_distance) in self
                .iter_pair_distance(self.initial_node_id, minutes, minimum_valve_value)
                .skip(i + 1)
            {
                max_pressure = max_pressure.max(self.pressure_release_pair_working_recursive(
                    my_next_node,
                    elephant_next_node,
                    minutes - my_distance,
                    minutes - elephant_distance,
                    &mut seen_nodes,
                    minimum_valve_value,
                ));
            }
        }
        max_pressure
    }
    fn pressure_release_pair_working_recursive(
        &self,
        my_node: NodeIndex<u32>,
        elephant_node: NodeIndex<u32>,
        mut my_remaining_minutes: u32,
        mut elephant_remaining_minutes: u32,
        seen_nodes: &mut Vec<NodeIndex<u32>>,
        minimum_valve_value: u32,
    ) -> u32 {
        let my_pressure_release = if !seen_nodes.contains(&my_node) {
            my_remaining_minutes -= 1;
            self.valves.node_weight(my_node).unwrap().1 * my_remaining_minutes
        } else {
            0
        };
        let elephant_pressure_release = if !seen_nodes.contains(&elephant_node) {
            elephant_remaining_minutes -= 1;
            self.valves.node_weight(elephant_node).unwrap().1 * elephant_remaining_minutes
        } else {
            0
        };
        seen_nodes.push(my_node);
        seen_nodes.push(elephant_node);
        let mut max_next_pressure = 0;
        match (my_remaining_minutes > 1, elephant_remaining_minutes > 1) {
            (false, false) => (),
            (true, false) => {
                for (next_node, distance) in
                    self.iter_pair_distance(my_node, my_remaining_minutes, minimum_valve_value)
                {
                    if !seen_nodes.contains(&next_node) {
                        max_next_pressure =
                            max_next_pressure.max(self.pressure_release_pair_working_recursive(
                                next_node,
                                elephant_node,
                                my_remaining_minutes - distance,
                                elephant_remaining_minutes,
                                seen_nodes,
                                minimum_valve_value,
                            ));
                    }
                }
            }
            (false, true) => {
                for (next_node, distance) in self.iter_pair_distance(
                    elephant_node,
                    elephant_remaining_minutes,
                    minimum_valve_value,
                ) {
                    if !seen_nodes.contains(&next_node) {
                        max_next_pressure =
                            max_next_pressure.max(self.pressure_release_pair_working_recursive(
                                my_node,
                                next_node,
                                my_remaining_minutes,
                                elephant_remaining_minutes - distance,
                                seen_nodes,
                                minimum_valve_value,
                            ));
                    }
                }
            }
            (true, true) => {
                for (my_next_node, my_distance) in
                    self.iter_pair_distance(my_node, my_remaining_minutes, minimum_valve_value)
                {
                    if !seen_nodes.contains(&my_next_node) {
                        for (elephant_next_node, elephant_distance) in self.iter_pair_distance(
                            elephant_node,
                            elephant_remaining_minutes,
                            minimum_valve_value,
                        ) {
                            if !seen_nodes.contains(&elephant_next_node)
                                && elephant_next_node != my_next_node
                            {
                                max_next_pressure = max_next_pressure.max(
                                    self.pressure_release_pair_working_recursive(
                                        my_next_node,
                                        elephant_next_node,
                                        my_remaining_minutes - my_distance,
                                        elephant_remaining_minutes - elephant_distance,
                                        seen_nodes,
                                        minimum_valve_value,
                                    ),
                                );
                            }
                        }
                    }
                }
            }
        }
        seen_nodes.pop();
        seen_nodes.pop();
        my_pressure_release + elephant_pressure_release + max_next_pressure
    }
}

pub fn day_16() -> Result<()> {
    let input = include_str!("../../assets/day_16.txt");
    let valve_network = ValveNetwork::from(input);
    let minimum_valve_value = 3;
    let result_part1 = valve_network.best_pressure_release(minimum_valve_value);
    println!("result day 15 part 1: {}", result_part1);
    assert_eq!(result_part1, 2_077);

    //let result_part2 = 0;
    //println!("result day 15 part 2: {}", result_part2);
    //assert_eq!(result_part2, 13_172_087_230_812);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = include_str!("../../assets/day_16_example.txt");
        let valve_network = ValveNetwork::from(input);
        //eprintln!("valves");
        //eprintln!("{:?}", valve_network.valves);
        //eprintln!("pair_distance");
        //eprintln!("{:?}", valve_network.pair_distance);
        let minimum_valve_value = 2;
        let result_part1 = valve_network.best_pressure_release(minimum_valve_value);
        println!("result example day 16 part 1: {}", result_part1);
        assert_eq!(result_part1, 1_651);

        let result_part2 = valve_network.best_pressure_release_pair_working(1);
        println!("result example day 16 part 2: {}", result_part2);
        assert_eq!(result_part2, 1_707);
        Ok(())
    }
}

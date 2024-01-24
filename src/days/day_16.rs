//!day_16.rs

use anyhow::Result;
use petgraph::algo::floyd_warshall;
use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::HashMap;

struct ValveNetwork {
    valves: UnGraph<u32, u32>,
    initial_node_id: NodeIndex<u32>,
    pair_distance: HashMap<(NodeIndex<u32>, NodeIndex<u32>), u32>,
}

impl From<&str> for ValveNetwork {
    fn from(value: &str) -> Self {
        let base_capacity = value.lines().count();
        let mut valves: UnGraph<u32, u32> = UnGraph::with_capacity(base_capacity, base_capacity);
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
            let node_id = valves.add_node(valve_value);
            labels.insert(valve_label, node_id);
        }
        let initial_node_id = *labels.get("AA").unwrap();
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
        let pair_distance: HashMap<(NodeIndex<u32>, NodeIndex<u32>), u32> =
            floyd_warshall(&valves, |e| *e.weight())
                .expect("bad floyd warshall")
                .iter()
                .filter(|((n1, n2), _)| {
                    n1 != n2
                        && (*n1 == initial_node_id || *valves.node_weight(*n1).unwrap() > 0)
                        && *valves.node_weight(*n2).unwrap() > 0
                })
                .map(|((n1, n2), d)| ((*n1, *n2), *d))
                .collect();
        ValveNetwork {
            valves,
            initial_node_id,
            pair_distance,
        }
    }
}

impl ValveNetwork {
    fn iter_pair_distance(
        &self,
        node: NodeIndex<u32>,
        remaining_minutes: u32,
        minimum_valve_value: u32,
    ) -> impl Iterator<Item = (NodeIndex<u32>, u32)> + '_ {
        self.pair_distance
            .iter()
            .filter(move |((n1, n2), d)| {
                **d < remaining_minutes - 1
                    && *n1 == node
                    && *self.valves.node_weight(*n2).unwrap() >= minimum_valve_value
            })
            .map(|((_, n2), d)| (*n2, *d))
    }
    fn best_pressure_release(&self, minutes: u32, minimum_valve_value: u32) -> HashMap<u64, u32> {
        let mut pressure_hash: HashMap<u64, u32> = HashMap::new();
        for (next_node, distance) in
            self.iter_pair_distance(self.initial_node_id, minutes, minimum_valve_value)
        {
            self.pressure_release_recursive(
                next_node,
                minutes - distance,
                0,
                0,
                &mut pressure_hash,
                minimum_valve_value,
            );
        }
        pressure_hash
    }
    fn pressure_release_recursive(
        &self,
        current_node: NodeIndex<u32>,
        mut remaining_minutes: u32,
        mut bit_mask: u64,
        mut pressure: u32,
        pressure_hash: &mut HashMap<u64, u32>,
        minimum_valve_value: u32,
    ) {
        remaining_minutes -= 1;
        pressure += self.valves.node_weight(current_node).unwrap() * remaining_minutes;
        bit_mask += 2_u64.pow(current_node.index() as u32);
        if let Some(p) = pressure_hash.get_mut(&bit_mask) {
            *p = pressure.max(*p);
        } else {
            pressure_hash.insert(bit_mask, pressure);
        }
        if remaining_minutes <= 1 {
            return;
        }
        for (next_node, distance) in
            self.iter_pair_distance(current_node, remaining_minutes, minimum_valve_value)
        {
            if 2_u64.pow(next_node.index() as u32) & bit_mask == 0 {
                self.pressure_release_recursive(
                    next_node,
                    remaining_minutes - distance,
                    bit_mask,
                    pressure,
                    pressure_hash,
                    minimum_valve_value,
                );
            }
        }
    }
    fn best_pressure_release_pair_working(&self, minimum_valve_value: u32) -> u32 {
        let minutes: u32 = 26;
        let pressure_hash = self.best_pressure_release(minutes, minimum_valve_value);
        let mut max_pressure = 0;
        for (i, (bm_1, p1)) in pressure_hash.iter().enumerate() {
            for (bm_2, p2) in pressure_hash.iter().skip(i + 1) {
                if bm_1 & bm_2 == 0 {
                    max_pressure = max_pressure.max(*p1 + *p2);
                }
            }
        }
        max_pressure
    }
}

pub fn day_16() -> Result<()> {
    let input = include_str!("../../assets/day_16.txt");
    let valve_network = ValveNetwork::from(input);
    eprintln!("number of nodes: {}", valve_network.valves.node_count());
    let minutes = 30;
    let minimum_valve_value = 3;
    let result_part1 = *valve_network
        .best_pressure_release(minutes, minimum_valve_value)
        .values()
        .max()
        .unwrap();
    println!("result day 16 part 1: {}", result_part1);
    assert_eq!(result_part1, 2_077);

    let minimum_valve_value = 1;
    let result_part2 = valve_network.best_pressure_release_pair_working(minimum_valve_value);
    println!("result day 16 part 2: {}", result_part2);
    assert_eq!(result_part2, 2_741);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn text_bitmask() {
        let input = include_str!("../../assets/day_16_example.txt");
        let valve_network = ValveNetwork::from(input);
        let node_count = valve_network.valves.node_count();
        let mut bit_mask: u64 = 0;
        for node in valve_network.valves.node_indices() {
            bit_mask += 2_u64.pow(node.index() as u32);
        }
        assert_eq!(bit_mask, 2_u64.pow(node_count as u32) - 1);
        assert!(bit_mask & 2_u64.pow(valve_network.initial_node_id.index() as u32) > 0);
        assert!(bit_mask & 2_u64.pow(node_count as u32) == 0);
    }

    #[test]
    fn test_example() -> Result<()> {
        let input = include_str!("../../assets/day_16_example.txt");
        let valve_network = ValveNetwork::from(input);
        let minutes = 30;
        let minimum_valve_value = 2;
        let result_part1 = *valve_network
            .best_pressure_release(minutes, minimum_valve_value)
            .values()
            .max()
            .unwrap();
        println!("result example day 16 part 1: {}", result_part1);
        assert_eq!(result_part1, 1_651);

        let result_part2 = valve_network.best_pressure_release_pair_working(1);
        println!("result example day 16 part 2: {}", result_part2);
        assert_eq!(result_part2, 1_707);
        Ok(())
    }
}

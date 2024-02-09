//!day_19.rs

use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct BluePrint {
    id: u64,
    ore_robot_ore: u64,
    clay_robot_ore: u64,
    obsidian_robot_ore: u64,
    obsidian_robot_clay: u64,
    geode_robot_ore: u64,
    geode_robot_obsidian: u64,
    max_ore_required: u64,
}

impl From<&str> for BluePrint {
    fn from(value: &str) -> Self {
        let mut v_iter = value
            .split_ascii_whitespace()
            .filter_map(|s| s.strip_suffix(':').or(Some(s)))
            .filter_map(|s| s.parse::<u64>().ok());
        let mut bp = Self {
            id: v_iter.next().unwrap(),
            ore_robot_ore: v_iter.next().unwrap(),
            clay_robot_ore: v_iter.next().unwrap(),
            obsidian_robot_ore: v_iter.next().unwrap(),
            obsidian_robot_clay: v_iter.next().unwrap(),
            geode_robot_ore: v_iter.next().unwrap(),
            geode_robot_obsidian: v_iter.next().unwrap(),
            max_ore_required: 0,
        };
        assert!(v_iter.next().is_none());
        bp.max_ore_required = bp
            .ore_robot_ore
            .max(bp.clay_robot_ore)
            .max(bp.obsidian_robot_ore)
            .max(bp.geode_robot_ore);
        bp
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct GeodeCollection {
    n_ore_robots: u64,
    ore: u64,
    n_clay_robots: u64,
    clay: u64,
    n_obsidian_robots: u64,
    obsidian: u64,
    n_geode_robots: u64,
    geodes: u64,
    blue_print: BluePrint,
}

impl GeodeCollection {
    fn new(blue_print: BluePrint) -> Self {
        Self {
            n_ore_robots: 1,
            blue_print,
            ..Default::default()
        }
    }
    fn actions(&self, minutes: u64) -> Vec<(Self, u64)> {
        let mut actions: Vec<(Self, u64)> = Vec::with_capacity(4);
        // build geode robot if possible
        if let Some(geode_robot) = self.build_geode_robot_and_collect(minutes) {
            actions.push(geode_robot);
        }
        // build obsidian robot if possible
        if let Some(obsidian_robot) = self.build_obsidian_robot_and_collect(minutes) {
            actions.push(obsidian_robot);
        }
        // build clay robot if possible
        if let Some(clay_robot) = self.build_clay_robot_and_collect(minutes) {
            actions.push(clay_robot);
        }
        // build ore robot if possible
        if let Some(ore_robot) = self.build_ore_robot_and_collect(minutes) {
            actions.push(ore_robot);
        }
        actions
    }
    fn collect(&self, minutes: u64) -> Self {
        let mut collect = *self;
        collect.ore += collect.n_ore_robots * minutes;
        collect.clay += collect.n_clay_robots * minutes;
        collect.obsidian += collect.n_obsidian_robots * minutes;
        collect.geodes += collect.n_geode_robots * minutes;
        collect
    }
    fn clear_excess_ressources(&mut self, minutes: u64) {
        self.ore = self.ore.min(self.blue_print.max_ore_required * minutes);
        self.clay = self.clay.min(self.blue_print.obsidian_robot_clay * minutes);
        self.obsidian = self
            .obsidian
            .min(self.blue_print.geode_robot_obsidian * minutes);
    }
    fn build_ore_robot_and_collect(&self, minutes: u64) -> Option<(Self, u64)> {
        if self.n_ore_robots >= self.blue_print.max_ore_required {
            return None;
        }
        let production_minutes = if self.ore >= self.blue_print.ore_robot_ore {
            1
        } else {
            1 + (self.blue_print.ore_robot_ore - self.ore).div_ceil(self.n_ore_robots)
        };
        if production_minutes >= minutes {
            return None;
        }
        let mut ore_robot = self.collect(production_minutes);
        ore_robot.n_ore_robots += 1;
        ore_robot.ore -= ore_robot.blue_print.ore_robot_ore;
        let remaining_minutes = minutes - production_minutes;
        ore_robot.clear_excess_ressources(remaining_minutes);
        Some((ore_robot, remaining_minutes))
    }
    fn build_clay_robot_and_collect(&self, minutes: u64) -> Option<(Self, u64)> {
        if self.n_clay_robots >= self.blue_print.obsidian_robot_clay {
            return None;
        }
        let production_minutes = if self.ore >= self.blue_print.clay_robot_ore {
            1
        } else {
            1 + (self.blue_print.clay_robot_ore - self.ore).div_ceil(self.n_ore_robots)
        };
        if production_minutes >= minutes {
            return None;
        }
        let mut clay_robot = self.collect(production_minutes);
        clay_robot.n_clay_robots += 1;
        clay_robot.ore -= clay_robot.blue_print.clay_robot_ore;
        let remaining_minutes = minutes - production_minutes;
        clay_robot.clear_excess_ressources(remaining_minutes);
        Some((clay_robot, remaining_minutes))
    }
    fn build_obsidian_robot_and_collect(&self, minutes: u64) -> Option<(Self, u64)> {
        if self.n_clay_robots == 0 || self.n_obsidian_robots >= self.blue_print.geode_robot_obsidian
        {
            return None;
        }
        let ore_minutes = if self.ore >= self.blue_print.obsidian_robot_ore {
            1
        } else {
            1 + (self.blue_print.obsidian_robot_ore - self.ore).div_ceil(self.n_ore_robots)
        };
        let clay_minutes = if self.clay >= self.blue_print.obsidian_robot_clay {
            1
        } else {
            1 + (self.blue_print.obsidian_robot_clay - self.clay).div_ceil(self.n_clay_robots)
        };
        let production_minutes = ore_minutes.max(clay_minutes);
        if production_minutes >= minutes {
            return None;
        }
        let mut obsidian_robot = self.collect(production_minutes);
        obsidian_robot.n_obsidian_robots += 1;
        obsidian_robot.ore -= obsidian_robot.blue_print.obsidian_robot_ore;
        obsidian_robot.clay -= obsidian_robot.blue_print.obsidian_robot_clay;
        let remaining_minutes = minutes - production_minutes;
        obsidian_robot.clear_excess_ressources(remaining_minutes);
        Some((obsidian_robot, remaining_minutes))
    }
    fn build_geode_robot_and_collect(&self, minutes: u64) -> Option<(Self, u64)> {
        if self.n_obsidian_robots == 0 {
            return None;
        }
        let ore_minutes = if self.ore >= self.blue_print.geode_robot_ore {
            1
        } else {
            1 + (self.blue_print.geode_robot_ore - self.ore).div_ceil(self.n_ore_robots)
        };
        let obsidian_minutes = if self.obsidian >= self.blue_print.geode_robot_obsidian {
            1
        } else {
            1 + (self.blue_print.geode_robot_obsidian - self.obsidian)
                .div_ceil(self.n_obsidian_robots)
        };
        let production_minutes = ore_minutes.max(obsidian_minutes);
        if production_minutes >= minutes {
            return None;
        }
        let mut geode_robot = self.collect(production_minutes);
        geode_robot.n_geode_robots += 1;
        geode_robot.ore -= geode_robot.blue_print.geode_robot_ore;
        geode_robot.obsidian -= geode_robot.blue_print.geode_robot_obsidian;
        let remaining_minutes = minutes - production_minutes;
        geode_robot.clear_excess_ressources(remaining_minutes);
        Some((geode_robot, remaining_minutes))
    }
    fn execute(&self, minutes: u64) -> u64 {
        let mut cache: HashMap<(u64, Self), u64> = HashMap::new();
        self.execute_recursive(&mut cache, minutes)
    }
    fn execute_recursive(&self, cache: &mut HashMap<(u64, Self), u64>, minutes: u64) -> u64 {
        if minutes == 0 {
            return self.geodes;
        }
        if let Some(max_geode_collection) = cache.get(&(minutes, *self)) {
            return *max_geode_collection;
        }

        let mut max_geode_collection = self.n_geode_robots * minutes + self.geodes;

        for (action, remaining_minutes) in self.actions(minutes).iter() {
            max_geode_collection =
                max_geode_collection.max(action.execute_recursive(cache, *remaining_minutes));
        }

        cache.insert((minutes, *self), max_geode_collection);
        max_geode_collection
    }
}

pub fn day_19() -> Result<()> {
    let input = include_str!("../../assets/day_19.txt");
    let geode_collectors: Vec<GeodeCollection> = input
        .lines()
        .map(BluePrint::from)
        .map(GeodeCollection::new)
        .collect();
    let minutes = 24;
    let result_part1: u64 = geode_collectors
        .iter()
        .map(|gc| gc.execute(minutes) * gc.blue_print.id)
        .sum();
    println!("result day 19 part 1: {}", result_part1);
    assert_eq!(result_part1, 2_341);

    #[cfg(feature = "long-run-time")]
    {
        let minutes = 32;
        let result_part2: u64 = geode_collectors
            .iter()
            .take(3)
            .map(|gc| gc.execute(minutes))
            .product();
        println!("result day 19 part 2: {}", result_part2);
        assert_eq!(result_part2, 3_689);
    }
    #[cfg(not(feature = "long-run-time"))]
    {
        println!("day 19 part 2 skipped because of long run time")
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = "Blueprint 1: \
            Each ore robot costs 4 ore. \
            Each clay robot costs 2 ore. \
            Each obsidian robot costs 3 ore and 14 clay. \
            Each geode robot costs 2 ore and 7 obsidian.\n\
        Blueprint 2: \
            Each ore robot costs 2 ore. \
            Each clay robot costs 3 ore. \
            Each obsidian robot costs 3 ore and 8 clay. \
            Each geode robot costs 3 ore and 12 obsidian.";
        let geode_collectors: Vec<GeodeCollection> = input
            .lines()
            .map(BluePrint::from)
            .map(GeodeCollection::new)
            .collect();
        let minutes = 24;
        let result_part1: u64 = geode_collectors
            .iter()
            .map(|gc| gc.execute(minutes) * gc.blue_print.id)
            .sum();
        println!("result example day 19 part 1: {}", result_part1);
        assert_eq!(result_part1, 33);
        // part 2
        let minutes = 32;
        let result_part2_0 = geode_collectors[0].execute(minutes);
        println!("result example day 19 part 2, 0: {}", result_part2_0);
        assert_eq!(result_part2_0, 56);
        let result_part2_1 = geode_collectors[1].execute(minutes);
        println!("result example day 19 part 2, 1: {}", result_part2_1);
        assert_eq!(result_part2_1, 62);
        Ok(())
    }
}

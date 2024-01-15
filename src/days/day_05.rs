//!day_05.rs

use anyhow::Result;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct CrateStacks {
    stacks: HashMap<u8, VecDeque<char>>,
}

impl From<&str> for CrateStacks {
    fn from(value: &str) -> Self {
        let mut cs = CrateStacks {
            stacks: HashMap::new(),
        };
        for stack in value
            .lines()
            .last()
            .unwrap()
            .split_whitespace()
            .filter_map(|c| c.parse::<u8>().ok())
        {
            cs.stacks.insert(stack, VecDeque::new());
        }
        for line in value.lines() {
            for (i, c) in line
                .chars()
                .enumerate()
                .filter(|(_, c)| c.is_ascii_alphabetic())
            {
                assert_eq!(i % 4, 1);
                let stack = (1 + i / 4) as u8;
                cs.stacks.get_mut(&stack).unwrap().push_back(c);
            }
        }
        cs
    }
}

impl CrateStacks {
    fn apply_crane_command(&mut self, crane_command: &CraneCommand) {
        for _i in 0..crane_command.count {
            if let Some(cr) = self
                .stacks
                .get_mut(&crane_command.source)
                .unwrap()
                .pop_front()
            {
                self.stacks
                    .get_mut(&crane_command.target)
                    .unwrap()
                    .push_front(cr);
            }
        }
    }
    fn apply_crane_9001_command(&mut self, crane_command: &CraneCommand) {
        let mut intermediate: VecDeque<char> =
            VecDeque::with_capacity(crane_command.count as usize);
        for _i in 0..crane_command.count {
            if let Some(cr) = self
                .stacks
                .get_mut(&crane_command.source)
                .unwrap()
                .pop_front()
            {
                intermediate.push_front(cr);
            }
        }
        for cr in intermediate.iter() {
            self.stacks
                .get_mut(&crane_command.target)
                .unwrap()
                .push_front(*cr);
        }
    }
    fn get_top_crates(&self) -> String {
        let mut top_crates = String::new();
        for stack in 1..=self.stacks.len() as u8 {
            top_crates.push(self.stacks.get(&stack).unwrap()[0]);
        }
        top_crates
    }
}

#[derive(Debug)]
struct CraneCommand {
    count: u8,
    source: u8,
    target: u8,
}

impl From<&str> for CraneCommand {
    fn from(value: &str) -> Self {
        let mut value_iter = value
            .split_whitespace()
            .filter_map(|v| v.parse::<u8>().ok());
        Self {
            count: value_iter.next().unwrap(),
            source: value_iter.next().unwrap(),
            target: value_iter.next().unwrap(),
        }
    }
}

pub fn day_05() -> Result<()> {
    let input = include_str!("../../assets/day_05.txt");
    let (crate_stack_str, crane_commands) = input.split_once("\n\n").unwrap();
    let mut crate_stack = CrateStacks::from(crate_stack_str);
    let crane_commands: Vec<CraneCommand> =
        crane_commands.lines().map(CraneCommand::from).collect();
    for cr in crane_commands.iter() {
        crate_stack.apply_crane_command(cr);
    }

    let result_part1 = crate_stack.get_top_crates();
    println!("result day 05 part 1: {}", result_part1);
    assert_eq!(result_part1, String::from("QNHWJVJZW"));

    crate_stack = CrateStacks::from(crate_stack_str);
    for cr in crane_commands.iter() {
        crate_stack.apply_crane_9001_command(cr);
    }

    let result_part2 = crate_stack.get_top_crates();
    println!("result day 05 part 2: {}", result_part2);
    assert_eq!(result_part2, String::from("BPCZJLFJW"));

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part_1() -> Result<()> {
        let input = "    [D]    \n\
                           [N] [C]    \n\
                           [Z] [M] [P]\n\
                           1   2   3 \n\
                           \n\
                           move 1 from 2 to 1\n\
                           move 3 from 1 to 3\n\
                           move 2 from 2 to 1\n\
                           move 1 from 1 to 2";
        // add your test here
        let (crate_stack_str, crane_commands) = input.split_once("\n\n").unwrap();
        let mut crate_stack = CrateStacks::from(crate_stack_str);
        let crane_commands: Vec<CraneCommand> =
            crane_commands.lines().map(CraneCommand::from).collect();
        for cr in crane_commands.iter() {
            crate_stack.apply_crane_command(cr);
        }

        let result_part1 = crate_stack.get_top_crates();
        println!("result example day 05 part 1: {}", result_part1);
        assert_eq!(result_part1, String::from("CMZ"));

        crate_stack = CrateStacks::from(crate_stack_str);
        for cr in crane_commands.iter() {
            crate_stack.apply_crane_9001_command(cr);
        }

        let result_part2 = crate_stack.get_top_crates();
        println!("result example day 05 part 2: {}", result_part2);
        assert_eq!(result_part2, String::from("MCD"));
        Ok(())
    }
}

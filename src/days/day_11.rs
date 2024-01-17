//!day_11.rs

use anyhow::Result;
use evalexpr::eval_int;
use std::collections::VecDeque;


#[derive(Clone, Copy)]
enum InspectionMethod {
    Devision(i64),
    #[cfg(feature = "long-run-time")]
    Modulo(i64),
}



#[derive(Debug)]
struct Monkey {
    items: VecDeque<i64>,
    operation: String,
    test_divisor: i64,
    test_true: usize,
    test_false: usize,
    inspected_items_count: i64,
}

impl From<&str> for Monkey {
    fn from(value: &str) -> Self {
        let mut line_iter = value.lines().skip(1);
        let items: VecDeque<i64> = line_iter
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|i| i.parse::<i64>().unwrap())
            .collect();
        let operation = line_iter
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .to_string();
        let test_divisor = line_iter
            .next()
            .unwrap()
            .split_once("by ")
            .unwrap()
            .1
            .parse::<i64>()
            .unwrap();
        let test_true = line_iter
            .next()
            .unwrap()
            .split_once("monkey ")
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap();
        let test_false = line_iter
            .next()
            .unwrap()
            .split_once("monkey ")
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap();
        Self {
            items,
            operation,
            test_divisor,
            test_true,
            test_false,
            inspected_items_count: 0,
        }
    }
}

impl Monkey {
    fn catch_item(&mut self, item: i64) {
        self.items.push_back(item);
    }
    fn throw_item(&mut self, inspection_method: InspectionMethod) -> Option<(usize, i64)> {
        match self.items.pop_front() {
            Some(item) => {
                self.inspected_items_count += 1;
                let expression = format!("old = {}; {}; new", item, self.operation);
                let new = eval_int(expression.as_str()).expect("bad expression");
                let new = match inspection_method {
                    InspectionMethod::Devision(divisor) => new / divisor,
                    #[cfg(feature = "long-run-time")]
                    InspectionMethod::Modulo(divisor) => new % divisor,
                };
                if new % self.test_divisor == 0 {
                    Some((self.test_true, new))
                } else {
                    Some((self.test_false, new))
                }
            }
            None => None,
        }
    }
}

fn play_n_rounds(monkeys: &mut Vec<Monkey>, inspection_method: InspectionMethod, n_rounds: usize) -> i64 {
    let mut max_inspections = 0;
    let mut second_max_inspections = 0;
    for round in 0..n_rounds {
        let mut monkey_index = 0;
        while monkey_index < monkeys.len() {
            while let Some((catch_index, item)) = monkeys[monkey_index].throw_item(inspection_method) {
                monkeys[catch_index].catch_item(item);
            }
            if round == n_rounds - 1 {
                if monkeys[monkey_index].inspected_items_count > max_inspections {
                    second_max_inspections = max_inspections;
                    max_inspections = monkeys[monkey_index].inspected_items_count;
                } else if monkeys[monkey_index].inspected_items_count > second_max_inspections {
                    second_max_inspections = monkeys[monkey_index].inspected_items_count;
                }
            }
            monkey_index += 1;
        }
    }
    max_inspections * second_max_inspections
}

pub fn day_11() -> Result<()> {
    let input = include_str!("../../assets/day_11.txt");
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::from).collect();
    let inspection_method = InspectionMethod::Devision(3);
    let result_part1 = play_n_rounds(&mut monkeys, inspection_method, 20);
    println!("result day 11 part 1: {}", result_part1);
    assert_eq!(result_part1, 67_830);

    #[cfg(feature = "long-run-time")]
    {
        let mut monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::from).collect();
        let super_divisor: i64 = monkeys.iter().map(|m| m.test_divisor).product();
        let inspection_method = InspectionMethod::Modulo(super_divisor);
        let result_part2 = play_n_rounds(&mut monkeys, inspection_method, 10_000);
        println!("result day 11 part 2: {}", result_part2);
        assert_eq!(result_part2, 15_305_381_442);
    }
    #[cfg(not(feature = "long-run-time"))]
    {
        println!("day 11 part 2 skipped because of long run time")
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../assets/day_11_example.txt");
        let mut monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::from).collect();
        let inspection_method = InspectionMethod::Devision(3);
        let result_part1 = play_n_rounds(&mut monkeys, inspection_method, 20);
        println!("result example day 11 part 1: {}", result_part1);
        assert_eq!(result_part1, 10_605);


        let mut monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::from).collect();
        let super_divisor: i64 = monkeys.iter().map(|m| m.test_divisor).product();
        let inspection_method = InspectionMethod::Modulo(super_divisor);
        let result_part2 = play_n_rounds(&mut monkeys, inspection_method, 10_000);
        println!("result example day 11 part 2: {}", result_part2);
        assert_eq!(result_part2, 2_713_310_158);
        Ok(())
    }
}

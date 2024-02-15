//!day_21.rs

use anyhow::{anyhow, Result};
use evalexpr::eval_int;
use std::collections::HashMap;

fn eval_monkeys(key: String, monkeys: &HashMap<String, String>) -> Result<i64> {
    let value = monkeys.get(&key).ok_or(anyhow!("key not found"))?;
    match value.parse::<i64>() {
        Ok(v) => Ok(v),
        Err(_) => {
            let mut expression_iter = value.split_whitespace();
            let left = eval_monkeys(expression_iter.next().unwrap().to_string(), monkeys)?;
            let symbol = expression_iter.next().unwrap();
            let right = eval_monkeys(expression_iter.next().unwrap().to_string(), monkeys)?;
            let expression = format!("{} {} {}", left, symbol, right);
            let v = eval_int(expression.as_str())?;
            Ok(v)
        }
    }
}

fn eval_non_humn_expr<'a>(
    key: &String,
    humn_key: &String,
    monkeys: &'a HashMap<String, String>,
) -> Result<(i64, bool, &'a str)> {
    let value = monkeys.get(key).ok_or(anyhow!("key not found"))?;
    let mut expression_iter = value.split_whitespace();
    let left = expression_iter.next().unwrap();
    let symbol = expression_iter.next().unwrap();
    let right = expression_iter.next().unwrap();
    if left == humn_key {
        Ok((eval_monkeys(right.to_string(), monkeys)?, false, symbol))
    } else if right == humn_key {
        Ok((eval_monkeys(left.to_string(), monkeys)?, true, symbol))
    } else {
        Err(anyhow!("bad humn_key"))
    }
}

fn eval_human(monkeys: &HashMap<String, String>) -> Result<i64> {
    // 1. go from first call of humn up to root and collect keys in path to humn
    let mut current_key = &String::from("humn");
    let mut humn_keys: Vec<&String> = vec![current_key];
    loop {
        let (key, _) = monkeys
            .iter()
            .find(|(_, v)| v.contains(current_key))
            .unwrap();
        current_key = key;
        if key == "root" {
            break;
        }
        humn_keys.push(key);
    }
    // 2. caculate comparision value
    let humn_key = humn_keys.pop().unwrap();
    let (mut eval_value, ..) = eval_non_humn_expr(current_key, humn_key, monkeys)?;
    // 3. move down to humn expression and formulate eval_string
    current_key = humn_key;
    while let Some(humn_key) = humn_keys.pop() {
        let (next_eval_value, right, symbol) = eval_non_humn_expr(current_key, humn_key, monkeys)?;
        eval_value = match (right, symbol) {
            (_, "+") => eval_value - next_eval_value,
            (_, "*") => eval_value / next_eval_value,
            (false, "-") => eval_value + next_eval_value,
            (true, "-") => next_eval_value - eval_value,
            (false, "/") => eval_value * next_eval_value,
            (true, "/") => next_eval_value / eval_value,
            _ => return Err(anyhow!("internal calculation error")),
        };
        current_key = humn_key;
    }

    Ok(eval_value)
}

pub fn day_21() -> Result<()> {
    let input = include_str!("../../assets/day_21.txt");
    let monkeys: HashMap<String, String> = input
        .lines()
        .map(|l| {
            l.split_once(": ")
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .unwrap()
        })
        .collect();

    let result_part1 = eval_monkeys(String::from("root"), &monkeys)?;
    println!("result day 21 part 1: {}", result_part1);
    assert_eq!(result_part1, 232_974_643_455_000);

    let result_part2 = eval_human(&monkeys)?;
    println!("result day 21 part 2: {}", result_part2);
    assert_eq!(result_part2, 3_740_214_169_961);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = include_str!("../../assets/day_21_example.txt");
        let monkeys: HashMap<String, String> = input
            .lines()
            .map(|l| {
                l.split_once(": ")
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .unwrap()
            })
            .collect();

        let result_part1 = eval_monkeys(String::from("root"), &monkeys)?;
        println!("result example day 21 part 1: {}", result_part1);
        assert_eq!(result_part1, 152);

        let result_part2 = eval_human(&monkeys)?;
        println!("result example day 21 part 2: {}", result_part2);
        assert_eq!(result_part2, 301);
        Ok(())
    }
}

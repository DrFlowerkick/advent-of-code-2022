//!day_01.rs

use anyhow::Result;

pub fn day_01() -> Result<()> {
    let input = include_str!("../../assets/day_01.txt");
    let mut max_calories: Vec<u64> = Vec::new();
    for calorie_package in input.split("\n\n") {
        let calorie_sum: u64 = calorie_package
            .lines()
            .map(|l| l.parse::<u64>().expect("bad input"))
            .sum();
        max_calories.push(calorie_sum);
    }
    max_calories.sort();

    let result_part1 = max_calories.last().unwrap();
    println!("result day 01 part 1: {}", result_part1);
    assert_eq!(*result_part1, 74_711);

    let result_part2: u64 = max_calories[max_calories.len() - 3..].iter().sum();
    println!("result day 01 part 2: {}", result_part2);
    assert_eq!(result_part2, 209_481);

    Ok(())
}

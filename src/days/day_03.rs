//!day_03.rs

use anyhow::Result;

enum LineState {
    First(String),
    Second(String),
    Last,
}

fn calc_priority(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        // a as u32 is 97 -> -96 == 1
        return (c as u32) - 96;
    }
    if c.is_ascii_uppercase() {
        // A as u32 is 65 -> -38 == 27
        return (c as u32) - 38;
    }
    panic!("non ascii char");
}

pub fn day_03() -> Result<()> {
    let input = include_str!("../../assets/day_03.txt");
    let mut result_part1 = 0;
    let mut result_part2 = 0;
    let mut line_state = LineState::Last;
    for rucksack in input.lines() {
        let num_items = rucksack.chars().count();
        let compartment_1 = &rucksack[..num_items / 2];
        let compartment_2 = &rucksack[num_items / 2..];
        for item in compartment_1.chars() {
            if compartment_2.contains(item) {
                result_part1 += calc_priority(item);
                break;
            }
        }
        line_state = match line_state {
            LineState::Last => LineState::First(rucksack.to_string()),
            LineState::First(first_rucksack) => {
                let mut merge_first_second = String::with_capacity(first_rucksack.len());
                for item in first_rucksack.chars() {
                    if rucksack.contains(item) {
                        merge_first_second.push(item);
                    }
                }
                LineState::Second(merge_first_second)
            }
            LineState::Second(merge_first_second) => {
                for item in merge_first_second.chars() {
                    if rucksack.contains(item) {
                        result_part2 += calc_priority(item);
                        break;
                    }
                }
                LineState::Last
            }
        }
    }
    println!("result day 03 part 1: {}", result_part1);
    assert_eq!(result_part1, 8_088);

    println!("result day 03 part 2: {}", result_part2);
    assert_eq!(result_part2, 2_522);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part_1() -> Result<()> {
        let input = include_str!("../../assets/day_03.txt");
        for line in input.lines() {
            assert_eq!(line.chars().count(), line[..].len());
        }
        eprintln!("a: {}", 'a' as u32);
        eprintln!("z: {}", 'z' as u32);
        eprintln!("A: {}", 'A' as u32);
        eprintln!("Z: {}", 'Z' as u32);
        Ok(())
    }
}

//!day_04.rs

use anyhow::Result;

pub fn day_04() -> Result<()> {
    let input = include_str!("../../assets/day_04.txt");
    let mut result_part1 = 0;
    //let mut result_part2 = 0;

    

    println!("result day 04 part 1: {}", result_part1);
    //assert_eq!(result_part1, 8_088);
    /*
    println!("result day 04 part 2: {}", result_part2);
    assert_eq!(result_part2, 2_522);
    */
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part_1() -> Result<()> {
        let input = include_str!("../../assets/day_04.txt");
        // add your test here
        Ok(())
    }
}

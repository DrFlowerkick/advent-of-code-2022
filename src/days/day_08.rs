//!day_08.rs

use anyhow::Result;

pub fn day_08() -> Result<()> {
    let input = include_str!("../../assets/day_08.txt");
    
    let result_part1 = 0;
    println!("result day 08 part 1: {}", result_part1);
    //assert_eq!(result_part1, 1_644_735);
    //let result_part2 = 0;
    //println!("result day 08 part 2: {}", result_part2);
    //assert_eq!(result_part2, 1_300_850);
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = "30373\n\
                           25512\n\
                           65332\n\
                           33549\n\
                           35390";
        let result_part1 = 0;
        println!("result example day 08 part 1: {}", result_part1);
        assert_eq!(result_part1, 21);
        //let result_part2 = 0;
        //println!("result example day 08 part 2: {}", result_part2);
        //assert_eq!(result_part2, 1);
        Ok(())
    }
}

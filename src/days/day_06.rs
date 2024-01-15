//!day_06.rs

use anyhow::Result;
use std::collections::VecDeque;

#[derive(Default)]
struct MarkerState {
    chars: VecDeque<char>,
    count: usize,
}

impl MarkerState {
    fn check_marker(&mut self, c: char, marker_size: usize) -> Option<usize> {
        self.count += 1;
        self.chars.push_back(c);
        if self.count >= marker_size {
            for (i, c1) in self.chars.iter().enumerate() {
                for c2 in self.chars.iter().skip(i + 1) {
                    if c1 == c2 {
                        self.chars.pop_front();
                        return None;
                    }
                }
            }
            return Some(self.count);
        }
        None
    }
}

pub fn day_06() -> Result<()> {
    let input = include_str!("../../assets/day_06.txt");

    let mut result_part1 = 0;
    let mut result_part2 = 0;
    let mut start_of_paket_marker_state = MarkerState::default();
    let mut check_start_of_paket_marker = true;
    let mut start_of_message_marker_state = MarkerState::default();
    for c in input.chars() {
        if check_start_of_paket_marker {
            if let Some(marker_pos) = start_of_paket_marker_state.check_marker(c, 4) {
                result_part1 = marker_pos;
                check_start_of_paket_marker = false;
            }
        }
        if let Some(marker_pos) = start_of_message_marker_state.check_marker(c, 14) {
            result_part2 = marker_pos;
            break;
        }
    }
    println!("result day 06 part 1: {}", result_part1);
    assert_eq!(result_part1, 1_920);

    println!("result day 06 part 2: {}", result_part2);
    assert_eq!(result_part2, 2_334);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb\n\
                           bvwbjplbgvbhsrlpgdmjqwftvncz\n\
                           nppdvjthqldpwncqszvftbrmjlhg\n\
                           nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg\n\
                           zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        // add your test here
        let test_vector_1 = [7, 5, 6, 10, 11];
        let test_vector_2 = [19, 23, 23, 29, 26];
        for (i, line) in input.lines().enumerate() {
            let mut start_of_paket_marker_state = MarkerState::default();
            let mut check_start_of_paket_marker = true;
            let mut start_of_message_marker_state = MarkerState::default();
            for c in line.chars() {
                if check_start_of_paket_marker {
                    if let Some(marker_pos) = start_of_paket_marker_state.check_marker(c, 4) {
                        let result_part1 = marker_pos;
                        check_start_of_paket_marker = false;
                        println!("result example {} day 06 part 1: {}", i + 1, result_part1);
                        assert_eq!(result_part1, test_vector_1[i]);
                    }
                }
                if let Some(marker_pos) = start_of_message_marker_state.check_marker(c, 14) {
                    let result_part2 = marker_pos;
                    println!("result example {} day 06 part 2: {}", i + 1, result_part2);
                    assert_eq!(result_part2, test_vector_2[i]);
                    break;
                }
            }
        }

        //let result_part2 = 0;
        //println!("result example day 06 part 2: {}", result_part2);
        //assert_eq!(result_part2, 1);
        Ok(())
    }
}

//!day_20.rs

use anyhow::Result;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
struct Code {
    code: i64,
    id: usize,
}

impl From<(usize, i64)> for Code {
    fn from(value: (usize, i64)) -> Self {
        Code {
            code: value.1,
            id: value.0,
        }
    }
}

impl Code {
    fn apply_decryption_key(&self, decryption_key: i64) -> Code {
        Self {
            code: self.code * decryption_key,
            id: self.id,
        }
    }
}

fn rotate_codes(codes: &[Code], decryption_key: i64, cycles: u64) -> Vec<Code> {
    let len = codes.len() as i64;
    let codes: Vec<Code> = codes
        .iter()
        .map(|c| c.apply_decryption_key(decryption_key))
        .collect();
    let mut rotated_codes: Vec<Code> = codes.clone();
    for _ in 0..cycles {
        for c in codes.iter() {
            let index = rotated_codes.iter().position(|rc| rc.id == c.id).unwrap();
            // number of steps of a full cycle is euqal to len - 1
            // remove steps of full cycles with modulo
            let steps = c.code % (len - 1);
            let step_index = steps + index as i64;
            // if cycling over one end, do an extra step in the corresponding direction
            let cycle_offset = if step_index >= len {
                1
            } else if step_index < 0 {
                -1
            } else {
                0
            };
            // use rem_euclid() to make sure, we always have a positiv index in range 0..len
            let step_index = (step_index + cycle_offset).rem_euclid(len) as usize;
            match step_index.cmp(&index) {
                Ordering::Equal => continue,
                Ordering::Greater => rotated_codes[index..=step_index].rotate_left(1),
                Ordering::Less => rotated_codes[step_index..=index].rotate_right(1),
            }
        }
    }
    rotated_codes
}

fn get_coordinates_sum(codes: &[Code]) -> i64 {
    let len = codes.len();
    let index_0 = codes.iter().position(|c| c.code == 0).unwrap();
    let mut sum = 0;
    for steps in [1000_usize, 2000, 3000].into_iter() {
        let remaining_steps = steps % len;
        let index_steps = (index_0 + remaining_steps) % len;
        #[cfg(test)]
        eprintln!("{}", codes[index_steps].code);
        sum += codes[index_steps].code;
    }
    sum
}

pub fn day_20() -> Result<()> {
    let input = include_str!("../../assets/day_20.txt");
    let codes: Vec<Code> = input
        .lines()
        .enumerate()
        .map(|(id, c)| Code::from((id, c.parse::<i64>().expect("bad input"))))
        .collect();
    let rotated_codes = rotate_codes(&codes, 1, 1);
    let result_part1 = get_coordinates_sum(&rotated_codes);
    println!("result day 20 part 1: {}", result_part1);
    assert_eq!(result_part1, 4_426);

    let decryption_key = 811_589_153;
    let cycles = 10;
    let rotated_codes = rotate_codes(&codes, decryption_key, cycles);
    let result_part2 = get_coordinates_sum(&rotated_codes);
    println!("result day 20 part 2: {}", result_part2);
    assert_eq!(result_part2, 8_119_137_886_612);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let a: i64 = -9;
        eprintln!("mod test: {}", a.rem_euclid(7));
        let codes = [1, 2, -3, 3, -2, 0, 4];
        let codes: Vec<Code> = codes.into_iter().enumerate().map(Code::from).collect();
        let rotated_codes = rotate_codes(&codes, 1, 1);
        let result_part1 = get_coordinates_sum(&rotated_codes);
        println!("result example day 20 part 1: {}", result_part1);
        assert_eq!(result_part1, 3);

        let decryption_key = 811_589_153;
        let cycles = 10;
        let rotated_codes = rotate_codes(&codes, decryption_key, cycles);
        let result_part2 = get_coordinates_sum(&rotated_codes);
        println!("result example day 20 part 2: {}", result_part2);
        assert_eq!(result_part2, 1_623_178_306);

        Ok(())
    }
}

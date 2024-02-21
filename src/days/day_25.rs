//!day_25.rs

use std::fmt::Display;

use anyhow::Result;

#[derive(Debug, Clone, Copy)]
struct Base5Place {
    // 5^value: 0 -> 1, 1 -> 5, 2 -> 25, and so on
    place: u32,
    // allowed values are -2, -1, 0, 1, 2
    value: i64,
    bound: i64,
}

impl Default for Base5Place {
    fn default() -> Self {
        Self {
            place: 0,
            value: 0,
            bound: 2,
        }
    }
}

impl Display for Base5Place {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base_char = match self.value {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => return Err(std::fmt::Error),
        };
        write!(f, "{}", base_char)
    }
}

impl Base5Place {
    fn new(place: u32, value: i64) -> Self {
        assert!((-2..=2).contains(&value));
        let mut bound = 0;
        for p in 0..place {
            bound += 2 * 5_i64.pow(p);
        }
        Self {
            place,
            value,
            bound,
        }
    }
    fn increment_place(&mut self) {
        self.place += 1;
        self.bound += 2 * 5_i64.pow(self.place);
    }
    fn decrement_place(&mut self) {
        if self.place > 0 {
            self.bound -= 2 * 5_i64.pow(self.place);
            self.place -= 1;
        }
    }
    fn place_dezimal(&self) -> i64 {
        5_i64.pow(self.place)
    }
    fn base5_to_dezimal(&self) -> i64 {
        self.value * self.place_dezimal()
    }
    fn from_dezimal(dezimal: i64) -> (Self, i64) {
        let mut base_5_num = Self::default();
        while dezimal.abs() > base_5_num.bound {
            base_5_num.increment_place();
        }

        base_5_num.value = dezimal / base_5_num.place_dezimal();
        let mod_base = dezimal % base_5_num.place_dezimal();
        let mut next_lower_base5_place = base_5_num;
        next_lower_base5_place.decrement_place();
        if mod_base < -next_lower_base5_place.bound {
            base_5_num.value -= 1;
        } else if mod_base > next_lower_base5_place.bound {
            base_5_num.value += 1;
        }
        assert!((-2..=2).contains(&base_5_num.value));

        (base_5_num, dezimal - base_5_num.base5_to_dezimal())
    }
}

struct Base5Num {
    // index in vec must be equal to place of Base5Place
    num: Vec<Base5Place>,
}

impl From<&str> for Base5Num {
    fn from(value: &str) -> Self {
        let mut num: Vec<Base5Place> = Vec::with_capacity(value.chars().count());
        // read chars from right to left!
        for (place, val) in value.chars().rev().enumerate() {
            let base5_value: i64 = match val {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => panic!("bad input"),
            };
            num.push(Base5Place::new(place as u32, base5_value));
        }
        Self { num }
    }
}

impl Display for Base5Num {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // insert chars from max to min base5 place
        for base5_num in self.num.iter().rev() {
            write!(f, "{}", base5_num)?;
        }
        Ok(())
    }
}

impl Base5Num {
    fn to_dezimal(&self) -> i64 {
        let mut dezimal = 0;
        for base5_num in self.num.iter() {
            dezimal += base5_num.base5_to_dezimal();
        }
        dezimal
    }
    fn from_dezimal(dezimal: i64) -> Self {
        let (mut base5_num, mut reminder) = Base5Place::from_dezimal(dezimal);
        let mut num: Vec<Base5Place> = vec![Base5Place::default(); (base5_num.place + 1) as usize];
        num[base5_num.place as usize] = base5_num;
        loop {
            (base5_num, reminder) = Base5Place::from_dezimal(reminder);
            num[base5_num.place as usize] = base5_num;
            if base5_num.place == 0 {
                assert_eq!(reminder, 0);
                break;
            }
        }
        Self { num }
    }
}

pub fn day_25() -> Result<()> {
    let input = include_str!("../../assets/day_25.txt");
    let result_part1: i64 = input.lines().map(|l| Base5Num::from(l).to_dezimal()).sum();
    let result_part1 = format!("{}", Base5Num::from_dezimal(result_part1));
    println!("result day 25 part 1: {}", result_part1);
    assert_eq!(result_part1, String::from("20=02=120-=-2110-0=1"));
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_some_basic_integer_math() {
        let base: i64 = 5;
        eprintln!("base: {}", base);
        for a in -12_i64..12 {
            eprintln!("{} / {} = {}", a, base, a / base);
            eprintln!("{} % {} = {}", a, base, a % base);
        }
    }

    #[test]
    fn base5_to_dezimal_test() -> Result<()> {
        let test_input = "        1              1\n\
                                        2              2\n\
                                        3             1=\n\
                                        4             1-\n\
                                        5             10\n\
                                        6             11\n\
                                        7             12\n\
                                        8             2=\n\
                                        9             2-\n\
                                       10             20\n\
                                       15            1=0\n\
                                       20            1-0\n\
                                     2022         1=11-2\n\
                                    12345        1-0---0\n\
                                314159265  1121-1110-1=0\n\
                                     1747         1=-0-2\n\
                                      906          12111\n\
                                      198           2=0=\n\
                                       11             21\n\
                                      201           2=01\n\
                                       31            111\n\
                                     1257          20012\n\
                                       32            112\n\
                                      353          1=-1=\n\
                                      107           1-12\n\
                                        7             12\n\
                                        3             1=\n\
                                       37            122";
        for line in test_input.lines().map(|l| l.trim()) {
            let mut lsplit = line.split_whitespace();
            let dezimal = lsplit.next().unwrap().parse::<i64>()?;
            let base5_num = Base5Num::from(lsplit.next().unwrap());
            assert_eq!(base5_num.to_dezimal(), dezimal);
        }

        Ok(())
    }

    #[test]
    fn test_example() -> Result<()> {
        let input = "1=-0-2\n\
                           12111\n\
                           2=0=\n\
                           21\n\
                           2=01\n\
                           111\n\
                           20012\n\
                           112\n\
                           1=-1=\n\
                           1-12\n\
                           12\n\
                           1=\n\
                           122";
        let result_part1: i64 = input.lines().map(|l| Base5Num::from(l).to_dezimal()).sum();
        assert_eq!(result_part1, 4_890);
        let result_part1 = format!("{}", Base5Num::from_dezimal(result_part1));
        println!("result example day 25 part 1: {}", result_part1);
        assert_eq!(result_part1, String::from("2=-1=0"));

        Ok(())
    }
}

//!day_02.rs

use anyhow::Result;

#[derive(Clone, Copy)]
enum Rps {
    Rock,
    Paper,
    Scissor,
}

impl From<&str> for Rps {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Rps::Rock,
            "B" | "Y" => Rps::Paper,
            "C" | "Z" => Rps::Scissor,
            _ => panic!("bad input"),
        }
    }
}

impl Rps {
    fn score_shape(&self) -> u64 {
        match self {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissor => 3,
        }
    }
    fn play(&self, other: &Self) -> u64 {
        match (self, other) {
            (Rps::Rock, Rps::Scissor) | (Rps::Paper, Rps::Rock) | (Rps::Scissor, Rps::Paper) => 6,
            (Rps::Rock, Rps::Paper) | (Rps::Paper, Rps::Scissor) | (Rps::Scissor, Rps::Rock) => 0,
            _ => 3,
        }
    }
    fn from_cheating(code: &str, other: &Self) -> Self {
        match code {
            "X" => match other {
                Rps::Rock => Rps::Scissor,
                Rps::Paper => Rps::Rock,
                Rps::Scissor => Rps::Paper,
            },
            "Y" => *other,
            "Z" => match other {
                Rps::Rock => Rps::Paper,
                Rps::Paper => Rps::Scissor,
                Rps::Scissor => Rps::Rock,
            },
            _ => panic!("bad input"),
        }
    }
}

pub fn day_02() -> Result<()> {
    let input = include_str!("../../assets/day_02.txt");
    let mut result_part1 = 0;
    let mut result_part2: u64 = 0;
    for (opp, me) in input.lines().map(|l| l.split_once(' ').unwrap()) {
        let opp = Rps::from(opp);
        let me_task1 = Rps::from(me);
        result_part1 += me_task1.play(&opp) + me_task1.score_shape();
        let me_task2 = Rps::from_cheating(me, &opp);
        result_part2 += me_task2.play(&opp) + me_task2.score_shape();
    }

    println!("result day 02 part 1: {}", result_part1);
    assert_eq!(result_part1, 14_375);

    println!("result day 02 part 2: {}", result_part2);
    assert_eq!(result_part2, 10_274);

    Ok(())
}

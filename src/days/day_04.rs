//!day_04.rs

use anyhow::Result;

struct SectionRange {
    start: u32,
    // end is not included in range as in any proper range definition
    end: u32,
}

impl From<&str> for SectionRange {
    fn from(value: &str) -> Self {
        let (start, end) = value
            .split_once('-')
            .map(|(s, e)| {
                (
                    s.parse::<u32>().expect("bad input"),
                    e.parse::<u32>().expect("bad input") + 1,
                )
            })
            .unwrap();
        Self { start, end }
    }
}

impl SectionRange {
    fn size(&self) -> u32 {
        self.end - self.start
    }
    fn get_overlap(&self, other: &Self) -> Option<Self> {
        let overlap_start = self.start.max(other.start);
        let overlap_end = self.end.min(other.end);
        if overlap_end > overlap_start {
            Some(Self {
                start: overlap_start,
                end: overlap_end,
            })
        } else {
            None
        }
    }
}

pub fn day_04() -> Result<()> {
    let input = include_str!("../../assets/day_04.txt");
    let mut result_part1 = 0;
    let mut result_part2 = 0;
    for line in input.lines() {
        let (left, right) = line
            .split_once(',')
            .map(|(l, r)| (SectionRange::from(l), SectionRange::from(r)))
            .unwrap();
        if let Some(ol) = left.get_overlap(&right) {
            result_part2 += 1;
            if ol.size() == left.size() || ol.size() == right.size() {
                result_part1 += 1;
            }
        }
    }

    println!("result day 04 part 1: {}", result_part1);
    assert_eq!(result_part1, 582);

    println!("result day 04 part 2: {}", result_part2);
    assert_eq!(result_part2, 893);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part_1() -> Result<()> {
        let input = "2-4,6-8\n\
                           2-3,4-5\n\
                           5-7,7-9\n\
                           2-8,3-7\n\
                           6-6,4-6\n\
                           2-6,4-8";
        // add your test here
        let mut result_part1 = 0;
        let mut result_part2 = 0;
        for line in input.lines() {
            let (left, right) = line
                .split_once(',')
                .map(|(l, r)| (SectionRange::from(l), SectionRange::from(r)))
                .unwrap();
            if let Some(ol) = left.get_overlap(&right) {
                result_part2 += 1;
                if ol.size() == left.size() || ol.size() == right.size() {
                    result_part1 += 1;
                }
            }
        }
        println!("result example day 04 part 1: {}", result_part1);
        assert_eq!(result_part1, 2);
        println!("result example day 04 part 2: {}", result_part2);
        assert_eq!(result_part2, 4);
        Ok(())
    }
}

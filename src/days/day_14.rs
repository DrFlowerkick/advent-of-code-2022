//!day_14.rs

use anyhow::Result;
use my_lib::my_geometry::my_point::Point;
use std::cmp::Ordering;

struct RockAndSand {
    rocks: Vec<Point>,
    sand: Vec<Point>,
    start_of_sand: Point,
}

impl From<&str> for RockAndSand {
    fn from(value: &str) -> Self {
        let mut rocks: Vec<Point> = Vec::new();
        for line in value.lines() {
            let mut rock_corners_iter = line.split(" -> ").filter_map(|rc| {
                rc.split_once(',').map(|(x, y)| {
                    Point::from((
                        x.parse::<i64>().expect("bad input"),
                        y.parse::<i64>().expect("bad input"),
                    ))
                })
            });
            let mut current_corner = rock_corners_iter.next().unwrap();
            rocks.push(current_corner);
            for rock_corner in rock_corners_iter {
                let direction = match (
                    current_corner.x.cmp(&rock_corner.x),
                    current_corner.y.cmp(&rock_corner.y),
                ) {
                    (Ordering::Equal, Ordering::Greater) => Point::new(0, -1),
                    (Ordering::Equal, Ordering::Less) => Point::new(0, 1),
                    (Ordering::Greater, Ordering::Equal) => Point::new(-1, 0),
                    (Ordering::Less, Ordering::Equal) => Point::new(1, 0),
                    _ => panic!("bad rock corners"),
                };
                loop {
                    current_corner = current_corner.add(direction);
                    if !rocks.contains(&current_corner) {
                        rocks.push(current_corner);
                    }
                    if current_corner == rock_corner {
                        break;
                    }
                }
            }
        }

        Self {
            rocks,
            sand: Vec::new(),
            start_of_sand: Point::new(500, 0),
        }
    }
}

struct FallingSand<'a> {
    rock_and_sand: &'a RockAndSand,
    sand_unit: Point,
    lowest_rock: i64,
    bottom: Option<i64>,
    finished: bool,
}

impl<'a> FallingSand<'a> {
    fn new(rock_and_sand: &'a RockAndSand, bottom: bool) -> Self {
        let lowest_rock = rock_and_sand.rocks.iter().map(|p| p.y).max().unwrap();
        let bottom = if bottom { Some(lowest_rock + 2) } else { None };
        Self {
            rock_and_sand,
            sand_unit: rock_and_sand.start_of_sand,
            lowest_rock,
            bottom,
            finished: false,
        }
    }
}

impl<'a> Iterator for FallingSand<'a> {
    type Item = (Point, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let result = (
            self.sand_unit,
            self.sand_unit.y >= self.lowest_rock && self.bottom.is_none(),
        );
        let directions = [Point::new(0, 1), Point::new(-1, 1), Point::new(1, 1)];
        for direction in directions.iter() {
            let new_pos = direction.add(self.sand_unit);
            if let Some(b) = self.bottom {
                if new_pos.y == b {
                    break;
                }
            }
            if self.rock_and_sand.sand.contains(&new_pos) {
                continue;
            }
            if self.rock_and_sand.rocks.contains(&new_pos) {
                continue;
            }
            self.sand_unit = new_pos;
            self.finished = new_pos.y > self.lowest_rock && self.bottom.is_none();
            return Some(result);
        }
        self.finished = true;
        Some(result)
    }
}

impl RockAndSand {
    fn pouring_sand(&mut self, bottom: bool) -> usize {
        loop {
            let (new_sand, into_the_abyss) = FallingSand::new(self, bottom).last().unwrap();
            if into_the_abyss {
                break;
            }
            self.sand.push(new_sand);
            if new_sand == self.start_of_sand {
                break;
            }
        }
        self.sand.len()
    }
}

pub fn day_14() -> Result<()> {
    let input = include_str!("../../assets/day_14.txt");
    let mut rock_and_sand = RockAndSand::from(input);
    let result_part1 = rock_and_sand.pouring_sand(false);
    println!("result day 14 part 1: {}", result_part1);
    assert_eq!(result_part1, 1_068);
    #[cfg(feature = "long-run-time")]
    {
        let result_part2 = rock_and_sand.pouring_sand(true);
        println!("result day 14 part 2: {}", result_part2);
        assert_eq!(result_part2, 27_936);
    }
    #[cfg(not(feature = "long-run-time"))]
    {
        println!("day 14 part 2 skipped because of long run time")
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = "498,4 -> 498,6 -> 496,6\n\
                           503,4 -> 502,4 -> 502,9 -> 494,9";
        let mut rock_and_sand = RockAndSand::from(input);
        let result_part1 = rock_and_sand.pouring_sand(false);
        println!("result example day 14 part 1: {}", result_part1);
        assert_eq!(result_part1, 24);

        let result_part2 = rock_and_sand.pouring_sand(true);
        println!("result example day 14 part 2: {}", result_part2);
        assert_eq!(result_part2, 93);
        Ok(())
    }
}

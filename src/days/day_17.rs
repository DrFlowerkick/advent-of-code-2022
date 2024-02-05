//!day_17.rs

use anyhow::Result;
use std::collections::{hash_map::Entry, HashMap};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    fn add(&self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Block {
    HorizontalLine(Point),
    Plus(Point),
    J(Point),
    VerticalLine(Point),
    Square(Point),
}

impl Block {
    fn init() -> Self {
        Self::Square(Point::default())
    }
    fn spawn_new_block(&mut self, highest_block: isize) -> (u8, Self) {
        let point = Point::new(3, highest_block + 4);
        *self = match self {
            Self::HorizontalLine(_) => Self::Plus(point),
            Self::Plus(_) => Self::J(point),
            Self::J(_) => Self::VerticalLine(point),
            Self::VerticalLine(_) => Self::Square(point),
            Self::Square(_) => Self::HorizontalLine(point),
        };
        match self {
            Self::HorizontalLine(_) => (0, *self),
            Self::Plus(_) => (1, *self),
            Self::J(_) => (2, *self),
            Self::VerticalLine(_) => (3, *self),
            Self::Square(_) => (4, *self),
        }
    }
    fn left_rock(&self) -> isize {
        match self {
            Self::HorizontalLine(p) => p.x,
            Self::Plus(p) => p.x,
            Self::J(p) => p.x,
            Self::VerticalLine(p) => p.x,
            Self::Square(p) => p.x,
        }
    }
    fn right_rock(&self) -> isize {
        match self {
            Self::HorizontalLine(p) => p.x + 3,
            Self::Plus(p) => p.x + 2,
            Self::J(p) => p.x + 2,
            Self::VerticalLine(p) => p.x,
            Self::Square(p) => p.x + 1,
        }
    }
    fn bottom_rock(&self) -> isize {
        match self {
            Self::HorizontalLine(p) => p.y,
            Self::Plus(p) => p.y,
            Self::J(p) => p.y,
            Self::VerticalLine(p) => p.y,
            Self::Square(p) => p.y,
        }
    }
    fn top_rock(&self) -> isize {
        match self {
            Self::HorizontalLine(p) => p.y,
            Self::Plus(p) => p.y + 2,
            Self::J(p) => p.y + 2,
            Self::VerticalLine(p) => p.y + 3,
            Self::Square(p) => p.y + 1,
        }
    }
    fn rock_positions(&self) -> Vec<Point> {
        let delta_horizontal_line = [
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(2, 0),
            Point::new(3, 0),
        ];
        let delta_plus = [
            Point::new(1, 0),
            Point::new(0, 1),
            Point::new(1, 1),
            Point::new(2, 1),
            Point::new(1, 2),
        ];
        let delta_j = [
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(2, 0),
            Point::new(2, 1),
            Point::new(2, 2),
        ];
        let delta_vertical_line = [
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(0, 2),
            Point::new(0, 3),
        ];
        let delta_square = [
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(0, 1),
            Point::new(1, 1),
        ];
        let calc_points = |point: &Point, deltas: &[Point]| {
            deltas.iter().map(|d| point.add(*d)).collect::<Vec<Point>>()
        };
        match self {
            Self::HorizontalLine(p) => calc_points(p, &delta_horizontal_line),
            Self::Plus(p) => calc_points(p, &delta_plus),
            Self::J(p) => calc_points(p, &delta_j),
            Self::VerticalLine(p) => calc_points(p, &delta_vertical_line),
            Self::Square(p) => calc_points(p, &delta_square),
        }
    }
    fn apply_jet(&self, jet: bool) -> Self {
        let delta = if jet {
            // move right
            Point::new(1, 0)
        } else {
            // move left
            Point::new(-1, 0)
        };
        match self {
            Self::HorizontalLine(p) => Self::HorizontalLine(p.add(delta)),
            Self::Plus(p) => Self::Plus(p.add(delta)),
            Self::J(p) => Self::J(p.add(delta)),
            Self::VerticalLine(p) => Self::VerticalLine(p.add(delta)),
            Self::Square(p) => Self::Square(p.add(delta)),
        }
    }
    fn move_down(&self) -> Self {
        let delta = Point::new(0, -1);
        match self {
            Self::HorizontalLine(p) => Self::HorizontalLine(p.add(delta)),
            Self::Plus(p) => Self::Plus(p.add(delta)),
            Self::J(p) => Self::J(p.add(delta)),
            Self::VerticalLine(p) => Self::VerticalLine(p.add(delta)),
            Self::Square(p) => Self::Square(p.add(delta)),
        }
    }
}

struct Chamber {
    rocks: Vec<Point>,
    highest_block: isize,
    top_rocks: [isize; 7],
    normalized_top_rocks: [isize; 7],
    offset: isize,
}

impl Chamber {
    fn new() -> Self {
        let mut rocks: Vec<Point> = Vec::with_capacity(2_022);
        for x in 1..8 {
            rocks.push(Point::new(x, 0));
        }
        Self {
            rocks,
            highest_block: 0,
            top_rocks: [0; 7],
            normalized_top_rocks: [0; 7],
            offset: 0,
        }
    }
    fn check_block(&self, block: &Block) -> bool {
        if block.left_rock() == 0 || block.right_rock() == 8 || block.bottom_rock() == 0 {
            return false;
        }
        if block.bottom_rock() > self.highest_block {
            return true;
        }
        for rock in block.rock_positions().iter() {
            if self.rocks.contains(rock) {
                return false;
            }
        }
        true
    }
    fn add_block(&mut self, block: &Block) {
        for rock in block.rock_positions().iter() {
            self.rocks.push(*rock);
            let index = (rock.x - 1) as usize;
            self.top_rocks[index] = self.top_rocks[index].max(rock.y);
        }
        self.highest_block = self.highest_block.max(block.top_rock());
        for (i, tr) in self.top_rocks.iter().enumerate() {
            self.normalized_top_rocks[i] = tr - self.highest_block;
        }
    }
    fn falling_blocks(&mut self, num_blocks: isize, jet_streams: &str) -> isize {
        assert!(num_blocks > 0);
        let mut block_source = Block::init();
        let mut block_counter = 0;
        let mut seen_sequences: HashMap<(u8, usize, [isize; 7]), (isize, isize)> =
            HashMap::with_capacity(2_022);
        let mut jet_iter = jet_streams.chars().map(|c| c == '>').enumerate().cycle();
        while block_counter < num_blocks {
            let (block_index, mut block) = block_source.spawn_new_block(self.highest_block);
            for (jet_index, jet) in &mut jet_iter {
                let jet_block = block.apply_jet(jet);
                if self.check_block(&jet_block) {
                    block = jet_block;
                }
                let falling_block = block.move_down();
                if self.check_block(&falling_block) {
                    block = falling_block;
                } else {
                    self.add_block(&block);
                    block_counter += 1;
                    if self.offset == 0 {
                        let key = (block_index, jet_index, self.normalized_top_rocks);
                        if let Entry::Vacant(e) = seen_sequences.entry(key) {
                            e.insert((block_counter, self.highest_block));
                        } else {
                            if let Some((last_block_counter, last_highest_block)) =
                                seen_sequences.get(&key)
                            {
                                // calc offset and increment block_counter with number of sequence blocks
                                let num_sequences = (num_blocks - block_counter)
                                    / (block_counter - last_block_counter);
                                self.offset =
                                    num_sequences * (self.highest_block - last_highest_block);
                                block_counter +=
                                    num_sequences * (block_counter - last_block_counter);
                            }
                        }
                    }
                    break;
                }
            }
        }
        self.highest_block + self.offset
    }
}

pub fn day_17() -> Result<()> {
    let input = include_str!("../../assets/day_17.txt");
    let num_rocks = 2_022;
    let mut chamber = Chamber::new();
    let result_part1 = chamber.falling_blocks(num_rocks, input);
    println!("result day 17 part 1: {}", result_part1);
    assert_eq!(result_part1, 3_193);
    let num_rocks = 1_000_000_000_000;
    let mut chamber = Chamber::new();
    let result_part1 = chamber.falling_blocks(num_rocks, input);
    println!("result day 17 part 2: {}", result_part1);
    assert_eq!(result_part1, 1_577_650_429_835);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let num_rocks = 2_022;
        let mut chamber = Chamber::new();
        let result_part1 = chamber.falling_blocks(num_rocks, input);
        println!("result example day 17 part 1: {}", result_part1);
        assert_eq!(result_part1, 3_068);
        let num_rocks = 1_000_000_000_000;
        let mut chamber = Chamber::new();
        let result_part1 = chamber.falling_blocks(num_rocks, input);
        println!("result example day 17 part 2: {}", result_part1);
        assert_eq!(result_part1, 1_514_285_714_288);
        Ok(())
    }
}

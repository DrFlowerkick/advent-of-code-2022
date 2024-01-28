//!day_17.rs

use anyhow::Result;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self {x, y}
    }
    fn add(&self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    fn substract(&self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
    fn iter_neighbors(&self, start_from_neighbor: Point) -> impl Iterator<Item = Point> {
        IterPointNeighbors::new(*self, start_from_neighbor)
    }
}

struct IterPointNeighbors {
    center: Point,
    start_from_neighbor: Point,
    current_delta: Point,
    finished: bool,
}

impl IterPointNeighbors {
    fn new(center: Point, start_from_neighbor: Point) -> Self {
        let delta = start_from_neighbor.substract(center);
        assert!(delta.x >= -1 && delta.x <=1 && delta.y >= -1 && delta.y <= 1);
        Self {
            center,
            start_from_neighbor,
            current_delta: delta,
            finished: false,
        }
    }
    fn next_delta(&mut self) -> Point {
        let deltas = [
            Point::new(0, 1),
            Point::new(1, 1),
            Point::new(1, 0),
            Point::new(1, -1),
            Point::new(0, -1),
            Point::new(-1, -1),
            Point::new(-1, 0),
            Point::new(-1, 1),
        ];
        let index = (deltas.iter().position(|d| *d == self.current_delta).unwrap() + 1) % deltas.len();
        self.current_delta = deltas[index];
        self.current_delta
    }
}

impl Iterator for IterPointNeighbors {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let delta = self.next_delta();
        let next_point = self.center.add(delta);
        self.finished = next_point == self.start_from_neighbor;
        Some(next_point)
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
    fn spawn_new_block(&mut self, highest_block: isize) -> Self {
        let point = Point::new(3, highest_block + 4);
        *self = match self {
            Self::HorizontalLine(_) => Self::Plus(point),
            Self::Plus(_) => Self::J(point),
            Self::J(_) => Self::VerticalLine(point),
            Self::VerticalLine(_) => Self::Square(point),
            Self::Square(_) => Self::HorizontalLine(point),
        };
        *self
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
        }
        self.highest_block = self.highest_block.max(block.top_rock());
    }
    fn falling_blocks(&mut self, num_blocks: isize, jet_streams: &str) -> isize {
        assert!(num_blocks > 0);
        let mut block_source = Block::init();
        let mut block_counter = 0;
        let mut jet_iter = jet_streams.chars().map(|c| c == '>').cycle();
        while block_counter < num_blocks {
            let mut block = block_source.spawn_new_block(self.highest_block);
            for jet in &mut jet_iter {
                let jet_block = block.apply_jet(jet);
                if self.check_block(&jet_block) {
                    block = jet_block;
                }
                let falling_block = block.move_down();
                if self.check_block(&falling_block) {
                    block = falling_block;
                } else {
                    self.add_block(&block);
                    break;
                }
            }
            //self.rocks = self.iter_outer_rim().collect();
            block_counter += 1;
        }
        self.highest_block
    }
    fn _iter_outer_rim(&self) -> impl Iterator<Item = Point> + '_ {
        IterOuterRim::_new(self)
    }
}

struct IterOuterRim<'a> {
    chamber: &'a Chamber,
    current_point_index: usize,
    last_point: Point,
    finished: bool,
}

impl<'a> IterOuterRim<'a> {
    fn _new(chamber: &'a Chamber) -> Self {
        let current_point_index = chamber.rocks.iter().enumerate().filter(|(_, p)| p.x == 1).max_by_key(|(_, p)| p.y).unwrap().0;
        let last_point = chamber.rocks[current_point_index].add(Point::new(0, 1));
        Self {
            chamber,
            current_point_index,
            last_point,
            finished: false,
        }
    }
}

impl<'a> Iterator for IterOuterRim<'a> {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let current_point = self.chamber.rocks[self.current_point_index];
        for point in current_point.iter_neighbors(self.last_point) {
            if let Some(next_index) = self.chamber.rocks.iter().position(|p| *p == point) {
                self.current_point_index = next_index;
                self.last_point = current_point;
                break;
            }
        }
        self.finished = current_point.x == 7;
        Some(current_point)
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
    //assert_eq!(result_part1, 3_193);

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

    #[test]
    fn debug_falling_rocks() {

    }
}

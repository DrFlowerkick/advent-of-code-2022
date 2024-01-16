//!day_09.rs

use anyhow::Result;
use my_lib::my_point::Point;

struct RopeAction {
    direction: Point,
    steps: u32,
}

impl From<&str> for RopeAction {
    fn from(value: &str) -> Self {
        let (c, steps) = value
            .split_once(' ')
            .map(|(c, s)| (c, s.parse::<u32>().expect("bad input")))
            .unwrap();
        let direction = match c {
            "U" => Point::new(0, -1),
            "D" => Point::new(0, 1),
            "L" => Point::new(-1, 0),
            "R" => Point::new(1, 0),
            _ => panic!("bad input"),
        };
        Self { direction, steps }
    }
}

struct Rope {
    knots: Vec<Point>,
    tail_seen: Vec<Point>,
}

impl Rope {
    fn new(size: usize) -> Self {
        assert!(size > 1);
        Rope {
            knots: vec![Point::default(); size],
            tail_seen: vec![Point::default()],
        }
    }
    fn apply(&mut self, action: &RopeAction) {
        let index_tail = self.knots.len() - 1;
        for _i in 0..action.steps {
            let mut previous_knot: Option<Point> = None;
            for (knot_index, knot) in self.knots.iter_mut().enumerate() {
                match previous_knot {
                    Some(pk) => {
                        let mut delta = pk.subtract(*knot);
                        if delta.x.abs() > 1 || delta.y.abs() > 1 {
                            delta.x = delta.x.max(-1).min(1);
                            delta.y = delta.y.max(-1).min(1);
                            *knot = knot.add(delta);
                            if knot_index == index_tail && !self.tail_seen.contains(knot) {
                                self.tail_seen.push(*knot);
                            }
                        }
                    }
                    None => {
                        *knot = knot.add(action.direction);
                    }
                }
                previous_knot = Some(*knot);
            }
        }
    }
}

pub fn day_09() -> Result<()> {
    let input = include_str!("../../assets/day_09.txt");
    let rope_actions: Vec<RopeAction> = input.lines().map(RopeAction::from).collect();
    let mut rope = Rope::new(2);
    let mut long_rope = Rope::new(10);
    for ra in rope_actions.iter() {
        rope.apply(ra);
        long_rope.apply(ra);
    }
    let result_part1 = rope.tail_seen.len();
    println!("result day 09 part 1: {}", result_part1);
    assert_eq!(result_part1, 6_311);

    let result_part2 = long_rope.tail_seen.len();
    println!("result day 09 part 2: {}", result_part2);
    assert_eq!(result_part2, 2_482);
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = "R 4\n\
                           U 4\n\
                           L 3\n\
                           D 1\n\
                           R 4\n\
                           D 1\n\
                           L 5\n\
                           R 2";
        let rope_actions: Vec<RopeAction> = input.lines().map(RopeAction::from).collect();
        let mut rope = Rope::new(2);
        for ra in rope_actions.iter() {
            rope.apply(ra);
        }
        let result_part1 = rope.tail_seen.len();
        println!("result example day 09 part 1: {}", result_part1);
        assert_eq!(result_part1, 13);

        let input = "R 5\n\
                           U 8\n\
                           L 8\n\
                           D 3\n\
                           R 17\n\
                           D 10\n\
                           L 25\n\
                           U 20";
        let rope_actions: Vec<RopeAction> = input.lines().map(RopeAction::from).collect();
        let mut rope = Rope::new(10);
        for ra in rope_actions.iter() {
            rope.apply(ra);
        }
        let result_part2 = rope.tail_seen.len();
        println!("result example day 09 part 2: {}", result_part2);
        assert_eq!(result_part2, 36);
        Ok(())
    }
}

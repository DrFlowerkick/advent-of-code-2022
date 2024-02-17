//!day_23.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_geometry::my_point::Point};

use std::collections::HashMap;

struct ElveSwarm {
    elves: Vec<Point>,
    top_left: Point,
    bottom_right: Point,
    directions: [Compass; 4],
}

impl From<&str> for ElveSwarm {
    fn from(value: &str) -> Self {
        let top_left = Point::default();
        let mut bottom_right = Point::default();
        let mut elves: Vec<Point> = Vec::new();
        for (y, line) in value.lines().enumerate().map(|(y, l)| (y as i64, l)) {
            for x in line
                .chars()
                .enumerate()
                .filter(|(_, e)| *e == '#')
                .map(|(x, _)| x as i64)
            {
                bottom_right = (x, y).into();
                elves.push(bottom_right);
            }
        }
        Self {
            elves,
            top_left,
            bottom_right,
            directions: [Compass::N, Compass::S, Compass::W, Compass::E],
        }
    }
}

impl ElveSwarm {
    fn move_elve(&mut self, index: usize, new_position: Point) {
        self.elves[index] = new_position;
        self.top_left.x = self.top_left.x.min(new_position.x);
        self.top_left.y = self.top_left.y.min(new_position.y);
        self.bottom_right.x = self.bottom_right.x.max(new_position.x);
        self.bottom_right.y = self.bottom_right.y.max(new_position.y);
    }
    fn one_movement_phase(&mut self) -> usize {
        let mut new_positions: HashMap<Point, Option<usize>> =
            HashMap::with_capacity(self.elves.len());
        // collect new_positions
        for (index, elve) in self.elves.iter().enumerate() {
            let mut new_position: Option<(Point, usize)> = None;
            let mut free_dir_counter = 0;
            for dir in self.directions.iter() {
                if dir
                    .get_cardinal_and_ordinals()
                    .unwrap()
                    .iter()
                    .all(|s| !self.elves.contains(&elve.add(Point::from(*s))))
                {
                    free_dir_counter += 1;
                    if new_position.is_none() {
                        new_position = Some((elve.add(Point::from(*dir)), index));
                    }
                }
            }
            if free_dir_counter == 4 {
                // all directions are free, no movement required
                continue;
            }
            if let Some((new_position, elve_index)) = new_position {
                new_positions
                    .entry(new_position)
                    // if entry exists, set it to None, since multiple elves want to move to same position
                    .and_modify(|curr| *curr = None)
                    // if entry does not exist, set elve_index
                    .or_insert(Some(elve_index));
            }
        }
        // apply movement
        let mut movement_counter = 0;
        for (new_position, index) in new_positions
            .iter()
            .filter_map(|(n, ei)| ei.map(|i| (*n, i)))
        {
            self.move_elve(index, new_position);
            movement_counter += 1;
        }
        // move first direction to end of list
        self.directions[..].rotate_left(1);
        movement_counter
    }
    fn count_empty_tiles(&self) -> usize {
        let diagonale = self.bottom_right.subtract(self.top_left);
        ((diagonale.x + 1) * (diagonale.y + 1)) as usize - self.elves.len()
    }
}

pub fn day_23() -> Result<()> {
    let input = include_str!("../../assets/day_23.txt");
    let mut elve_swarm = ElveSwarm::from(input);
    let num_rounds: usize = 10;
    for _ in 0..num_rounds {
        elve_swarm.one_movement_phase();
    }
    let result_part1 = elve_swarm.count_empty_tiles();
    println!("result day 23 part 1: {}", result_part1);
    assert_eq!(result_part1, 4_034);
    #[cfg(feature = "long-run-time")]
    {
        let mut num_rounds = num_rounds;
        while elve_swarm.one_movement_phase() > 0 {
            num_rounds += 1;
        }
        num_rounds += 1;
        println!("result day 23 part 1: {}", num_rounds);
        assert_eq!(num_rounds, 960);
    }
    #[cfg(not(feature = "long-run-time"))]
    {
        println!("day 23 part 2 skipped because of long run time")
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = "....#..\n\
                           ..###.#\n\
                           #...#.#\n\
                           .#...##\n\
                           #.###..\n\
                           ##.#.##\n\
                           .#..#..";
        let mut elve_swarm = ElveSwarm::from(input);
        let mut num_rounds: usize = 10;
        for _ in 0..num_rounds {
            elve_swarm.one_movement_phase();
        }
        let result_part1 = elve_swarm.count_empty_tiles();
        println!("result example day 23 part 1: {}", result_part1);
        assert_eq!(result_part1, 110);

        while elve_swarm.one_movement_phase() > 0 {
            num_rounds += 1;
        }
        num_rounds += 1;
        println!("result day 23 part 1: {}", num_rounds);
        assert_eq!(num_rounds, 20);

        Ok(())
    }
}

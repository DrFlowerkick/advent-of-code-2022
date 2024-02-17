//!day_24.rs

use anyhow::Result;
use num::integer::lcm;
use std::collections::BTreeSet;

use my_lib::{my_compass::Compass, my_geometry::my_point::Point};

#[derive(Default)]
struct BlizzardVale {
    north_blizzards: Vec<Point>,
    east_blizzards: Vec<Point>,
    south_blizzards: Vec<Point>,
    west_blizzards: Vec<Point>,
    num_rows: i64,
    num_columns: i64,
    lcm_rc: i64,
}

impl From<&str> for BlizzardVale {
    fn from(value: &str) -> Self {
        let mut blizzard_vale = BlizzardVale::default();
        for (y, line) in value
            .lines()
            .skip(1)
            .enumerate()
            .map(|(y, l)| (y as i64, l))
        {
            for (x, c) in line.chars().skip(1).enumerate().map(|(x, c)| (x as i64, c)) {
                match c {
                    '^' => blizzard_vale.north_blizzards.push((x, y).into()),
                    '>' => blizzard_vale.east_blizzards.push((x, y).into()),
                    'v' => blizzard_vale.south_blizzards.push((x, y).into()),
                    '<' => blizzard_vale.west_blizzards.push((x, y).into()),
                    _ => (),
                }
                blizzard_vale.num_columns = x;
            }
            blizzard_vale.num_rows = y;
        }
        blizzard_vale.lcm_rc = lcm(blizzard_vale.num_columns, blizzard_vale.num_rows);
        blizzard_vale
    }
}

impl BlizzardVale {
    fn is_start_pos(&self, state: ExpeditionState) -> bool {
        (state.x, state.y) == (0, -1)
    }
    fn is_end_pos(&self, state: ExpeditionState) -> bool {
        (state.x, state.y) == (self.num_columns - 1, self.num_rows)
    }
    fn is_out_of_vale(&self, state: ExpeditionState) -> bool {
        (state.x < 0 || state.y < 0 || state.x >= self.num_columns || state.y >= self.num_rows)
            && !self.is_start_pos(state)
            && !self.is_end_pos(state)
    }
    fn check_pos_for_blizzards(&self, state: ExpeditionState) -> bool {
        if self.is_start_pos(state) || self.is_end_pos(state) {
            return false;
        }
        let pos: Point = (state.x, state.y).into();
        for dir in Compass::cardinals().into_iter() {
            // shift pos in negativ direction of dir
            let mut shifted_pos = pos.subtract(Point::from(dir).scale(state.minutes));
            // modulo of shifted pos with size of vale (which means that pos always stays inside of vale)
            match dir {
                Compass::N | Compass::S => shifted_pos.y = shifted_pos.y.rem_euclid(self.num_rows),
                Compass::E | Compass::W => {
                    shifted_pos.x = shifted_pos.x.rem_euclid(self.num_columns)
                }
                _ => (),
            }
            // check if shifted_pos collides with a blizzard
            if match dir {
                Compass::N => self.north_blizzards.contains(&shifted_pos),
                Compass::E => self.east_blizzards.contains(&shifted_pos),
                Compass::S => self.south_blizzards.contains(&shifted_pos),
                Compass::W => self.west_blizzards.contains(&shifted_pos),
                _ => false,
            } {
                return true;
            }
        }
        false
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct ExpeditionState {
    minutes: i64,
    x: i64,
    y: i64,
    phase: i64,
}

impl ExpeditionState {
    fn new() -> Self {
        Self {
            minutes: 0,
            x: 0,
            y: -1,
            phase: 0,
        }
    }
    fn increment_phase(&self) -> Self {
        Self {
            minutes: self.minutes,
            x: self.x,
            y: self.y,
            phase: self.phase + 1,
        }
    }
    fn increment_minutes(&mut self) {
        self.minutes += 1;
    }
    fn check_cycle(&self, lcm_rc: i64) -> Self {
        Self {
            minutes: self.minutes % lcm_rc,
            x: self.x,
            y: self.y,
            phase: self.phase,
        }
    }
    fn shift_pos(&self, pos: Point) -> Self {
        Self {
            minutes: self.minutes,
            x: self.x + pos.x,
            y: self.y + pos.y,
            phase: self.phase,
        }
    }
    fn shortest_path_expedition(&mut self, blizzard_vale: &BlizzardVale, num_phases: i64) -> i64 {
        let mut queue: BTreeSet<Self> = BTreeSet::new();
        queue.insert(*self);
        let mut seen: Vec<Self> = Vec::new();
        let mut current_phase = self.phase;
        while let Some(mut current_state) = queue.pop_first() {
            if current_state.phase < current_phase {
                continue;
            }
            current_state.increment_minutes();
            for new_state in Compass::center_and_cardinals()
                .into_iter()
                .map(|c| current_state.shift_pos(Point::from(c)))
            {
                // reached end of phase?
                if (new_state.phase & 1 == 0 && blizzard_vale.is_end_pos(new_state))
                    || (new_state.phase & 1 == 1 && blizzard_vale.is_start_pos(new_state))
                {
                    let phase_end_state = new_state.increment_phase();
                    if phase_end_state.phase == num_phases {
                        *self = phase_end_state;
                        return new_state.minutes;
                    }
                    seen.push(phase_end_state);
                    queue.insert(phase_end_state);
                    current_phase = phase_end_state.phase;
                    continue;
                }
                // check for out of map position and blizzards
                if blizzard_vale.is_out_of_vale(new_state)
                    || blizzard_vale.check_pos_for_blizzards(new_state)
                {
                    continue;
                }
                // check if new_state is cycle of previous state
                let seen_state = new_state.check_cycle(blizzard_vale.lcm_rc);
                if seen.contains(&seen_state) {
                    continue;
                }

                seen.push(seen_state);
                queue.insert(new_state);
            }
        }
        // this should never happen
        -1
    }
}

// So inspired (again) by HyperNeutrino
// https://www.youtube.com/watch?v=R_QWG-cPp_k&list=PLnNm9syGLD3yf-YW-a5XNh1CJN07xr0Kz&index=24

pub fn day_24() -> Result<()> {
    let input = include_str!("../../assets/day_24.txt");
    let blizzard_vale = BlizzardVale::from(input);
    let mut expedition = ExpeditionState::new();
    let result_part1 = expedition.shortest_path_expedition(&blizzard_vale, 1);
    println!("result day 24 part 1: {}", result_part1);
    assert_eq!(result_part1, 274);
    let result_part2 = expedition.shortest_path_expedition(&blizzard_vale, 3);
    println!("result day 24 part 2: {}", result_part2);
    assert_eq!(result_part2, 839);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = "#.######\n\
                           #>>.<^<#\n\
                           #.<..<<#\n\
                           #>v.><>#\n\
                           #<^v^^>#\n\
                           ######.#";
        let blizzard_vale = BlizzardVale::from(input);
        let mut expedition = ExpeditionState::new();
        let result_part1 = expedition.shortest_path_expedition(&blizzard_vale, 1);
        println!("result example day 24 part 1: {}", result_part1);
        assert_eq!(result_part1, 18);
        let result_part2 = expedition.shortest_path_expedition(&blizzard_vale, 3);
        println!("result example day 24 part 2: {}", result_part2);
        assert_eq!(result_part2, 54);

        Ok(())
    }
}

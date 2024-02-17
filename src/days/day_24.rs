//!day_24.rs

use anyhow::Result;
use std::cmp::Ordering;
use std::collections::BTreeSet;

use my_lib::{my_compass::Compass, my_map_point::MapPoint, my_map_two_dim::MyMap2D};

// values taken from ../../assets/day_24.txt
const X: usize = 122;
const Y: usize = 27;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
struct ExpeditionState<const X: usize, const Y: usize> {
    vale: MyMap2D<u8, X, Y>,
    position: MapPoint<X, Y>,
    minutes: usize,
}

impl<const X: usize, const Y: usize> From<&str> for ExpeditionState<X, Y> {
    fn from(value: &str) -> Self {
        let mut vale: MyMap2D<u8, X, Y> = MyMap2D::default();
        for (y, line) in value.lines().enumerate() {
            for (x, v) in line.chars().enumerate() {
                let map_pos = MapPoint::<X, Y>::new(x, y);
                let value = match v {
                    '#' => 255_u8,
                    '.' => 0,
                    '^' => 1,
                    '>' => 2,
                    'v' => 4,
                    '<' => 8,
                    _ => panic!("bad input"),
                };
                vale.set(map_pos, value);
            }
        }
        let position = Self::START_POS;
        *vale.get_mut(position) = Self::EXPEDITION_BIT_MASK;
        Self {
            vale,
            position,
            minutes: 0,
        }
    }
}

impl<const X: usize, const Y: usize> PartialOrd for ExpeditionState<X, Y> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const X: usize, const Y: usize> Ord for ExpeditionState<X, Y> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.minutes.cmp(&other.minutes) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                let delta_self = Self::END_POS.distance(self.position);
                let delta_other = Self::END_POS.distance(other.position);
                match delta_self.cmp(&delta_other) {
                    Ordering::Greater => Ordering::Greater,
                    Ordering::Less => Ordering::Less,
                    Ordering::Equal => {
                        if self.vale == other.vale {
                            return Ordering::Equal;
                        }
                        //self.calculate_hash().cmp(&other.calculate_hash())
                        Ordering::Less
                    }
                }
            },
        }
    }
}

impl<const X: usize, const Y: usize> ExpeditionState<X, Y> {
    const EXPEDITION_BIT_MASK: u8 = 16;
    const START_POS: MapPoint<X, Y> = MapPoint::new_const(1, 0);
    const END_POS: MapPoint<X, Y> = MapPoint::new_const(X - 2, Y - 1);
    fn moving_blizzards(&self) -> Self {
        let mut next_state = *self;
        for (position, blizzards) in self
            .vale
            .iter()
            .filter(|(_, b)| **b > 0 && **b < Self::EXPEDITION_BIT_MASK)
        {
            for blizzard in Compass::from_u8(*blizzards).iter() {
                let bit_mask = blizzard.to_u8();
                let new_position = position.neighbor(*blizzard).unwrap();
                let new_position = match new_position.map_position() {
                    Compass::Center => new_position,
                    Compass::N | Compass::S => position.invert_y(),
                    Compass::E | Compass::W => position.invert_x(),
                    _ => panic!("internal error"),
                };
                *next_state.vale.get_mut(position) -= bit_mask;
                *next_state.vale.get_mut(new_position) += bit_mask;
            }
        }
        next_state.minutes += 1;
        next_state
    }
    fn set_new_position(&self, new_position: MapPoint<X, Y>) -> Self {
        let mut new_position_state = *self;
        *new_position_state.vale.get_mut(new_position_state.position) -= Self::EXPEDITION_BIT_MASK;
        *new_position_state.vale.get_mut(new_position) += Self::EXPEDITION_BIT_MASK;
        new_position_state.position = new_position;
        new_position_state
    }
    fn moving_expedition(&self) -> Vec<Self> {
        let mut new_positions = self
            .position
            .iter_neighbors(Compass::N, true, false, false)
            .filter(|(p, _)| *self.vale.get(*p) == 0)
            .map(|(p, _)| self.set_new_position(p))
            .collect::<Vec<Self>>();
        if *self.vale.get(self.position) == Self::EXPEDITION_BIT_MASK {
            // add state for waiting, if current position stays free of blizzards
            new_positions.push(*self);
        }
        new_positions
    }
    fn shortest_path_expedition(&self) -> usize {
        let mut processing_states: BTreeSet<Self> = BTreeSet::new();
        let mut seen_states: Vec<MyMap2D<u8, X, Y>> = Vec::new();
        processing_states.insert(*self);
        seen_states.push(self.vale);
        while let Some(current_state) = processing_states.pop_first() {
            let moved_blizzards = current_state.moving_blizzards();
            for next_state in moved_blizzards.moving_expedition().iter() {
                if next_state.position == Self::END_POS {
                    return next_state.minutes;
                }
                if seen_states.contains(&next_state.vale) {
                    continue;
                }
                processing_states.insert(*next_state);
                seen_states.push(next_state.vale);
            }
        }
        // if there is no more state to process, while loop ends, which should not happen before END_POS is reached
        0
    }
}

pub fn day_24() -> Result<()> {
    let input = include_str!("../../assets/day_24.txt");
    let expedition = ExpeditionState::<X, Y>::from(input);
    let result_part1 = expedition.shortest_path_expedition();
    println!("result day 24 part 1: {}", result_part1);
    //assert_eq!(result_part1, 4_034);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    const XB: usize = 7;
    const YB: usize = 7;

    const XD: usize = 8;
    const YD: usize = 6;

    #[test]
    fn test_blizzard_moving() {
        let input = "#.#####\n\
                           #.....#\n\
                           #>....#\n\
                           #.....#\n\
                           #...v.#\n\
                           #.....#\n\
                           #####.#";
        let expedition = ExpeditionState::<XB, YB>::from(input);
        eprintln!("{}", expedition.vale);
        let blizzards_moving = expedition.moving_blizzards();
        let blizzards_moving = blizzards_moving.moving_blizzards();
        let blizzards_moving = blizzards_moving.moving_blizzards();
        assert_eq!(*blizzards_moving.vale.get((4, 2).into()), 6);
        eprintln!("{}", blizzards_moving.vale);
        let blizzards_moving = blizzards_moving.moving_blizzards();
        let blizzards_moving = blizzards_moving.moving_blizzards();
        assert_eq!(blizzards_moving.vale, expedition.vale);
        eprintln!("{}", blizzards_moving.vale);
    }

    #[test]
    fn test_example() -> Result<()> {
        let input = "#.######\n\
                           #>>.<^<#\n\
                           #.<..<<#\n\
                           #>v.><>#\n\
                           #<^v^^>#\n\
                           ######.#";
        let expedition = ExpeditionState::<XD, YD>::from(input);
        let result_part1 = expedition.shortest_path_expedition();
        println!("result example day 24 part 1: {}", result_part1);
        assert_eq!(result_part1, 18);
        Ok(())
    }
}

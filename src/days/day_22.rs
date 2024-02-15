//!day_22.rs

use anyhow::Result;
use my_lib::{
    my_compass::Compass,
    my_cube_map::{CubeMap, CubeMapPoint},
    my_map_point::MapPoint,
    my_map_two_dim::MyMap2D,
};

// values taken from ../../assets/day_22.txt
const X: usize = 150;
const Y: usize = 200;
const N: usize = 50;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum Tile {
    #[default]
    Void,
    Free,
    Wall,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            ' ' => Tile::Void,
            '.' => Tile::Free,
            '#' => Tile::Wall,
            _ => panic!("bad input"),
        }
    }
}

trait JungleTrail {
    fn set_start_pos(&mut self);
    fn follow_trail(&mut self);
    fn calc_trail_end_value(&self) -> usize;
}

#[derive(Default)]
struct FlatJungleMap<const X: usize, const Y: usize> {
    map: MyMap2D<Tile, X, Y>,
    trail: Vec<(usize, Option<bool>)>,
    trail_position: MapPoint<X, Y>,
    trail_orientation: Compass,
}

impl<const X: usize, const Y: usize> From<&str> for FlatJungleMap<X, Y> {
    fn from(value: &str) -> Self {
        let (map, trail_str) = value.split_once("\n\n").unwrap();
        let mut trail: Vec<(usize, Option<bool>)> = Vec::new();
        let mut index = 0;
        for steps in trail_str.split(['R', 'L']) {
            index += steps.len();
            let dir: Option<bool> = if index < trail_str.len() {
                match &trail_str[index..=index] {
                    "R" => Some(true),
                    "L" => Some(false),
                    _ => panic!("bad trail input"),
                }
            } else {
                None
            };
            index += 1;
            let steps = steps.parse::<usize>().expect("bad steps input");
            trail.push((steps, dir));
        }
        Self {
            map: MyMap2D::from(map),
            trail,
            ..Default::default()
        }
    }
}

impl<const X: usize, const Y: usize> JungleTrail for FlatJungleMap<X, Y> {
    fn set_start_pos(&mut self) {
        self.trail_position = self
            .map
            .iter_row(0)
            .find(|(_, t)| **t == Tile::Free)
            .unwrap()
            .0;
        self.trail_orientation = Compass::E;
    }
    fn follow_trail(&mut self) {
        for (max_steps, turning) in self.trail.iter() {
            #[cfg(test)]
            eprintln!(
                "trail_position: {}, orientation: {:?}",
                self.trail_position, self.trail_orientation
            );
            if let Some(new_pos) = self
                .trail_position
                .iter_orientation_wrap_around(self.trail_orientation, Compass::Center)
                .filter(|p| *self.map.get(*p) != Tile::Void)
                .skip(1)
                .take(*max_steps)
                .take_while(|p| *self.map.get(*p) != Tile::Wall)
                .last()
            {
                self.trail_position = new_pos;
            }
            if let Some(turn_direction) = turning {
                self.trail_orientation = if *turn_direction {
                    self.trail_orientation.clockwise().clockwise()
                } else {
                    self.trail_orientation.counterclockwise().counterclockwise()
                };
            }
        }
        #[cfg(test)]
        eprintln!(
            "trail_position: {}, orientation: {:?}",
            self.trail_position, self.trail_orientation
        );
    }
    fn calc_trail_end_value(&self) -> usize {
        (self.trail_position.x() + 1) * 4
            + (self.trail_position.y() + 1) * 1_000
            + match self.trail_orientation {
                Compass::E => 0,
                Compass::S => 1,
                Compass::W => 2,
                Compass::N => 3,
                _ => panic!("internal error"),
            }
    }
}

#[derive(Default)]
struct CubicJungle<const N: usize> {
    cube_map: CubeMap<Tile, N>,
    trail: Vec<(usize, Option<bool>)>,
    trail_position: CubeMapPoint<N>,
    trail_orientation: Compass,
}

impl<const N: usize> From<&str> for CubicJungle<N> {
    fn from(value: &str) -> Self {
        let (cube_str, trail_str) = value.split_once("\n\n").unwrap();
        let cube_map: CubeMap<Tile, N> = CubeMap::from(cube_str);
        // read in trail
        let mut trail: Vec<(usize, Option<bool>)> = Vec::new();
        let mut index = 0;
        for steps in trail_str.split(['R', 'L']) {
            index += steps.len();
            let dir: Option<bool> = if index < trail_str.len() {
                match &trail_str[index..=index] {
                    "R" => Some(true),
                    "L" => Some(false),
                    _ => panic!("bad trail input"),
                }
            } else {
                None
            };
            index += 1;
            let steps = steps.parse::<usize>().expect("bad steps input");
            trail.push((steps, dir));
        }
        // return cube jungle
        CubicJungle {
            cube_map,
            trail,
            ..Default::default()
        }
    }
}

impl<const N: usize> JungleTrail for CubicJungle<N> {
    fn set_start_pos(&mut self) {
        self.trail_position.0 = 0;
        self.trail_position.1 = self
            .cube_map
            .get_surface(self.trail_position.0)
            .iter_row(0)
            .find(|(_, t)| **t == Tile::Free)
            .unwrap()
            .0;
        self.trail_orientation = Compass::E;
    }
    fn follow_trail(&mut self) {
        for (max_steps, turning) in self.trail.iter() {
            #[cfg(test)]
            eprintln!(
                "trail_position: {:?}, orientation: {:?}",
                self.cube_map
                    .cube_map_point_to_flat_map_coordinates(&self.trail_position)
                    .unwrap(),
                self.trail_orientation
            );
            if let Some((new_pos, new_orientation, _)) = self
                .cube_map
                .iter_orientation(self.trail_position, self.trail_orientation)
                .skip(1)
                .take(*max_steps)
                .take_while(|(.., v)| **v != Tile::Wall)
                .last()
            {
                self.trail_position = new_pos;
                self.trail_orientation = new_orientation;
            }
            if let Some(turn_direction) = turning {
                self.trail_orientation = if *turn_direction {
                    self.trail_orientation.clockwise().clockwise()
                } else {
                    self.trail_orientation.counterclockwise().counterclockwise()
                };
            }
        }
        #[cfg(test)]
        eprintln!(
            "trail_position: {:?}, orientation: {:?}",
            self.cube_map
                .cube_map_point_to_flat_map_coordinates(&self.trail_position)
                .unwrap(),
            self.trail_orientation
        );
    }
    fn calc_trail_end_value(&self) -> usize {
        let flat_map_trail_end = self
            .cube_map
            .cube_map_point_to_flat_map_coordinates(&self.trail_position)
            .unwrap();
        (flat_map_trail_end.0 + 1) * 4
            + (flat_map_trail_end.1 + 1) * 1_000
            + match self.trail_orientation {
                Compass::E => 0,
                Compass::S => 1,
                Compass::W => 2,
                Compass::N => 3,
                _ => panic!("internal error"),
            }
    }
}

fn explore_trail(mut jungle: impl JungleTrail) -> usize {
    jungle.set_start_pos();
    jungle.follow_trail();
    jungle.calc_trail_end_value()
}

pub fn day_22() -> Result<()> {
    let input = include_str!("../../assets/day_22.txt");
    let flat_jungle_trail = FlatJungleMap::<X, Y>::from(input);
    let result_part1 = explore_trail(flat_jungle_trail);
    println!("result day 22 part 1: {}", result_part1);
    assert_eq!(result_part1, 13_566);

    let cubic_jungle_trail = CubicJungle::<N>::from(input);
    let result_part2 = explore_trail(cubic_jungle_trail);
    println!("result day 22 part 2: {}", result_part2);
    assert_eq!(result_part2, 11_451);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    // values taken from ../../assets/day_22_example.txt
    const XD: usize = 16;
    const YD: usize = 12;
    const ND: usize = 4;

    #[test]
    fn test_example() -> Result<()> {
        let input = include_str!("../../assets/day_22_example.txt");
        let flat_ungle_trail = FlatJungleMap::<XD, YD>::from(input);
        let result_part1 = explore_trail(flat_ungle_trail);
        println!("result example day 22 part 1: {}", result_part1);
        assert_eq!(result_part1, 6_032);

        let cubic_jungle_trail = CubicJungle::<ND>::from(input);
        let result_part2 = explore_trail(cubic_jungle_trail);
        println!("result exa,ple day 22 part 2: {}", result_part2);
        assert_eq!(result_part2, 5_031);

        Ok(())
    }
}

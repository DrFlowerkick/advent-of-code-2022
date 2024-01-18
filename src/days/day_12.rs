//!day_12.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_map_point::MapPoint, my_map_two_dim::MyMap2D};

// taken from ../../assets/day_12.txt
const X: usize = 113;
const Y: usize = 41;

struct Heightmap<const X: usize, const Y: usize> {
    map: MyMap2D<char, X, Y>,
}

impl<const X: usize, const Y: usize> From<&str> for Heightmap<X, Y> {
    fn from(value: &str) -> Self {
        Heightmap {
            map: MyMap2D::from(value),
        }
    }
}

impl<const X: usize, const Y: usize> Heightmap<X, Y> {
    fn travel_shortest_path_from_startpoint(&self) -> usize {
        let (start_point, _) = self.map.iter().find(|(_, c)| **c == 'S').unwrap();
        self.calc_destance_map_from_endpoint()
            .get(start_point)
            .unwrap()
    }
    fn travel_shortest_path_from_lowest_elevation(&self) -> usize {
        let distance_map = self.calc_destance_map_from_endpoint();
        self.map
            .iter()
            .filter(|(_, v)| **v == 'a' || **v == 'S')
            .filter_map(|(p, _)| *distance_map.get(p))
            .min()
            .unwrap()
    }
    fn calc_destance_map_from_endpoint(&self) -> MyMap2D<Option<usize>, X, Y> {
        let mut distance_map: MyMap2D<Option<usize>, X, Y> = MyMap2D::default();
        let (end_point, _) = self.map.iter().find(|(_, c)| **c == 'E').unwrap();
        let filter_fn = Box::new(
            |_: MapPoint<X, Y>,
             value_of_next_cell: &char,
             _: Compass,
             _: MapPoint<X, Y>,
             value_of_current_cell: &char,
             _: usize| {
                match value_of_current_cell {
                    'E' => *value_of_next_cell as u32 >= 'z' as u32 - 1,
                    _ => match value_of_next_cell {
                        'S' => 'a' as u32 >= *value_of_current_cell as u32 - 1,
                        _ => *value_of_next_cell as u32 >= *value_of_current_cell as u32 - 1,
                    },
                }
            },
        );
        for (point, _, distance) in self.map.iter_distance(end_point, filter_fn) {
            distance_map.set(point, Some(distance));
        }
        distance_map
    }
}

pub fn day_12() -> Result<()> {
    let input = include_str!("../../assets/day_12.txt");
    let height_map = Heightmap::<X, Y>::from(input);
    let result_part1 = height_map.travel_shortest_path_from_startpoint();
    println!("result day 12 part 1: {}", result_part1);
    assert_eq!(result_part1, 380);

    let result_part2 = height_map.travel_shortest_path_from_lowest_elevation();
    println!("result day 12 part 2: {}", result_part2);
    assert_eq!(result_part2, 375);
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    const XT: usize = 8;
    const YT: usize = 5;

    #[test]
    fn test_example() -> Result<()> {
        let input = include_str!("../../assets/day_12.txt");
        let x_input = input.lines().next().unwrap().chars().count();
        let y_input = input.lines().count();
        eprintln!("X: {}, Y: {}", x_input, y_input);

        let input = "Sabqponm\n\
                           abcryxxl\n\
                           accszExk\n\
                           acctuvwj\n\
                           abdefghi";
        let height_map = Heightmap::<XT, YT>::from(input);
        eprint!("{}", height_map.map);

        let result_part1 = height_map.travel_shortest_path_from_startpoint();
        println!("result example day 12 part 1: {}", result_part1);
        assert_eq!(result_part1, 31);

        let result_part2 = height_map.travel_shortest_path_from_lowest_elevation();
        println!("result example day 12 part 2: {}", result_part2);
        assert_eq!(result_part2, 29);
        Ok(())
    }
}

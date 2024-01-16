//!day_08.rs

use anyhow::Result;
use my_lib::{my_map_point::MapPoint, my_map_two_dim::MyMap2D};

// taken from ../../assets/day_08.txt
const X: usize = 99;
const Y: usize = 99;

#[derive(Default)]
struct Forest<const X: usize, const Y: usize> {
    trees: MyMap2D<u32, X, Y>,
    visible: MyMap2D<bool, X, Y>,
    scenic_score: MyMap2D<u32, X, Y>,
}

impl<const X: usize, const Y: usize> From<&str> for Forest<X, Y> {
    fn from(value: &str) -> Self {
        let mut f = Self::default();
        for (y, line) in value.lines().enumerate() {
            for (x, t) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
                let position = MapPoint::<X, Y>::new(x, y);
                f.trees.set(position, t);
                // set visibility of edge
                if !position.map_position().is_center() {
                    f.visible.set(position, true);
                }
            }
        }
        f
    }
}

impl<const X: usize, const Y: usize> Forest<X, Y> {
    fn check_visbility(&mut self) {
        for (edge_point, edge_size) in self.trees.iter_edge((0, 0).into(), false) {
            if edge_point.map_position().is_cardinal() {
                let mut max_size = *edge_size;
                for (position, size) in self
                    .trees
                    .iter_orientation(edge_point, edge_point.map_position().flip())
                    .skip(1)
                {
                    *self.visible.get_mut(position) |= *size > max_size;
                    max_size = max_size.max(*size);
                }
            }
        }
    }
    fn num_visible_trees(&self) -> usize {
        self.visible.iter().filter(|(_, v)| **v).count()
    }
    fn calc_scenic_score(&mut self) {
        for (position, pos_size) in self
            .trees
            .iter()
            .filter(|(p, _)| p.map_position().is_center())
        {
            let mut scenic_scores: Vec<u32> = Vec::with_capacity(4);
            for orientation in position.available_cardinal_directions().iter() {
                let mut scenic_score = 0;
                for (_, size) in self.trees.iter_orientation(position, *orientation).skip(1) {
                    scenic_score += 1;
                    if size >= pos_size {
                        break;
                    }
                }
                scenic_scores.push(scenic_score);
            }
            self.scenic_score
                .set(position, scenic_scores.iter().product());
        }
    }
    fn max_scenic_score(&self) -> u32 {
        self.scenic_score.iter().map(|(_, v)| *v).max().unwrap()
    }
}

pub fn day_08() -> Result<()> {
    let input = include_str!("../../assets/day_08.txt");
    let mut forest = Forest::<X, Y>::from(input);
    forest.check_visbility();
    let result_part1 = forest.num_visible_trees();
    println!("result day 08 part 1: {}", result_part1);
    assert_eq!(result_part1, 1_669);
    forest.calc_scenic_score();
    let result_part2 = forest.max_scenic_score();
    println!("result day 08 part 2: {}", result_part2);
    assert_eq!(result_part2, 331_344);
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    const XT: usize = 5;
    const YT: usize = 5;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../assets/day_08.txt");
        eprintln!("X: {}", input.lines().next().unwrap().chars().count());
        eprintln!("Y: {}", input.lines().count());
        let input = "30373\n\
                           25512\n\
                           65332\n\
                           33549\n\
                           35390";
        let mut forest = Forest::<XT, YT>::from(input);
        forest.check_visbility();
        let result_part1 = forest.num_visible_trees();
        println!("result example day 08 part 1: {}", result_part1);
        assert_eq!(result_part1, 21);
        forest.calc_scenic_score();
        let result_part2 = forest.max_scenic_score();
        println!("result example day 08 part 2: {}", result_part2);
        assert_eq!(result_part2, 8);
        Ok(())
    }
}

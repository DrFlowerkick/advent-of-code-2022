//!day_18.rs

use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl From<&str> for Point3D {
    fn from(value: &str) -> Self {
        let mut v_iter = value
            .split(',')
            .map(|c| c.parse::<i32>().expect("bad input"));
        Self {
            x: v_iter.next().unwrap(),
            y: v_iter.next().unwrap(),
            z: v_iter.next().unwrap(),
        }
    }
}

impl Point3D {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
    fn min(&self, other: &Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }
    fn max(&self, other: &Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }
    fn in_range(&self, min: &Self, max: &Self) -> bool {
        min.x <= self.x
            && self.x <= max.x
            && min.y <= self.y
            && self.y <= max.y
            && min.z <= self.z
            && self.z <= max.z
    }
}

fn count_surfaces(points: &[Point3D]) -> (usize, usize) {
    let mut min = points[0];
    let mut max = points[0];
    let dirs = [
        Point3D::new(1, 0, 0),
        Point3D::new(-1, 0, 0),
        Point3D::new(0, 1, 0),
        Point3D::new(0, -1, 0),
        Point3D::new(0, 0, 1),
        Point3D::new(0, 0, -1),
    ];
    let mut n_surfaces = 0;
    for point in points.iter() {
        for dir in dirs.iter() {
            if !points.contains(&point.add(dir)) {
                n_surfaces += 1;
            }
        }
        min = min.min(point);
        max = max.max(point);
    }
    min = min.add(&Point3D::new(-1, -1, -1));
    max = max.add(&Point3D::new(1, 1, 1));
    let mut n_surfaces_outside = 0;
    let mut index = 0;
    let mut seen: Vec<Point3D> = vec![min];
    while index < seen.len() {
        let current_point = seen[index];
        for neighbor in dirs
            .iter()
            .map(|d| current_point.add(d))
            .filter(|p| p.in_range(&min, &max))
        {
            if points.contains(&neighbor) {
                n_surfaces_outside += 1;
            } else if !seen.contains(&neighbor) {
                seen.push(neighbor);
            }
        }
        index += 1;
    }
    (n_surfaces, n_surfaces_outside)
}

pub fn day_18() -> Result<()> {
    let input = include_str!("../../assets/day_18.txt");
    let points: Vec<Point3D> = input.lines().map(Point3D::from).collect();
    let (result_part1, result_part2) = count_surfaces(&points);
    println!("result day 18 part 1: {}", result_part1);
    assert_eq!(result_part1, 4_548);
    println!("result day 18 part 2: {}", result_part2);
    assert_eq!(result_part2, 2_588);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = "2,2,2\n\
                           1,2,2\n\
                           3,2,2\n\
                           2,1,2\n\
                           2,3,2\n\
                           2,2,1\n\
                           2,2,3\n\
                           2,2,4\n\
                           2,2,6\n\
                           1,2,5\n\
                           3,2,5\n\
                           2,1,5\n\
                           2,3,5";
        let points: Vec<Point3D> = input.lines().map(Point3D::from).collect();
        let (result_part1, result_part2) = count_surfaces(&points);
        println!("result example day 18 part 1: {}", result_part1);
        assert_eq!(result_part1, 64);

        println!("result example day 18 part 2: {}", result_part2);
        assert_eq!(result_part2, 58);
        Ok(())
    }
}

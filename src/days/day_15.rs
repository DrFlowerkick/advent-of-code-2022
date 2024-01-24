//!day_15.rs

use anyhow::Result;
use my_lib::my_geometry::{Diamond, Line};
use my_lib::my_point::Point;

#[derive(Debug, Clone, Copy)]
struct SensorBeacon {
    sensor: Diamond,
    beacon: Point,
}

impl From<&str> for SensorBeacon {
    fn from(value: &str) -> Self {
        let (sensor, beacon) = value
            .strip_prefix("Sensor at x=")
            .unwrap()
            .split_once(": closest beacon is at x=")
            .unwrap();
        let sensor = sensor
            .split_once(", y=")
            .map(|(x, y)| {
                Point::from((
                    x.parse::<i32>().expect("bad input"),
                    y.parse::<i32>().expect("bad input"),
                ))
            })
            .unwrap();
        let beacon = beacon
            .split_once(", y=")
            .map(|(x, y)| {
                Point::from((
                    x.parse::<i32>().expect("bad input"),
                    y.parse::<i32>().expect("bad input"),
                ))
            })
            .unwrap();
        Self {
            sensor: Diamond::new(sensor, sensor.delta(beacon)),
            beacon,
        }
    }
}

fn calc_scanned_positions_of_row(sensor_beacons: &[SensorBeacon], row: i32) -> i32 {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut sensor_beacons_in_row: Vec<Point> = Vec::with_capacity(sensor_beacons.len());
    let row_line = Line::new(0, 1, -row);
    for sb in sensor_beacons.iter() {
        let mut intersection_x: Vec<i32> = sb
            .sensor
            .diamond_line_intersection(&row_line)
            .iter()
            .map(|p| p.x)
            .collect();
        if intersection_x.len() == 2 {
            intersection_x.sort();
            min_x = min_x.min(intersection_x[0]);
            max_x = max_x.max(intersection_x[1]);
        }
        if sb.sensor.get_center().y == row
            && !sensor_beacons_in_row.contains(&sb.sensor.get_center())
        {
            sensor_beacons_in_row.push(sb.sensor.get_center());
        }
        if sb.beacon.y == row && !sensor_beacons_in_row.contains(&sb.beacon) {
            sensor_beacons_in_row.push(sb.beacon);
        }
    }
    let mut count_sensor_beacons_in_range = 0;
    for sb in sensor_beacons_in_row.iter() {
        if sb.x >= min_x && sb.x <= max_x {
            count_sensor_beacons_in_range += 1;
        }
    }
    max_x - min_x + 1 - count_sensor_beacons_in_range
}

fn find_distress_beacon(sensor_beacons: &[SensorBeacon], max_range: i32, x_factor: i64) -> i64 {
    let mut distress_beacons: Vec<Point> = Vec::new();
    for (i, sb_1) in sensor_beacons.iter().enumerate() {
        for sb_2 in sensor_beacons.iter().skip(i + 1) {
            let d1 = sb_1.sensor.stretch(1);
            let d2 = sb_2.sensor.stretch(1);
            for db in d1.diamond_intersection(&d2).iter() {
                if sensor_beacons.iter().any(|sb| sb.sensor >= *db) {
                    continue;
                }
                if !(0..max_range).contains(&db.x) || !(0..max_range).contains(&db.y) {
                    continue;
                }
                // found possible distress beacon
                if !distress_beacons.contains(db) {
                    distress_beacons.push(*db);
                }
            }
        }
    }
    assert_eq!(distress_beacons.len(), 1);
    (distress_beacons[0].x as i64) * x_factor + distress_beacons[0].y as i64
}

pub fn day_15() -> Result<()> {
    let input = include_str!("../../assets/day_15.txt");
    let sensor_beacons: Vec<SensorBeacon> = input.lines().map(SensorBeacon::from).collect();
    let row = 2_000_000;
    let result_part1 = calc_scanned_positions_of_row(&sensor_beacons, row);
    println!("result day 15 part 1: {}", result_part1);
    assert_eq!(result_part1, 5_112_034);

    let max_range = 4_000_000;
    let x_factor = 4_000_000;
    let result_part2 = find_distress_beacon(&sensor_beacons, max_range, x_factor);
    println!("result day 15 part 2: {}", result_part2);
    assert_eq!(result_part2, 13_172_087_230_812);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = include_str!("../../assets/day_15_example.txt");
        let sensor_beacons: Vec<SensorBeacon> = input.lines().map(SensorBeacon::from).collect();
        let row = 10;
        let result_part1 = calc_scanned_positions_of_row(&sensor_beacons, row);
        println!("result example day 15 part 1: {}", result_part1);
        assert_eq!(result_part1, 26);

        let max_range = 20;
        let x_factor = 4_000_000;
        let result_part2 = find_distress_beacon(&sensor_beacons, max_range, x_factor);
        println!("result example day 15 part 2: {}", result_part2);
        assert_eq!(result_part2, 56_000_011);
        Ok(())
    }
}

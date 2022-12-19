use interval::IntervalSet;
use itertools::Itertools;
use regex::Regex;
use interval::{ops::Range};
use interval::interval_set::{ToIntervalSet};
use gcollections::ops::*;

pub struct Position2D {
    pub x: i32,
    pub y: i32
}

impl Position2D {
    fn manhattan_distance(&self, other: &Position2D) -> i32 {
        return (self.x - other.x).abs() + (self.y - other.y).abs();
    }
}

struct Sensor {
    position: Position2D,
    nearest_beacon: Position2D
}

impl Sensor {
    fn find_excluded_positions_at_y(&self, y: i32) -> Option<(i32, i32)> {
        let midpoint = self.position.x;
        let distance_to_y = (self.position.y - y).abs();
        let width_at_this_y = self.position.manhattan_distance(&self.nearest_beacon);
        let width_at_target_y = width_at_this_y - distance_to_y;
        if width_at_target_y < 0 {
            return None
        } else {
            let interval = (midpoint - width_at_target_y, midpoint + width_at_target_y);
            return Some(interval);
        }
    }
}

fn parse_input(input: &str) -> Vec<Sensor> {
    let regex = r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$";
    let re = Regex::new(regex).unwrap();
    return input.lines().map(|line| {
        let captures = re.captures(line).unwrap();
        return Sensor {
            position: Position2D { x: captures[1].parse().unwrap(), y: captures[2].parse().unwrap() },
            nearest_beacon: Position2D { x: captures[3].parse().unwrap(), y: captures[4].parse().unwrap() }
        };
    }).collect_vec();
}

pub fn solve(input: &str, max_y: i32) -> usize {
    let sensors = parse_input(input);

    // Check every single y level in range
    for y in 0..=max_y {
        let mut interval_set = IntervalSet::new(0, 0);
        sensors
            .iter()
            .filter_map(|sensor| sensor.find_excluded_positions_at_y(y))
            .for_each(|interval| {
                interval_set = interval_set.union(&interval.to_interval_set());
            });

        if interval_set.interval_count() > 1 {
            let x = interval_set.iter().next().unwrap().upper() + 1;
            return x as usize * 4000000 + y as usize;
        }
    }

    unreachable!();
}

pub mod tests {
    use criterion::Criterion;
    use std::path::Path;

    #[test]
    fn verify_example() {
        assert_eq!(super::solve(super::super::INPUT_EXAMPLE, 20), 56000011);
    }

    #[test]
    fn verify_solution() {
        assert_eq!(super::solve(super::super::INPUT, 4000000), 10457634860779);
    }

    pub fn benchmark(c: &mut Criterion) {
        let path = Path::new(file!());
        let day = path.parent().unwrap().to_str().unwrap();
        let solution = path.file_stem().unwrap().to_str().unwrap();
        let id = day.to_owned() + "_" + solution; // dayX_partY_solveZ
        c.bench_function( &id, |b| b.iter(|| super::solve(super::super::INPUT, 56000011)));
    }
}
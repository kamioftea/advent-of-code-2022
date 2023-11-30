//! This is my solution for [Advent of Code - Day 15 - _Beacon Exclusion Zone_](https://adventofcode.com/2022/day/15)
//!
//!

use std::fs;
use itertools::Itertools;

type Range = (isize, isize);
type Position = (isize, isize);

#[derive(Eq, PartialEq, Debug)]
struct Sensor {
    sensor: Position,
    beacon: Position,
    zone_size: isize,
}

impl Sensor {
    fn x_coverage_for(&self, y: isize) -> Option<Range> {
        match self.zone_size - isize::try_from(y.abs_diff(self.sensor.1)).unwrap() {
            diff if diff < 0 => None,
            diff => Some(
                (self.sensor.0 - diff, self.sensor.0 + diff)
            )
        }
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-15-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 15.
pub fn run() {
    let contents = fs::read_to_string("res/day-15-input").expect("Failed to read file");
    let sensors = parse_input(&contents);

    println!(
        "On row 2,000,000 there are {} spaces known to be free of sensors.",
        coverage_for(&sensors, 2_000_000)
    );

    println!(
        "The tuning frequency is: {}",
        tuning_frequency(&sensors, (0, 4_000_000))
    );
}

fn parse_input(input: &String) -> Vec<Sensor> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Sensor {
    let mut parts = line.split_whitespace().dropping(2);
    let sx = parts.next().map(parse_digits).unwrap();
    let sy = parts.next().map(parse_digits).unwrap();

    parts = parts.dropping(4);
    let bx = parts.next().map(parse_digits).unwrap();
    let by = parts.next().map(parse_digits).unwrap();

    Sensor {
        sensor: (sx, sy),
        beacon: (bx, by),
        zone_size: manhatten_distance((sx, sy), (bx, by)),
    }
}

fn parse_digits(part: &str) -> isize {
    let number_str: String = part.chars().filter(|&c| c.is_digit(10) || c == '-').collect();
    number_str.parse::<isize>().unwrap()
}

fn manhatten_distance((y_a, x_a): (isize, isize), (y_b, x_b): (isize, isize)) -> isize {
    (y_a.abs_diff(y_b) + x_a.abs_diff(x_b)).try_into().unwrap()
}

fn coverage_for(sensors: &Vec<Sensor>, y: isize) -> usize {
    let coverage: usize =
        ranges_for(sensors, y)
            .into_iter()
            .map(|(start, end)| start.abs_diff(end) + 1)
            .sum();

    let beacon_count =
        sensors.into_iter()
               .filter(|s| s.beacon.1 == y)
               .map(|s| s.beacon.0)
               .unique()
               .count();

    coverage - beacon_count
}

fn ranges_for(sensors: &Vec<Sensor>, y: isize) -> Vec<Range> {
    sensors.into_iter()
           .flat_map(|s| s.x_coverage_for(y))
           .fold(
               Vec::new(),
               |acc: Vec<Range>, next_range: Range| {
                   let mut new: Vec<Range> = Vec::new();
                   let mut current = next_range;
                   for existing_range in acc.into_iter() {
                       match try_merge(&current, &existing_range) {
                           Some(merged) => { current = merged; }
                           None => { new.push(existing_range); }
                       }
                   }
                   new.push(current);
                   new
               },
           )
}

fn try_merge(&(s_a, e_a): &Range, &(s_b, e_b): &Range) -> Option<Range> {
    if s_a <= e_b && e_a >= s_b {
        Some((s_a.min(s_b), e_a.max(e_b)))
    } else {
        None
    }
}

fn tuning_frequency(sensors: &Vec<Sensor>, bounds: Range) -> isize {
    let (y, line_with_gap) = (bounds.0..=bounds.1)
        .map(|y| (y, ranges_for(&sensors, y)))
        .filter(|(_, ranges)| ranges.len() > 1)
        .next()
        .unwrap();

    let x = line_with_gap[0].1 + 1;

    4_000_000 * x + y
}

#[cfg(test)]
mod tests {
    use crate::day_15::{coverage_for, parse_input, Sensor, tuning_frequency};

    fn sample_sensors() -> Vec<Sensor> {
        vec![
            Sensor { sensor: (2, 18), beacon: (-2, 15), zone_size: 7 },
            Sensor { sensor: (9, 16), beacon: (10, 16), zone_size: 1 },
            Sensor { sensor: (13, 2), beacon: (15, 3), zone_size: 3 },
            Sensor { sensor: (12, 14), beacon: (10, 16), zone_size: 4 },
            Sensor { sensor: (10, 20), beacon: (10, 16), zone_size: 4 },
            Sensor { sensor: (14, 17), beacon: (10, 16), zone_size: 5 },
            Sensor { sensor: (8, 7), beacon: (2, 10), zone_size: 9 },
            Sensor { sensor: (2, 0), beacon: (2, 10), zone_size: 10 },
            Sensor { sensor: (0, 11), beacon: (2, 10), zone_size: 3 },
            Sensor { sensor: (20, 14), beacon: (25, 17), zone_size: 8 },
            Sensor { sensor: (17, 20), beacon: (21, 22), zone_size: 6 },
            Sensor { sensor: (16, 7), beacon: (15, 3), zone_size: 5 },
            Sensor { sensor: (14, 3), beacon: (15, 3), zone_size: 1 },
            Sensor { sensor: (20, 1), beacon: (15, 3), zone_size: 7 },
        ]
    }

    #[test]
    fn can_parse() {
        let sample_input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3".to_string();

        assert_eq!(
            parse_input(&sample_input),
            sample_sensors()
        )
    }

    #[test]
    fn can_get_coverage() {
        assert_eq!(coverage_for(&sample_sensors(), 9), 25);
        assert_eq!(coverage_for(&sample_sensors(), 10), 26);
        assert_eq!(coverage_for(&sample_sensors(), 11), 28);
    }

    #[test]
    fn can_find_tuning_frequecy() {
        assert_eq!(
            tuning_frequency(&sample_sensors(), (0, 20)),
            56_000_011
        )
    }
}

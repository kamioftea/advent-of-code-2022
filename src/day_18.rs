//! This is my solution for [Advent of Code - Day 18 - _Title_](https://adventofcode.com/2022/day/18)
//!
//!

use std::collections::{HashSet, VecDeque};
use std::fs;
use itertools::Itertools;

type Coordinate = (isize, isize, isize);

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-18-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 18.
pub fn run() {
    let contents = fs::read_to_string("res/day-18-input").expect("Failed to read file");
    let coords = parse_input(&contents);

    println!(
        "The surface area is {}.",
        get_surface_area(&coords)
    );

    println!(
        "The external surface area is {}.",
        get_external_surface_area(&coords)
    );
}

fn parse_input(input: &String) -> Vec<Coordinate> {
    input.trim().lines().map(parse_coordinate).collect()
}

fn parse_coordinate(line: &str) -> Coordinate {
    let mut values = line.split(',').flat_map(|s| s.parse::<isize>());

    (values.next().unwrap(), values.next().unwrap(), values.next().unwrap())
}

fn get_surface_area(coords: &Vec<Coordinate>) -> usize {
    let mut surface_area = 0;
    let mut coords_seen: HashSet<Coordinate> = HashSet::new();
    let deltas = vec![
        (1, 0, 0),
        (0, 1, 0),
        (0, 0, 1),
    ];

    for &(x, y, z) in coords {
        surface_area += 6;
        coords_seen.insert((x, y, z));
        for &(dx, dy, dz) in deltas.iter() {
            if coords_seen.contains(&(x + dx, y + dy, z + dz)) {
                surface_area -= 2;
            }

            if coords_seen.contains(&(x - dx, y - dy, z - dz)) {
                surface_area -= 2;
            }
        }
    }

    surface_area
}

fn get_external_surface_area(coords: &Vec<Coordinate>) -> usize {
    let (x_min, x_max) = coords.iter().map(|&c| c.0).minmax().into_option().unwrap();
    let (y_min, y_max) = coords.iter().map(|&c| c.1).minmax().into_option().unwrap();
    let (z_min, z_max) = coords.iter().map(|&c| c.2).minmax().into_option().unwrap();

    let coord_set: HashSet<Coordinate> = coords.into_iter().map(|&c| c).collect();
    let mut visited: HashSet<Coordinate> = HashSet::new();
    let mut surface_area = 0;

    let mut to_process: VecDeque<Coordinate> = VecDeque::new();
    to_process.push_back((x_min - 1, y_min - 1, z_min - 1));

    let deltas = vec![
        (1, 0, 0),
        (0, 1, 0),
        (0, 0, 1),
    ];

    while let Some((x, y, z)) = to_process.pop_front() {
        if visited.contains(&(x, y, z)) {
            continue;
        }

        if coord_set.contains(&(x, y, z)) {
            surface_area += 1;
            continue;
        }

        if x < x_min - 1 || x > x_max + 1
            || y < y_min - 1 || y > y_max + 1
            || z < z_min - 1|| z > z_max + 1 {
            continue;
        }

        visited.insert((x, y, z));

        for &(dx, dy, dz) in deltas.iter() {
            to_process.push_back((x + dx, y + dy, z + dz));
            to_process.push_back((x - dx, y - dy, z - dz));
        }
    }

    surface_area
}

#[cfg(test)]
mod tests {
    use crate::day_18::{Coordinate, get_external_surface_area, get_surface_area, parse_input};

    fn sample_coords() -> Vec<Coordinate> {
        vec![
            (2, 2, 2),
            (1, 2, 2),
            (3, 2, 2),
            (2, 1, 2),
            (2, 3, 2),
            (2, 2, 1),
            (2, 2, 3),
            (2, 2, 4),
            (2, 2, 6),
            (1, 2, 5),
            (3, 2, 5),
            (2, 1, 5),
            (2, 3, 5),
        ]
    }

    #[test]
    fn can_parse() {
        let input = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5".to_string();

        assert_eq!(parse_input(&input), sample_coords());
    }

    #[test]
    fn can_get_surface_area() {
        assert_eq!(get_surface_area(&sample_coords()), 64)
    }

    #[test]
    fn can_get_external_surface_area() {
        assert_eq!(get_external_surface_area(&sample_coords()), 58)
    }
}

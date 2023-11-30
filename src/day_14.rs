//! This is my solution for [Advent of Code - Day 14 - _Title_](https://adventofcode.com/2022/day/14)
//!
//!

use std::collections::HashSet;
use std::fs;

type Coordinates = (isize, isize);

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-14-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 14.
pub fn run() {
    let contents = fs::read_to_string("res/day-14-input").expect("Failed to read file");
    let (mut map, deepest_point) = parse_input(&contents);

    let (part_1, part_2) = count_added_sand(&mut map, deepest_point);
    println!(
        "Before reaching the floor, {} units of sand were added",
        part_1
    );

    println!(
        "Before blocking the inlet, {} units of sand were added",
        part_2
    );
}

fn parse_input(input: &String) -> (HashSet<Coordinates>, isize) {
    let mut points = HashSet::new();
    let mut deepest = 0;

    let points_list = input
        .lines()
        .map(parse_line)
        .flat_map(line_to_points);

    for (x, y) in points_list {
        points.insert((x,y));
        if y > deepest {
            deepest = y
        }
    }

    (points, deepest)
}

fn parse_line(line: &str) -> Vec<Coordinates> {
    line.split(" -> ")
        .map(parse_coordinate)
        .collect()
}

fn parse_coordinate(coordinate: &str) -> Coordinates {
    let (x, y) = coordinate.split_once(',').unwrap();

    (
        x.parse::<isize>().unwrap(),
        y.parse::<isize>().unwrap()
    )
}

fn line_to_points(line: Vec<Coordinates>) -> Vec<Coordinates> {
    line.windows(2)
        .flat_map(|seg| segment_to_points(seg))
        .collect()
}

fn segment_to_points(segment: &[Coordinates]) -> Vec<Coordinates> {
    let (start_x, start_y) = segment[0];
    let (end_x, end_y) = segment[1];

    if start_x == end_x {
        (start_y.min(end_y)..=start_y.max(end_y))
            .map(|y| (start_x, y))
            .collect()
    } else if start_y == end_y {
        (start_x.min(end_x)..=start_x.max(end_x))
            .map(|x| (x, start_y))
            .collect()
    } else {
        unreachable!()
    }
}

fn count_added_sand(map: &mut HashSet<Coordinates>, deepest_point: isize) -> (usize, usize) {
    let mut added_sand_to_floor = None;
    let mut added_sand = 0;

    let mut path : Vec<Coordinates> = vec![(500, 0)];

    while let Some((x, y)) = path.pop() {
        if y == deepest_point && added_sand_to_floor.is_none() {
            added_sand_to_floor = Some(added_sand);
        }

        let maybe_next =
            [(x, y + 1), (x-1, y+1), (x+1, y+1)].into_iter()
                .find(|&(x1, y1)| !map.contains(&(x1, y1)) && y1 < deepest_point + 2);

        match maybe_next {
            Some(c) => {
                path.push((x, y));
                path.push(c);
            }
            None => {
                map.insert((x, y));
                added_sand = added_sand + 1;
            }
        }
    }

    (added_sand_to_floor.unwrap(), added_sand)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::day_14::{Coordinates, count_added_sand, parse_input};

    fn sample_map() -> HashSet<Coordinates> {
        vec![
            (498, 4), (498, 5), (498, 6),
            (497, 6), (496, 6),
            (503, 4), (502, 4),
            (502, 5), (502, 6), (502, 7), (502, 8), (502, 9),
            (501, 9), (501, 9), (500, 9), (499, 9), (498, 9), (497, 9), (496, 9), (495, 9), (494, 9),
        ].into_iter().collect::<HashSet<Coordinates>>()
    }

    #[test]
    fn can_parse() {
        let input =
            "498,4 -> 498,6 -> 496,6\n\
             503,4 -> 502,4 -> 502,9 -> 494,9".to_string();

        assert_eq!(
            parse_input(&input),
            (sample_map(), 9)
        )

    }

    #[test]
    fn can_count_added_sand() {
        assert_eq!(count_added_sand(&mut sample_map(), 9), (24, 93))
    }
}

//! This is my solution for [Advent of Code - Day 23 - _Title_](https://adventofcode.com/2022/day/23)
//!
//!

use std::collections::{HashMap, HashSet};
use std::fs;
use itertools::Itertools;

type Coordiantes = (isize, isize);

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-23-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 23.
pub fn run() {
    let contents = fs::read_to_string("res/day-23-input").expect("Failed to read file");
    let map = parse_input(&contents);

    println!(
        "After 10 rounds there are {} empty spaces",
        get_space_after_rounds(&map, 10)
    );

    println!(
        "It takes {} rounds until the elves stabilise",
        rounds_until_stable(&map)
    );
}

fn parse_input(input: &String) -> HashSet<Coordiantes> {
    let mut elves = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, chr) in line.chars().enumerate() {
            if chr == '#' {
                elves.insert((x as isize, y as isize));
            }
        }
    }

    elves
}

fn bounds(map: &HashSet<Coordiantes>) -> (Coordiantes, Coordiantes) {
    let (min_x, max_x) = map.iter().map(|&(x, _)| x).minmax().into_option().unwrap();
    let (min_y, max_y) = map.iter().map(|&(_, y)| y).minmax().into_option().unwrap();

    ((min_x, min_y), (max_x, max_y))
}

#[allow(dead_code)]
fn print(map: &HashSet<Coordiantes>) -> String {
    let mut lines: Vec<String> = Vec::new();

    let ((min_x, min_y), (max_x, max_y)) = bounds(map);
    for y in min_y..=max_y {
        lines.push(
            (min_x..=max_x)
                .map(|x| if map.contains(&(x, y)) { '#' } else { '.' })
                .join("")
        )
    }

    lines.join("\n")
}

fn iterate(map: &HashSet<Coordiantes>, round: usize) -> HashSet<Coordiantes> {
    let mut moves: HashMap<Coordiantes, Coordiantes> = HashMap::new();
    let mut next_map: HashSet<Coordiantes> = HashSet::new();

    let order: Vec<usize> = [0, 4, 6, 2].into_iter().cycle().dropping(round % 4).take(4).collect();

    for &elf in map {
        let surrounds = get_surrounds(&elf, &map);

        if surrounds.iter().all(|&(_, has_elf)| has_elf == false) {
            next_map.insert(elf);
            continue;
        }

        let mut maybe_new_pos = None;

        for start_index in &order {
            if surrounds.iter().cycle().dropping(*start_index).take(3).all(|&(_, has_elf)| has_elf == false) {
                maybe_new_pos = surrounds.get(*start_index + 1);
                break;
            }
        }

        if let Some(&(new_pos, _)) = maybe_new_pos {
            match moves.get(&new_pos) {
                Some(&old_pos) => {
                    // Undo conflicting move
                    next_map.remove(&new_pos);
                    next_map.insert(old_pos);

                    next_map.insert(elf);
                }
                None => {
                    next_map.insert(new_pos);
                    moves.insert(new_pos, elf);
                }
            }
        }
        else {
            next_map.insert(elf);
        }
    }

    next_map
}

fn get_surrounds((x, y): &Coordiantes, map: &HashSet<Coordiantes>) -> Vec<(Coordiantes, bool)> {
    let deltas = [(-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0)];

    deltas
        .into_iter()
        .map(|(dx, dy)| (
            (x + dx, y + dy),
            map.contains(&(x + dx, y + dy))
        ))
        .collect()
}

fn get_space_after_rounds(map: &HashSet<Coordiantes>, rounds: usize) -> usize {
    let updated = (0..rounds).fold(
        map.clone(),
        |acc, round| iterate(&acc, round)
    );

    calculate_space(&updated)
}

fn calculate_space(map: &HashSet<Coordiantes>) -> usize {
    let ((min_x, min_y), (max_x, max_y)) = bounds(map);
    let area = (max_x + 1 - min_x) as usize * (max_y + 1 - min_y) as usize;

    area - map.len()
}

fn rounds_until_stable(map: &HashSet<Coordiantes>) -> usize {
    let mut round = 0;
    let mut current = map.clone();

    loop {
        let next = iterate(&current, round);
        round += 1;

        if next == current {
            return round
        }

        current = next;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::day_23::{Coordiantes, get_space_after_rounds, iterate, parse_input, rounds_until_stable};

    fn small_sample_elves() -> HashSet<Coordiantes> {
        vec![
            (2, 1), (3, 1),
            (2, 2),
            (2, 4), (3, 4),
        ].into_iter().collect()
    }

    fn medium_sample_elves() -> HashSet<Coordiantes> {
        vec![
            (4, 0),
            (2, 1), (3, 1), (4, 1), (6, 1),
            (0, 2), (4, 2), (6, 2),
            (1, 3), (5, 3), (6, 3),
            (0, 4), (2, 4), (3, 4), (4, 4),
            (0, 5), (1, 5), (3, 5), (5, 5), (6, 5),
            (1, 6), (4, 6),
        ].into_iter().collect()
    }

    #[test]
    fn can_parse() {
        let small = ".....
..##.
..#..
.....
..##.
.....".to_string();

        assert_eq!(parse_input(&small), small_sample_elves());

        let medium = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..".to_string();

        assert_eq!(parse_input(&medium), medium_sample_elves());
    }

    #[test]
    fn can_iterate() {
        let expected_round_1 = "..##.
.....
..#..
...#.
..#..
......".to_string();

        let expected_round_2 = ".....
..##.
.#...
....#
.....
..#..".to_string();

        let expected_round_3 = "..#..
....#
#....
....#
.....
..#..".to_string();

        let round_1 = iterate(&small_sample_elves(), 0);
        let round_2 = iterate(&round_1, 1);
        let round_3 = iterate(&round_2, 2);
        let round_4 = iterate(&round_3, 3);

        assert_eq!(round_1, parse_input(&expected_round_1));
        assert_eq!(round_2, parse_input(&expected_round_2));
        assert_eq!(round_3, parse_input(&expected_round_3));
        assert_eq!(round_4, parse_input(&expected_round_3));
    }

    #[test]
    fn can_find_space_after_ten_rounds() {
        assert_eq!(
            get_space_after_rounds(&medium_sample_elves(), 10),
            110
        )
    }

    #[test]
    fn can_find_stable_state() {
        assert_eq!(rounds_until_stable(&small_sample_elves()), 4);
        assert_eq!(rounds_until_stable(&medium_sample_elves()), 20);
    }
}

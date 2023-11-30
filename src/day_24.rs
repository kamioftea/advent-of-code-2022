//! This is my solution for [Advent of Code - Day 24 - _Title_](https://adventofcode.com/2022/day/24)
//!
//!

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs;
use itertools::Itertools;
use crate::day_24::Direction::{DOWN, LEFT, RIGHT, UP};

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(UP),
            '>' => Ok(RIGHT),
            'v' => Ok(DOWN),
            '<' => Ok(LEFT),
            _ => Err(())
        }
    }
}

type Blizzard = (usize, usize, Direction);
type Position = (usize, usize);

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct State {
    pos: Position,
    dist: usize,
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
             .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-24-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 24.
pub fn run() {
    let contents = fs::read_to_string("res/day-24-input").expect("Failed to read file");
    let (blizzards, goal) = parse_input(&contents);

    println!(
        "The shortest path is {} minutes",
        find_shortest_path_single(&blizzards, goal)
    );

    println!(
        "The shortest path when returning for snacks is {} minutes",
        find_shortest_path_returning(&blizzards, goal)
    );
}

fn parse_input(input: &String) -> (HashSet<Blizzard>, Position) {
    let mut blizzards = HashSet::new();

    for (y, line) in input.lines().dropping(1).enumerate() {
        for (x, chr) in line.chars().dropping(1).enumerate() {
            if let Ok(dir) = Direction::try_from(chr) {
                blizzards.insert((x, y, dir));
            }
        }
    }

    let goal_x = input.lines().next().unwrap().len() - 3;
    let goal_y = input.lines().count() - 3;

    (blizzards, (goal_x, goal_y))
}

fn manhatten_distance(&(y_a, x_a): &Position, &(y_b, x_b): &Position) -> usize {
    y_a.abs_diff(y_b) + x_a.abs_diff(x_b)
}


fn find_shortest_path(
    blizzards: &HashSet<Blizzard>,
    start_pos: Position,
    start_dist: usize,
    goal: Position,
    bounds: Position,
) -> usize {
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    // Could wait indefinitely at the entrance use this to track
    // when a wait of n has been added to the heap
    let mut wait_added = start_dist;
    let max_wait = bounds.0 * bounds.1 + start_dist;
    let mut seen: HashSet<State> = HashSet::new();

    heap.push(State {
        pos: start_pos,
        dist: start_dist + 1,
        cost: manhatten_distance(&start_pos, &goal) + 1,
    });

    let deltas = vec![
        (0, 0), (0, -1), (1, 0), (0, 1), (-1, 0),
    ];

    while let Some(State { dist, pos, .. }) = heap.pop() {
        while wait_added < dist && wait_added < max_wait {
            wait_added += 1;
            heap.push(State {
                pos: start_pos,
                dist: wait_added + 1,
                cost: manhatten_distance(&start_pos, &goal) + wait_added + 1,
            });
        }

        if !is_clear(blizzards, pos, dist, &bounds) {
            continue;
        }

        if pos == goal {
            return dist + 1;
        }

        for nex_pos in apply_deltas(pos, &deltas, &bounds) {
            let next_state = State {
                pos: nex_pos,
                dist: dist + 1,
                cost: manhatten_distance(&nex_pos, &goal) + dist + 1,
            };

            if seen.insert(next_state) {
                heap.push(next_state);
            }
        }
    }

    unreachable!("Failed to find path")
}

fn apply_deltas((x, y): Position, deltas: &Vec<(isize, isize)>, &(max_x, max_y): &Position) -> Vec<Position> {
    deltas
        .into_iter()
        .flat_map(|(dx, dy)| {
            let x1 = x as isize + dx;
            let y1 = y as isize + dy;
            if x1 >= 0 && x1 < max_x as isize && y1 >= 0 && y1 < max_y as isize {
                Some((x1 as usize, y1 as usize))
            } else {
                None
            }
        })
        .collect()
}

fn is_clear(
    blizzards: &HashSet<Blizzard>,
    (pos_x, pos_y): Position,
    distance: usize,
    &(wrap_x, wrap_y): &Position,
) -> bool {
    let intersecting_blizzards = [
        (wrapping_add(pos_x, distance as isize, wrap_x), pos_y, LEFT),
        (wrapping_add(pos_x, -(distance as isize), wrap_x), pos_y, RIGHT),
        (pos_x, wrapping_add(pos_y, distance as isize, wrap_y), UP),
        (pos_x, wrapping_add(pos_y, -(distance as isize), wrap_y), DOWN),
    ];

    intersecting_blizzards
        .into_iter()
        .all(|blizzard| !blizzards.contains(&blizzard))
}

fn wrapping_add(num: usize, delta: isize, wrap: usize) -> usize {
    let mut value = (num as isize + delta) % wrap as isize;
    if value < 0 {
        value += wrap as isize
    }

    value as usize
}

fn find_shortest_path_single(blizzards: &HashSet<Blizzard>, goal: Position) -> usize {
    let bounds = (goal.0 + 1, goal.1 + 1);
    find_shortest_path(blizzards, (0, 0), 0, goal, bounds)
}

fn find_shortest_path_returning(blizzards: &HashSet<Blizzard>, goal: Position) -> usize {
    let bounds = (goal.0 + 1, goal.1 + 1);
    let first_trip = find_shortest_path(blizzards, (0, 0), 0, goal, bounds);
    let return_trip = find_shortest_path(blizzards, goal, first_trip, (0, 0), bounds);

    find_shortest_path(blizzards, (0, 0), return_trip, goal, bounds)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::day_24::{Blizzard, find_shortest_path, find_shortest_path_returning, find_shortest_path_single, is_clear, parse_input};
    use crate::day_24::Direction::{DOWN, LEFT, RIGHT, UP};

    fn sample_blizzards() -> HashSet<Blizzard> {
        vec![
            (0, 0, RIGHT),
            (1, 0, RIGHT),
            (3, 0, LEFT),
            (4, 0, UP),
            (5, 0, LEFT),
            //
            (1, 1, LEFT),
            (4, 1, LEFT),
            (5, 1, LEFT),
            //
            (0, 2, RIGHT),
            (1, 2, DOWN),
            (3, 2, RIGHT),
            (4, 2, LEFT),
            (5, 2, RIGHT),
            //
            (0, 3, LEFT),
            (1, 3, UP),
            (2, 3, DOWN),
            (3, 3, UP),
            (4, 3, UP),
            (5, 3, RIGHT),
        ].into_iter().collect()
    }

    #[test]
    fn can_parse() {
        let input = "#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#".to_string();

        let expected: HashSet<Blizzard> =
            vec![(0, 1, RIGHT), (3, 3, DOWN)]
                .into_iter()
                .collect();

        let (actual, goal) = parse_input(&input);

        assert_eq!(goal, (4, 4));
        assert_eq!(actual, expected);

        let input = "#E######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#".to_string();

        let expected = sample_blizzards();
        let (actual, goal) = parse_input(&input);

        assert_eq!(goal, (5, 3));
        assert_eq!(actual, expected);
    }

    #[test]
    fn can_find_shortest_path() {
        assert_eq!(find_shortest_path(&sample_blizzards(), (0, 0), 0, (5, 3), (6, 4)), 18);
        assert_eq!(find_shortest_path(&sample_blizzards(), (5, 3), 18, (0, 0), (6, 4)), 18 + 23);
    }

    #[test]
    fn can_find_shortest_path_single() {
        assert_eq!(find_shortest_path_single(&sample_blizzards(), (5, 3)), 18);
    }

    #[test]
    fn can_find_shortest_path_returning() {
        assert_eq!(find_shortest_path_returning(&sample_blizzards(), (5, 3)), 54);
    }

    #[test]
    fn can_determine_clear_areas() {
        let blizzards = sample_blizzards();
        let bounds = (6, 4);

        for (pos, dist, expected) in vec![
            ((0, 0), 1, true),
            ((1, 0), 1, false),
            ((0, 1), 2, true),
            ((2, 0), 6, true),
            ((2, 1), 6, false),
        ] {
            assert_eq!(
                is_clear(&blizzards, pos, dist, &bounds),
                expected,
                "is_clear for pos: ({:?}) after {dist} steps", pos
            )
        }
    }
}

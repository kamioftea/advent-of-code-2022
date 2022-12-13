//! This is my solution for [Advent of Code - Day 12 - _Hill Climbing Algorithm_](https://adventofcode.com/2022/day/12)
//!
//!

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs;
use crate::util::grid::Grid;

type Position = (usize, usize);

/// This is juts copied from  the example [`BinaryHeap`] with position swapped for coords.
#[derive(Copy, Clone, Eq, PartialEq)]
struct Cell {
    dist: usize,
    cost: usize,
    coords: Position,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.coords.cmp(&other.coords))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-12-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 12.
pub fn run() {
    let contents =
        fs::read_to_string("res/day-12-input").expect("Failed to read file");
    let (grid, start, goal) = parse_input(&contents);

    println!(
        "The shortest path to the goal is: {}",
        find_shortest_path_from_start(&grid, start, goal).unwrap()
    );

    println!(
        "The shortest trail to the goal is: {}",
        find_shortest_trail(&grid, goal).unwrap()
    )
}

fn parse_input(input: &String) -> (Grid, Position, Position) {
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, line) in input.lines().enumerate() {
        for (x, chr) in line.chars().enumerate() {
            if chr == 'S' {
                start = (y, x)
            } else if chr == 'E' {
                end = (y, x)
            }
        }
    }

    let grid = Grid::from_string_with_mapping(
        input,
        |chr| match chr {
            'S' => 1,
            'E' => 26,
            c => u8::try_from(c).unwrap() & 0b11111
        },
    );

    (grid, start, end)
}

fn manhatten_distance((y_a, x_a): Position, (y_b, x_b): Position) -> usize {
    y_a.abs_diff(y_b) + x_a.abs_diff(x_b)
}

fn find_shortest_path<F, GP, HP>(
    grid: &Grid,
    start: Position,
    height_difference_predicate: HP,
    goal_met_predicate: GP,
    cost_function: F,
) -> Option<usize>
    where
        HP: Fn(u8, u8) -> bool,
        GP: Fn(Position, u8) -> bool,
        F: Fn(Position, usize, u8) -> usize,
{
    let mut heap: BinaryHeap<Cell> = BinaryHeap::new();
    let mut dists: Vec<usize> = (0..grid.numbers.len()).map(|_| usize::MAX).collect();

    dists[grid.pos_of(start).unwrap()] = 0;
    heap.push(Cell {
        dist: 0,
        cost: cost_function(start, 0, 0),
        coords: start,
    });

    while let Some(Cell { dist, coords, .. }) = heap.pop() {
        let current_height = grid.get(coords.0, coords.1).unwrap();

        if goal_met_predicate(coords, current_height) {
            return Some(dist);
        }

        if dist > dists[grid.pos_of(coords).unwrap()] {
            continue;
        }


        for (next_coords, next_height) in grid.get_orthogonal_surrounds(coords) {
            let next_pos = grid.pos_of(next_coords).unwrap();

            if height_difference_predicate(current_height, next_height) && (dist + 1 < dists[next_pos]) {
                heap.push(Cell {
                    dist: dist + 1,
                    cost: cost_function(next_coords, dist + 1, next_height),
                    coords: next_coords,
                });
                dists[next_pos] = dist + 1
            }
        }
    }

    None
}

fn find_shortest_trail(
    grid: &Grid,
    goal: Position,
) -> Option<usize> {
    find_shortest_path(
        grid,
        goal,
        |curr_h, next_h| curr_h - 1 <= next_h,
        |_, h| h == 1,
        |_, d, h| d + usize::from(h),
    )
}

fn find_shortest_path_from_start(
    grid: &Grid,
    start: Position,
    goal: Position,
) -> Option<usize> {
    find_shortest_path(
        grid,
        start,
        |curr_h, next_h| curr_h + 1 >= next_h,
        |pos, _| pos == goal,
        |pos, d, h| manhatten_distance(pos, goal) + d + usize::from(h),
    )
}

#[cfg(test)]
mod tests {
    use crate::day_12::{find_shortest_path_from_start, find_shortest_trail, parse_input, Position};
    use crate::util::grid::Grid;

    fn sample_data() -> (Grid, Position, Position) {
        (
            Grid {
                width: 8,
                numbers: vec![
                    1, 1, 2, 17, 16, 15, 14, 13,
                    1, 2, 3, 18, 25, 24, 24, 12,
                    1, 3, 3, 19, 26, 26, 24, 11,
                    1, 3, 3, 20, 21, 22, 23, 10,
                    1, 2, 4, 5, 6, 7, 8, 9,
                ],
            },
            (0, 0),
            (2, 5)
        )
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn can_parse() {
        let input =
            "Sabqponm\n\
             abcryxxl\n\
             accszExk\n\
             acctuvwj\n\
             abdefghi".to_string();

        assert_eq!(parse_input(&input), sample_data())
    }

    #[test]
    fn can_find_shortest_path() {
        let (grid, start, goal) = sample_data();
        assert_eq!(
            find_shortest_path_from_start(&grid, start, goal),
            Some(31)
        )
    }

    #[test]
    fn can_find_shortest_trail() {
        let (grid, _, goal) = sample_data();
        assert_eq!(
            find_shortest_trail(&grid, goal),
            Some(29)
        )
    }
}

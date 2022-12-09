//! This is my solution for [Advent of Code - Day 8 - _Treetop Tree House_](https://adventofcode.com/2022/day/8)
//!
//! Identify the best tree in a grid to build a tree house in, it must be hidden and have a good view.

use std::fs;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use crate::util::grid::Grid;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-8-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 8.
pub fn run() {
    let contents = fs::read_to_string("res/day-8-input").expect("Failed to read file");
    let grid = Grid::from(contents);

    println!(
        "The count of visible trees is: {}",
        find_visible_count(&grid)
    );

    println!(
        "The tree with the highest scenery score is: {}",
        find_best_scenery_score(&grid)
    );
}

/// Count the trees visible from the edges of the grid.
fn find_visible_count(grid: &Grid) -> usize {
    let mut visible = Grid::new(
        grid.width,
        grid.height(),
        |x, y| u8::from(x == 0 || y == 0 || x == grid.width - 1 || y == grid.height() - 1),
    );

    for y in 1..(grid.height() - 1) {
        mark_visible_trees((y, 0), (0, 1), &grid, &mut visible);
        mark_visible_trees((y, grid.width - 1), (0, -1), &grid, &mut visible);
    }

    for x in 1..(grid.width - 1) {
        mark_visible_trees((0, x), (1, 0), &grid, &mut visible);
        mark_visible_trees((grid.height() - 1, x), (-1, 0), &grid, &mut visible);
    }

    visible.sum()
}

/// For a given row or column start or end step forwards or backwards marking those that can be seen from the starting
/// position
fn mark_visible_trees(
    (origin_y, origin_x): (usize, usize),
    (dy, dx): (isize, isize),
    trees_grid: &Grid,
    visibility_grid: &mut Grid,
) {
    let mut max_height = trees_grid.get(origin_x, origin_y).unwrap();

    (1..)
        .map(
            |pos| {
                with_delta(origin_y, dy, pos)
                    .zip(with_delta(origin_x, dx, pos))
                    .and_then(|(y, x)| trees_grid.get(y, x).map(|h| (y, x, h)))
            })
        .while_some()
        .for_each(|(y, x, h)| {
            if max_height < h {
                max_height = h;
                visibility_grid.set(y, x, 1);
            }
        });
}

/// Apply a multiple of a delta to a starting position to get the position for a step if it is positive
fn with_delta(init: usize, delta: isize, multiplier: isize) -> Option<usize> {
    isize::try_from(init)
        .map(|init_i| init_i + delta * multiplier)
        .and_then(|result| usize::try_from(result))
        .ok()
}

/// For each tree in the grid, multiply the trees they can see in each direction. Return the best score.
fn find_best_scenery_score(grid: &Grid) -> usize {
    let deltas: Vec<(isize, isize)> = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];

    grid.iter().map(
        |(pos, height)| {
            deltas.iter()
                  .map(|&delta| count_visible_with_delta(pos, delta, height, grid))
                  .reduce(|a, b| a * b).unwrap_or(0)
        }
    ).max().unwrap_or(0)
}

/// For a given tree, count how many trees can be seen in a given direction
fn count_visible_with_delta((origin_y, origin_x): (usize, usize), (dy, dx): (isize, isize), origin_height: u8, grid: &Grid) -> usize {
    (1..)
        .map(
            |pos| {
                with_delta(origin_y, dy, pos)
                    .zip(with_delta(origin_x, dx, pos))
                    .and_then(|(y, x)| grid.get(y, x))
            })
        .while_some()
        .fold_while(
            0,
            |count, h|
                if h >= origin_height { Done(count + 1) } else { Continue(count + 1) },
        ).into_inner()
}

#[cfg(test)]
mod tests {
    use crate::day_8::{find_best_scenery_score, find_visible_count};
    use crate::util::grid::Grid;

    fn sample_grid() -> Grid {
        let input = "30373
25512
65332
33549
35390".to_string();

        Grid::from(input)
    }

    #[test]
    fn can_count_visible() {
        assert_eq!(find_visible_count(&sample_grid()), 21);
    }

    #[test]
    fn can_find_max_score() {
        assert_eq!(find_best_scenery_score(&sample_grid()), 8);
    }
}

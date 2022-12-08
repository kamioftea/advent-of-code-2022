//! This is my solution for [Advent of Code - Day 8 - _Title_](https://adventofcode.com/2022/day/8)
//!
//!

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

fn find_visible_count(grid: &Grid) -> usize {
    let mut visible = Grid::new(
        grid.width,
        grid.height(),
        |x, y| u8::from(x == 0 || y == 0 || x == grid.width - 1 || y == grid.height() - 1));

    //rowa
    for y in 1..(grid.height() - 1) {
        let mut left_height: u8 = grid.get(y, 0).unwrap();
        let mut right_height: u8 = grid.get(y, grid.width - 1).unwrap();

        for x in 1..(grid.width - 1) {
            if left_height < grid.get(y, x).unwrap() {
                left_height = grid.get(y, x).unwrap();
                visible.set(y, x, 1);
            }

            let right_x = grid.width - 1 - x;
            if right_height < grid.get(y, right_x).unwrap() {
                right_height = grid.get(y, right_x).unwrap();
                visible.set(y, right_x, 1);
            }
        }
    }

    //cols
    for x in 1..(grid.width - 1) {
        let mut top_height: u8 = grid.get(0, x).unwrap();
        let mut bottom_height: u8 = grid.get(grid.height() - 1, x).unwrap();

        for y in 1..(grid.height() - 1) {
            if top_height < grid.get(y, x).unwrap() {
                top_height = grid.get(y, x).unwrap();
                visible.set(y, x, 1);
            }

            let bottom_y = grid.height() - 1 - y;

            if bottom_height < grid.get(bottom_y, x).unwrap() {
                bottom_height = grid.get(bottom_y, x).unwrap();
                visible.set(bottom_y, x, 1);
            }
        }
    }

    visible.sum()
}

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

fn with_delta(init: usize, delta: isize, multiplier: isize) -> Option<usize> {
    isize::try_from(init)
        .map(|init_i| init_i + delta * multiplier)
        .and_then(|result| usize::try_from(result))
        .ok()
}

#[cfg(test)]
mod tests {
    use crate::day_8::{find_best_scenery_score, find_visible_count};
    use crate::util::grid::Grid;

    #[test]
    fn can_count_visible() {
        let input = "30373
25512
65332
33549
35390".to_string();
        let grid = Grid::from(input);

        assert_eq!(find_visible_count(&grid), 21);
    }

    #[test]
    fn can_find_max_score() {
        let input = "30373
25512
65332
33549
35390".to_string();
        let grid = Grid::from(input);

        assert_eq!(find_best_scenery_score(&grid), 8);
    }
}

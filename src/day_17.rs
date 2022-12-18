//! This is my solution for [Advent of Code - Day 17 - _Title_](https://adventofcode.com/2022/day/17)
//!
//!

use std::collections::HashMap;
use std::fmt::{Debug};
use std::fs;
use itertools::Itertools;
use crate::day_17::GasJet::{LEFT, RIGHT};

#[derive(Eq, PartialEq, Debug)]
enum GasJet {
    LEFT,
    RIGHT,
}

impl TryFrom<char> for GasJet {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(LEFT),
            '>' => Ok(RIGHT),
            _ => Err(())
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
struct Shape {
    width: u8,
    bitmap: Vec<u8>,
}

#[derive(Eq, PartialEq)]
struct Column {
    rows: Vec<u8>,
    current_shape: Option<Shape>,
    shape_x: u8,
    shape_y: usize,
    deepest_fall: usize,
    skipped: usize,
}

impl Column {
    fn new() -> Self {
        Self {
            rows: Vec::new(),
            current_shape: None,
            shape_x: 0,
            shape_y: 0,
            deepest_fall: 0,
            skipped: 0,
        }
    }

    fn intersects(&self, new_x: u8, new_y: usize) -> bool {
        if let Some(shape) = &self.current_shape {
            let shift = 7 - new_x - shape.width;

            for (dy, slice) in shape.bitmap.iter().rev().enumerate() {
                if let Some(row) = self.rows.get(new_y + dy) {
                    if row & (slice << shift) != 0 {
                        return true;
                    }
                };
            }
        }

        false
    }

    fn add_shape(&mut self, s: Shape) {
        self.current_shape = Some(s);
        self.shape_x = 2;
        self.shape_y = self.rows.len() + 3;
    }

    fn drop_shape(&mut self) -> Option<(Shape, Vec<u8>)> {
        if let Some(shape) = &self.current_shape {
            if self.shape_y == 0 || self.intersects(self.shape_x, self.shape_y - 1) {
                while self.rows.len() < self.shape_y + shape.bitmap.len() {
                    self.rows.push(0)
                }

                let shift = 7 - self.shape_x - shape.width;

                for (dy, &slice) in shape.bitmap.iter().rev().enumerate() {
                    let row = self.rows.get_mut(self.shape_y + dy).unwrap();
                    *row = *row | (slice << shift);
                }

                let rock_fell = self.rows.len() - self.shape_y;
                if rock_fell > self.deepest_fall {
                    self.deepest_fall = rock_fell
                }

                let snapshot = (shape.clone(), self.rows.iter().rev().take(self.deepest_fall).map(|&row| row).collect());

                self.current_shape = None;

                return Some(snapshot)
            } else {
                self.shape_y = self.shape_y - 1
            }
        }

        None
    }

    fn push_shape(&mut self, jet: &GasJet) {
        match (jet, &self.current_shape) {
            (LEFT, Some(_))
            if self.shape_x > 0 && !self.intersects(self.shape_x - 1, self.shape_y) =>
                self.shape_x = self.shape_x - 1,

            (RIGHT, Some(s))
            if s.width + self.shape_x < 7 && !self.intersects(self.shape_x + 1, self.shape_y) =>
                self.shape_x = self.shape_x + 1,

            _ => {}
        }
    }

    fn height(&self) -> usize {
        self.rows.len() + self.skipped
    }

    #[allow(dead_code)]
    fn print(&self) -> String {
        let rows = self.rows.iter().rev()
            .map(|&row| {
                format!(
                    "|{}|",
                    (0..7).rev()
                        .map(|offset| if row & (1 << offset) == 0 { '.' } else { '#' })
                        .join(""))
            })
            .join("\n");

        format!("{rows}\n+-------+")
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-17-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 17.
pub fn run() {
    let contents = fs::read_to_string("res/day-17-input").expect("Failed to read file");
    let jets = parse_input(&contents);

    println!(
        "After 2022 rocks the pile is {} units high",
        simulate_rocks(&jets, 2022).height()
    );

    println!(
        "After 1,000,000,000,000 rocks the pile is {} units high",
        simulate_rocks(&jets, 1_000_000_000_000).height()
    )
}

fn parse_input(input: &String) -> Vec<GasJet> {
    input.chars().flat_map(GasJet::try_from).collect()
}

fn simulate_rocks(jets: &Vec<GasJet>, rock_count: usize) -> Column {
    let dash: Shape = Shape { width: 4, bitmap: vec![0b1111] };
    let plus: Shape = Shape { width: 3, bitmap: vec![0b010, 0b111, 0b010] };
    let angle: Shape = Shape { width: 3, bitmap: vec![0b001, 0b001, 0b111] };
    let pipe: Shape = Shape { width: 1, bitmap: vec![0b1, 0b1, 0b1, 0b1] };
    let square: Shape = Shape { width: 2, bitmap: vec![0b11, 0b11] };

    let mut shapes = vec![dash, plus, angle, pipe, square].into_iter().cycle();
    let mut column = Column::new();
    let mut states: HashMap<(Shape, Vec<u8>, usize), (usize, usize)> = HashMap::new();
    let mut rocks_dropped = 0;


    for (jet_id, jet) in jets.into_iter().enumerate().cycle() {
        if column.current_shape.is_some() {
            if let Some((shape, top_section)) = column.drop_shape() {
                let snapshot = (shape.clone(), top_section.clone(), jet_id);
                if let Some((prev_height, prev_dropped)) = states.get(&snapshot) {
                    if column.skipped == 0 {
                        let cycle_height = column.height() - prev_height;
                        let cycle_length = rocks_dropped - prev_dropped;

                        let cycles_to_skip = (rock_count - rocks_dropped) / cycle_length;

                        column.skipped = cycle_height * cycles_to_skip;
                        rocks_dropped += cycle_length * cycles_to_skip;
                    }
                }

                if column.skipped == 0 {
                    states.insert((shape.clone(), top_section.clone(), jet_id), (column.rows.len(), rocks_dropped));
                }
            }
        }

        if column.current_shape.is_none() {
            if rocks_dropped == rock_count {
                return column;
            }

            rocks_dropped += 1;
            column.add_shape(shapes.next().unwrap().clone());
        }

        column.push_shape(jet)
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use crate::day_17::{GasJet, parse_input, simulate_rocks};
    use crate::day_17::GasJet::{LEFT, RIGHT};

    fn sample_jets() -> Vec<GasJet> {
        vec![
            RIGHT, RIGHT, RIGHT, LEFT, LEFT, RIGHT, LEFT, RIGHT, RIGHT, LEFT, LEFT, LEFT, RIGHT, RIGHT, LEFT, RIGHT,
            RIGHT, RIGHT, LEFT, LEFT, LEFT, RIGHT, RIGHT, RIGHT, LEFT, LEFT, LEFT, RIGHT, LEFT, LEFT, LEFT, RIGHT,
            RIGHT, LEFT, RIGHT, RIGHT, LEFT, LEFT, RIGHT, RIGHT,
        ]
    }

    #[test]
    fn can_parse() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".to_string();

        assert_eq!(parse_input(&input), sample_jets());
    }

    #[test]
    fn can_simulate_rocks() {
        let expected_6 = "|.####..|
|....##.|
|....##.|
|....#..|
|..#.#..|
|..#.#..|
|#####..|
|..###..|
|...#...|
|..####.|
+-------+".to_string();

        let expected_10 = "|....#..|
|....#..|
|....##.|
|##..##.|
|######.|
|.###...|
|..#....|
|.####..|
|....##.|
|....##.|
|....#..|
|..#.#..|
|..#.#..|
|#####..|
|..###..|
|...#...|
|..####.|
+-------+".to_string();

        assert_eq!(simulate_rocks(&sample_jets(), 6).print(), expected_6);
        assert_eq!(simulate_rocks(&sample_jets(), 10).print(), expected_10);
        assert_eq!(simulate_rocks(&sample_jets(), 2022).height(), 3068);
        assert_eq!(simulate_rocks(&sample_jets(), 1_000_000_000_000).height(), 1_514_285_714_288)
    }
}

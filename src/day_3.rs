//! This is my solution for [Advent of Code - Day 3 - _Calorie Counting_](https://adventofcode.com/2022/day/1)
//!
//! The task is to sum all the calories carried per elf on our expedition and find those that are carrying the most.

use std::fs;

type Rucksack = (Set<char>, Set<char>);

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-3-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 1.
pub fn run() {
    let contents = fs::read_to_string("res/day-3-input").expect("Failed to read file");
}

fn parse_input(input: &String) -> Vec<Rucksack>{
    input.lines()
      .map(parse_rucksack)
      .collect()
}

fn parse_rucksack(line: &str) -> Rucksack {
    (a, b) = line.split(line.length());
    (parse_compartment(a), )
}

fn parse_compartment(line_half: &str) -> Set<char> {
    line_half.chars().collect()
}

#[cfg(test)]
mod tests {
  use crate::day_3::{parse_rucksack}

  #[test]
  fn can_parse_rucksack() {
    assert_eq!(
        parse_rucksack("ttgJtRGJQctTZtZT"),
        (set!('t', 'g', 'J', 'R'))
     );
  }
}

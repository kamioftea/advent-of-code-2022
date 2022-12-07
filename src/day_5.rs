//! This is my solution for [Advent of Code - Day 5 - _Supply Stacks_](https://adventofcode.com/2022/day/5)
//!
//!

use std::collections::VecDeque;
use std::fs;
use itertools::Itertools;

/// Specifies one move of a stack of crates: `(number_of_crates, source_stack, target_stack)`
type Move = (usize, usize, usize);

/// Represents the current state of the set of stacks being moved.
#[derive(Eq, PartialEq, Debug, Clone)]
struct SupplyStacks {
    stacks: Vec<VecDeque<char>>
}

impl From<&str> for SupplyStacks {
    /// Parses a diagram of crates into the internal representation.
    ///
    /// Example input:
    /// ```text
    ///     [D]
    /// [N] [C]
    /// [Z] [M] [P]
    ///  1   2   3
    /// ```
    fn from(input: &str) -> Self {
        let mut stacks: Vec<VecDeque<char>> = Vec::new();
        let mut lines = input.lines().rev();
        let numbers = lines.next().unwrap();

        for _ in numbers.split_whitespace() {
            stacks.push(VecDeque::new())
        };

        for line in lines {
            for (i, chunk) in line.chars().chunks(4).into_iter().enumerate() {
                // Chunk is either `[#] ` ore `    `, the last in each line will be missing the final space
                let character: char = chunk.dropping(1).next().unwrap();
                if character.is_alphabetic() {
                    stacks[i].push_front(character)
                }
            }
        }

        SupplyStacks { stacks }
    }
}

impl SupplyStacks {
    /// Apply a single move of crates - either one by one, or all at once
    fn do_move(&mut self, (count, from, to): Move, all_at_once: bool) {
        let mut temp = Vec::new();
        for _ in 0..count {
            temp.push(self.stacks[from - 1].pop_front().unwrap());
        }

        if all_at_once {
            temp.reverse()
        }

        for c in temp {
            self.stacks[to - 1].push_front(c)
        }
    }

    /// Apply a list of moves delegating each move to [`SupplyStacks::do_move`]
    fn do_moves(&mut self, mvs: &Vec<Move>, all_at_once: bool) {
        for &mv in mvs {
            self.do_move(mv, all_at_once)
        }
    }

    /// Combine the characters at the top of each stack into a string used as the puzzle output.
    fn get_top_crates(&self) -> String {
        self.stacks.to_owned().into_iter().map(|stack| stack[0]).join("")
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-5-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 5.
pub fn run() {
    let contents = fs::read_to_string("res/day-5-input").expect("Failed to read file");
    let (mut part_1_stacks, moves) = parse_input(&contents);
    let mut part_2_stacks = part_1_stacks.clone();

    part_1_stacks.do_moves(&moves, false);
    println!(
        "After moving one crate at a time, the top of the stacks are: {}",
        part_1_stacks.get_top_crates()
    );

    part_2_stacks.do_moves(&moves, true);
    println!(
        "After moving the crates in bulk, the top of the stacks are: {}",
        part_2_stacks.get_top_crates()
    );
}

/// SPlit the input into the two sections and independently parse each one
fn parse_input(input: &String) -> (SupplyStacks, Vec<Move>) {
    let (stack_spec, moves_spec) = input.split_once("\n\n").unwrap();

    (SupplyStacks::from(stack_spec), parse_moves(moves_spec))
}

/// Map the list of moves to the internal representation
fn parse_moves(input: &str) -> Vec<Move> {
    input.lines().map(parse_move).collect()
}

/// Parse a single move line in the format `move 2 from 2 to 1`
fn parse_move(line: &str) -> Move {
    let parts: Vec<usize> =
        line.split_whitespace()
            .flat_map(|str| str.parse::<usize>())
            .collect();

    (parts[0], parts[1], parts[2])
}

#[cfg(test)]
mod tests {
    use crate::day_5::{SupplyStacks, Move, parse_input};

    #[test]
    fn can_parse() {
        let sample_input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2".to_string();

        let (actual_stacks, actual_moves) = parse_input(&sample_input);

        assert_eq!(actual_stacks, sample_stacks());
        assert_eq!(actual_moves, sample_moves());
    }

    fn sample_moves() -> Vec<Move> {
        vec![
            (1, 2, 1),
            (3, 1, 3),
            (2, 2, 1),
            (1, 1, 2)
        ]
    }

    fn sample_stacks() -> SupplyStacks {
        SupplyStacks {
            stacks: vec![
                vec!['N', 'Z'].into_iter().collect(),
                vec!['D', 'C', 'M'].into_iter().collect(),
                vec!['P'].into_iter().collect(),
            ]
        }
    }

    #[test]
    fn can_apply_moves() {
        let mut stacks_singly = sample_stacks();
        let mut stacks_bulk = stacks_singly.clone();

        stacks_singly.do_moves(&sample_moves(), false);
        assert_eq!(
            stacks_singly,
            sample_stacks_after_moving_one_at_a_time()
        );

        stacks_bulk.do_moves(&sample_moves(), true);
        assert_eq!(
            stacks_bulk,
            sample_stacks_after_moving_in_bulk()
        );
    }

    fn sample_stacks_after_moving_one_at_a_time() -> SupplyStacks {
        SupplyStacks {
            stacks: vec![
                vec!['C'].into_iter().collect(),
                vec!['M'].into_iter().collect(),
                vec!['Z', 'N', 'D', 'P'].into_iter().collect(),
            ]
        }
    }

    fn sample_stacks_after_moving_in_bulk() -> SupplyStacks {
        SupplyStacks {
            stacks: vec![
                vec!['M'].into_iter().collect(),
                vec!['C'].into_iter().collect(),
                vec!['D', 'N', 'Z', 'P'].into_iter().collect(),
            ]
        }
    }

    #[test]
    fn can_get_stack_tops() {
        assert_eq!(sample_stacks().get_top_crates(), "NDP");
        assert_eq!(sample_stacks_after_moving_one_at_a_time().get_top_crates(), "CMZ");
    }
}

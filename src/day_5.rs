//! This is my solution for [Advent of Code - Day 5 - _Supply Stacks_](https://adventofcode.com/2022/day/5)
//!
//!

use std::collections::VecDeque;
use std::fs;
use itertools::Itertools;

#[derive(Eq, PartialEq, Debug, Clone)]
struct CargoStacks {
    stacks: Vec<VecDeque<char>>
}

type Move = (usize, usize, usize);

impl From<&str> for CargoStacks {
    fn from(input: &str) -> Self {
        let mut stacks: Vec<VecDeque<char>> = Vec::new();
        let mut lines = input.lines().rev();
        let numbers = lines.next().unwrap();

        for _ in numbers.split_whitespace() {
            stacks.push(VecDeque::new())
        };

        for line in lines {
            for (i, chunk) in line.chars().chunks(4).into_iter().enumerate() {
                let character: char = chunk.dropping(1).next().unwrap();
                if character.is_alphabetic() {
                    stacks[i].push_front(character)
                }
            }
        }

        CargoStacks{ stacks }
    }
}

impl CargoStacks {
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

    fn do_moves(&mut self, mvs: &Vec<Move>, all_at_once: bool) {
        for &mv in mvs {
            self.do_move(mv, all_at_once)
        }
    }

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

fn parse_input(input: &String) -> (CargoStacks, Vec<Move>) {
    let (stack_spec, moves_spec) = input.split_once("\n\n").unwrap();

    (CargoStacks::from(stack_spec), parse_moves(moves_spec))
}

fn parse_moves(input: &str) -> Vec<Move> {
    input.lines().map(parse_move).collect()
}

fn parse_move(line: &str) -> Move {
    let parts: Vec<usize> = line.split_whitespace().flat_map(|str| str.parse::<usize>()).collect();

    (parts[0], parts[1], parts[2])
}

#[cfg(test)]
mod tests {
    use crate::day_5::{CargoStacks, Move, parse_input};

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

    fn sample_stacks() -> CargoStacks {
        CargoStacks {
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

    fn sample_stacks_after_moving_one_at_a_time() -> CargoStacks {
        CargoStacks {
            stacks: vec![
                vec!['C'].into_iter().collect(),
                vec!['M'].into_iter().collect(),
                vec!['Z', 'N', 'D', 'P'].into_iter().collect(),
            ]
        }
    }

    fn sample_stacks_after_moving_in_bulk() -> CargoStacks {
        CargoStacks {
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

//! This is my solution for [Advent of Code - Day 2 - _Title_](https://adventofcode.com/2021/day/2)
//!
//!

use std::fs;
use crate::day_2::Move::{Paper, Rock, Scissors};

#[derive(Eq, PartialEq, Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

type Round = (Move, Move);

/// The entry point for running the solutions with the 'real' puzzle input.
//
/// - The puzzle input is expected to be at `<project_root>/res/day-2-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 2.
pub fn run() {
    let _contents = fs::read_to_string("res/day-02-input").expect("Failed to read file");
}

fn parse_guide(guide: &String) -> Vec<Round> {
    guide.lines()
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> Round {
    let chars: Vec<char> = line.chars().collect();
    (parse_move(chars[0]).unwrap(), parse_move(chars[2]).unwrap())
}

fn parse_move(chr: char) -> Option<Move> {
    match chr {
         'A' | 'X' => Some(Rock),
         'B' | 'Y' => Some(Paper),
         'C' | 'Z' => Some(Scissors),
         _ => None
    }
}



#[cfg(test)]
mod tests {
    use crate::day_2::{parse_guide, Round};
    use crate::day_2::Move::{Paper, Rock, Scissors};

    #[test]
    fn can_parse() {
        let example_guide = "A Y
B X
C Z".to_string();

        assert_eq!(
            parse_guide(&example_guide),
            vec![
                (Rock, Paper),
                (Paper, Rock),
                (Scissors, Scissors),
            ]
        )
    }

    // fn can_score() {
    //
    // }
}

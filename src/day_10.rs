//! This is my solution for [Advent of Code - Day 10 - _Cathode-Ray Tube_](https://adventofcode.com/2022/day/10)
//!
//! Interpret a set of instructions into pixels on a display

use std::fs;
use itertools::Itertools;
use crate::day_10::Instruction::{ADDX, NOOP};

/// Represent the two possible instruction types that the puzzle input can contain
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Instruction {
    ADDX(isize),
    NOOP,
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-10-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 10.
pub fn run() {
    let contents =
        fs::read_to_string("res/day-10-input").expect("Failed to read file");

    let instructions = parse_input(&contents);

    println!(
        "The sum of sampled signal strengths is: {}",
        sample_and_sum_signal_strength(&instructions)
    );

    println!(
        "The screen shows: \n{}",
        draw_pixels(&instructions)
    );
}

/// Parse the puzzle input string
fn parse_input(input: &String) -> Vec<Instruction> {
    input.lines().map(parse_instruction).collect()
}

/// Parse a line of the input to a signal
fn parse_instruction(line: &str) -> Instruction {
    if line.starts_with("addx") {
        let (_, value) = line.split_once(" ").unwrap();
        ADDX(value.parse::<isize>().unwrap())
    } else {
        NOOP
    }
}

/// Interpret the instruction list into the signals sent to the display
fn to_signals(instructions: &Vec<Instruction>) -> Vec<isize> {
    let mut register = 1;
    let mut signals = Vec::new();
    for &instruction in instructions {
        match instruction {
            ADDX(x) => {
                signals.push(register);
                signals.push(register);
                register = register + x;
            }
            NOOP => signals.push(register)
        }
    }

    signals
}

/// Take specific signals and sum them
fn sample_and_sum_signal_strength(instructions: &Vec<Instruction>) -> isize {
    to_signals(instructions)
        .iter()
        .enumerate()
        .dropping(19)
        .step_by(40)
        .take(6)
        .map(|(step, &signal)| isize::try_from(step + 1).unwrap() * signal)
        .sum()
}

/// Interpret the signals as controlling a "sprite" that will cause a pixel to be lit if the sprite overlaps whilst the
/// pixel is drawn.
fn draw_pixels(instructions: &Vec<Instruction>) -> String {
    let mut lines = String::new();

    for (i, &signal) in to_signals(instructions).iter().enumerate() {
        let pos = isize::try_from(i % 40).unwrap();

        lines.push(
            if pos.abs_diff(signal) <= 1 { '█' } else { '.' }
        );

        if pos == 39 {
            lines.push('\n')
        }
    }

    lines
}

#[cfg(test)]
mod tests {
    use crate::day_10::Instruction::{ADDX, NOOP};
    use crate::day_10::{draw_pixels, Instruction, parse_input, sample_and_sum_signal_strength,
                        to_signals};

    #[test]
    fn can_parse() {
        let input = "noop
addx 3
addx -5".to_string();

        assert_eq!(parse_input(&input), vec![NOOP, ADDX(3), ADDX(-5)])
    }

    #[test]
    fn can_generate_signals() {
        assert_eq!(
            to_signals(&vec![NOOP, ADDX(3), ADDX(-5), NOOP]),
            vec!(1, 1, 1, 4, 4, -1)
        )
    }

    #[test]
    fn can_sum_signal_samples() {
        assert_eq!(
            sample_and_sum_signal_strength(&sample_instructions()),
            13140
        )
    }

    #[test]
    fn can_draw_pixels() {
        let expected = "██..██..██..██..██..██..██..██..██..██..
███...███...███...███...███...███...███.
████....████....████....████....████....
█████.....█████.....█████.....█████.....
██████......██████......██████......████
███████.......███████.......███████.....\n".to_string();

        assert_eq!(draw_pixels(&sample_instructions()), expected);
    }

    fn sample_instructions() -> Vec<Instruction> {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop".to_string();

        parse_input(&input)
    }
}

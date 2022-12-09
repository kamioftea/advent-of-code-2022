//! This is my solution for [Advent of Code - Day 9 - _Rope Bridge_](https://adventofcode.com/2022/day/9)
//!
//!

use std::fs;
use itertools::Itertools;
use crate::day_9::Direction::*;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

type Motion = (Direction, usize);
type Position = (isize, isize);

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-9-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 9.
pub fn run() {
    let contents =
        fs::read_to_string("res/day-9-input").expect("Failed to read file");

    let motions = parse_input(&contents);

    println!(
        "The tail of the rope with one knot passes through {} unique positions",
        count_tail_positions(&motions, 1)
    );

    println!(
        "The tail of the rope with 9 knots passes through {} unique positions",
        count_tail_positions(&motions, 9)
    );
}

/// Map the input file to the internal representation
fn parse_input(input: &String) -> Vec<Motion> {
    input.lines().map(parse_motion).collect()
}

/// Map a line of the input to the internal representation
fn parse_motion(line: &str) -> Motion {
    let (letter, number) = line.split_once(" ").unwrap();

    let direction = match letter {
        "U" => UP,
        "D" => DOWN,
        "L" => LEFT,
        "R" => RIGHT,
        _ => unreachable!()
    };

    let distance = number.parse().unwrap();

    (direction, distance)
}

/// Map a specification of a move of the head of the rope to the list of positions it follows
fn apply_motion((x, y): Position, (direction, distance): Motion) -> Vec<Position> {
    let mut positions = Vec::new();
    for d in 1..=distance {
        let d_i = isize::try_from(d).unwrap();
        positions.push(
            match direction {
                UP => (x, y - d_i),
                DOWN => (x, y + d_i),
                LEFT => (x - d_i, y),
                RIGHT => (x + d_i, y),
            }
        )
    }

    positions
}

/// Give a new head position, move the tail so it is still touching
fn update_tail((head_x, head_y): Position, (tail_x, tail_y): Position) -> Position {
    if (head_x - tail_x).abs() <= 1 && (head_y - tail_y).abs() <= 1 {
        (tail_x, tail_y)
    } else {
        (
            if tail_x < head_x { tail_x + 1 } else if tail_x > head_x { tail_x - 1 } else { tail_x },
            if tail_y < head_y { tail_y + 1 } else if tail_y > head_y { tail_y - 1 } else { tail_y },
        )
    }
}

/// map a list of motions specifications to the list of positions it follows
fn apply_motions(origin: Position, motions: &Vec<Motion>) -> Vec<Position> {
    let mut positions = Vec::new();
    positions.push(origin);

    for &motion in motions {
        apply_motion(*positions.last().unwrap(), motion)
            .iter()
            .for_each(|&pos| positions.push(pos));
    }

    positions
}

// Map the positions the previous section of a rope follows to the positions the next section follows
fn follow_head(origin: Position, head_positions: Vec<Position>) -> Vec<Position> {
    let mut tail_positions = Vec::new();
    tail_positions.push(origin);

    for head_position in head_positions {
        tail_positions.push(
            update_tail(
                head_position,
                *tail_positions.last().unwrap(),
            )
        );
    }

    tail_positions
}

/// Map the movement specification to the tail movement of an arbitrary length of rope.
fn count_tail_positions(head_motions: &Vec<Motion>, rope_length: usize) -> usize {
    (0..rope_length)
        .fold(
            apply_motions((0, 0), head_motions),
            |previous_knot, _| follow_head((0, 0), previous_knot),
        )
        .iter().unique().count()
}

#[cfg(test)]
mod tests {
    use crate::day_9::Direction::*;
    use crate::day_9::{apply_motion, apply_motions, count_tail_positions, follow_head, Motion, parse_input, update_tail};

    fn sample_motions() -> Vec<Motion> {
        vec![
            (RIGHT, 4),
            (UP, 4),
            (LEFT, 3),
            (DOWN, 1),
            (RIGHT, 4),
            (DOWN, 1),
            (LEFT, 5),
            (RIGHT, 2),
        ]
    }

    #[test]
    fn can_parse() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2".to_string();

        assert_eq!(parse_input(&input), sample_motions());
    }

    #[test]
    fn can_move_head() {
        assert_eq!(
            apply_motion((0, 0), (RIGHT, 4)),
            vec![(1, 0), (2, 0), (3, 0), (4, 0)]
        );

        assert_eq!(
            apply_motion((4, 0), (UP, 2)),
            vec![(4, -1), (4, -2)]
        )
    }

    #[test]
    fn can_apply_motions() {
        assert_eq!(
            apply_motions((0, 0), &vec![(RIGHT, 4), (UP, 2)]),
            vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (4, -1), (4, -2)]
        );
    }

    #[test]
    fn can_update_tail_for_head() {
        assert_eq!(update_tail((0, 0), (0, 0)), (0, 0));
        assert_eq!(update_tail((0, 1), (0, 0)), (0, 0));
        assert_eq!(update_tail((1, 1), (0, 0)), (0, 0));
        assert_eq!(update_tail((2, 2), (0, 0)), (1, 1));
        assert_eq!(update_tail((-2, 0), (0, 0)), (-1, 0));
        assert_eq!(update_tail((-2, 1), (0, 0)), (-1, 1));
    }

    #[test]
    fn can_follow_head() {
        assert_eq!(
            follow_head(
                (0, 0),
                vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (4, -1), (4, -2)],
            ),
            vec![(0, 0), (0, 0), (0, 0), (1, 0), (2, 0), (3, 0), (3, 0), (4, -1)]
        )
    }

    #[test]
    fn can_count_tail_locations() {
        assert_eq!(count_tail_positions(&sample_motions(), 1), 13);
        assert_eq!(count_tail_positions(&sample_motions(), 9), 1);

        let larger_input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20".to_string();
        let larger_example = parse_input(&larger_input);
        assert_eq!(count_tail_positions(&larger_example, 9), 36);

    }
}

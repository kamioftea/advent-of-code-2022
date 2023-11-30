//! This is my solution for [Advent of Code - Day 22 - _Title_](https://adventofcode.com/2022/day/22)
//!
//!

use std::collections::HashMap;
use std::fs;
use itertools::Itertools;
use crate::day_22::Facing::{DOWN, LEFT, RIGHT, UP};
use crate::day_22::Instruction::{Left, Move, Right};
use crate::util::grid::Grid;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Instruction {
    Move(usize),
    Left,
    Right,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum Facing {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Facing {
    fn turn(&self, instruction: &Instruction) -> Self {
        match (*instruction, *self) {
            (Left, UP) | (Right, DOWN) => LEFT,
            (Left, RIGHT) | (Right, LEFT) => UP,
            (Left, DOWN) | (Right, UP) => RIGHT,
            (Left, LEFT) | (Right, RIGHT) => DOWN,
            _ => unreachable!()
        }
    }

    fn as_usize(&self) -> usize {
        match *self {
            UP => 3,
            RIGHT => 0,
            DOWN => 1,
            LEFT => 2,
        }
    }

    fn as_delta(&self) -> (isize, isize) {
        match *self {
            UP => (-1, 0),
            RIGHT => (0, 1),
            DOWN => (1, 0),
            LEFT => (0, -1),
        }
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-22-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 22.
pub fn run() {
    let contents = fs::read_to_string("res/day-22-input").expect("Failed to read file");
    let (map, instructions) = parse_input(&contents);

    println!(
        "The final position reference when flat is: {}",
        walk_map(&map, &instructions)
    );

    println!(
        "The final position reference when a cube is: {}",
        walk_cube(&map, &instructions, 50, &actual_faces_map())
    );
}

fn parse_input(input: &String) -> (Grid, Vec<Instruction>) {
    let (map_input, instruction_input) = input.split_once("\n\n").unwrap();

    (parse_map(map_input), parse_instructions(instruction_input))
}

fn parse_map(input: &str) -> Grid {
    let max_length = input.lines().map(|l| l.len()).max().unwrap();
    let padded_input =
        input.lines()
             .map(|l| format!(
                 "{}{}",
                 l,
                 " ".repeat(max_length - l.len())
             ))
             .join("\n");

    Grid::from_string_with_mapping(
        &padded_input,
        |c| match c {
            ' ' => 0,
            '.' => 1,
            '#' => 2,
            _ => unreachable!("unexpected grid char: '{}'", c)
        },
    )
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut current_number = 0;
    let mut instructions = Vec::new();

    for c in input.trim().chars() {
        match c {
            d if d.is_digit(10) =>
                current_number =
                    current_number * 10
                        + c.to_digit(10)
                           .and_then(|d| usize::try_from(d).ok())
                           .unwrap(),
            'L' | 'R' => {
                instructions.push(Move(current_number));
                current_number = 0;
                if c == 'L' { instructions.push(Left) } else { instructions.push(Right) };
            }
            _ => unreachable!("unexpected instruction {}", c)
        }
    }

    if current_number > 0 {
        instructions.push(Move(current_number))
    }

    instructions
}

fn walk_map(map: &Grid, route: &Vec<Instruction>) -> usize {
    let mut position_x = 0;
    let mut position_y = 0;
    let mut facing = RIGHT;

    while map.get(position_y, position_x).unwrap() != 1 {
        position_x += 1
    }

    for instruction in route {
        match *instruction {
            Move(distance) => {
                let mut moved = 0;
                let mut steps = 0;

                let delta = facing.as_delta();
                let mut next_x = position_x;
                let mut next_y = position_y;

                while moved < distance {
                    steps += 1;

                    let (y, x) = apply_delta(
                        (position_y, position_x),
                        delta,
                        steps,
                        (map.height(), map.width),
                    );

                    match map.get(y, x) {
                        Some(0) => {}
                        Some(1) => {
                            moved += 1;
                            (next_x, next_y) = (x, y);
                        }
                        Some(2) => break,
                        d => unreachable!(
                            "unexpected grid cell {:?} at ({y}, {x}) vs ({}, {})",
                            d,
                            map.height(),
                            map.width
                        )
                    }
                }

                (position_x, position_y) = (next_x, next_y)
            }
            Left | Right => facing = facing.turn(instruction)
        }
    }


    1000 * (position_y + 1) + 4 * (position_x + 1) + facing.as_usize()
}

fn apply_delta(
    (start_y, start_x): (usize, usize),
    (delta_y, delta_x): (isize, isize),
    distance: usize,
    (wrap_y, wrap_x): (usize, usize),
) -> (usize, usize) {
    (
        wrapping_add(start_y, delta_y, distance, wrap_y),
        wrapping_add(start_x, delta_x, distance, wrap_x)
    )
}

fn wrapping_add(start: usize, delta: isize, multiple: usize, wrap_at: usize) -> usize {
    if delta == 0 {
        return start;
    }

    ((start as isize + delta * multiple as isize + wrap_at as isize) % wrap_at as isize) as usize
}

fn actual_faces_map() -> HashMap<((usize, usize), Facing), ((usize, usize), Facing)> {
    vec![
        (((0, 1), LEFT), ((2, 0), RIGHT)),
        (((0, 1), UP), ((3, 0), RIGHT)),
        //
        (((0, 2), UP), ((3, 0), UP)),
        (((0, 2), RIGHT), ((2, 1), LEFT)),
        (((0, 2), DOWN), ((1, 1), LEFT)),
            //
        (((1, 1), LEFT), ((2, 0), DOWN)),
        (((1, 1), RIGHT), ((0, 2), UP)),
            //
        (((2, 0), LEFT), ((0, 1), RIGHT)),
        (((2, 0), UP), ((1, 1), RIGHT)),

        (((2, 1), RIGHT), ((0, 2), LEFT)),
        (((2, 1), DOWN), ((3, 0), LEFT)),
            //
        (((3, 0), LEFT), ((0, 1), DOWN)),
        (((3, 0), RIGHT), ((2, 1), UP)),
        (((3, 0), DOWN), ((0, 2), DOWN)),
    ].into_iter().collect()
}

fn apply_cube_delta(
    (start_y, start_x): (usize, usize),
    start_facing: Facing,
    (wrap_y, wrap_x): (usize, usize),
    face_size: usize,
    face_map: &HashMap<((usize, usize), Facing), ((usize, usize), Facing)>,
) -> (usize, usize, Facing) {
    let (end_y, end_x) = apply_delta(
        (start_y, start_x),
        start_facing.as_delta(),
        1,
        (wrap_y, wrap_x),
    );

    let start_face_x = start_x / face_size;
    let start_face_y = start_y / face_size;
    let end_face_x = end_x / face_size;
    let end_face_y = end_y / face_size;

    let maybe_next_face = face_map.get(&((start_face_y, start_face_x), start_facing));
    if (start_face_x == end_face_x && start_face_y == end_face_y) ||
        maybe_next_face.is_none()
    {
        return (end_y, end_x, start_facing);
    }

    let &((next_face_y, next_face_x), next_facing) = maybe_next_face.unwrap();

    let pos_y = end_y % face_size;
    let pos_x = end_x % face_size;

    let rotation_mode = wrapping_add(
        next_facing.as_usize(),
        -(start_facing.as_usize() as isize),
        1,
        4
    );

    let (pos_y1, pos_x1) = match rotation_mode {
        0 => (pos_y, pos_x),
        1 => (pos_x, face_size - pos_y - 1),
        2 => (face_size - pos_y - 1, face_size - pos_x - 1),
        3 => (face_size - pos_x - 1, pos_y),
        _ => unreachable!("only 4 rotations"),
    };

    (pos_y1 + next_face_y * face_size, pos_x1 + next_face_x * face_size, next_facing)
}

fn walk_cube(
    map: &Grid,
    route: &Vec<Instruction>,
    face_size: usize,
    face_map: &HashMap<((usize, usize), Facing), ((usize, usize), Facing)>,
) -> usize {
    let mut position_x = 0;
    let mut position_y = 0;
    let mut facing = RIGHT;

    while map.get(position_y, position_x).unwrap() != 1 {
        position_x += 1
    }

    for instruction in route {
        match *instruction {
            Move(distance) => {
                let mut moved = 0;

                while moved < distance {
                    let (next_y, next_x, next_facing) = apply_cube_delta(
                        (position_y, position_x),
                        facing,
                        (map.height(), map.width),
                        face_size,
                        face_map
                    );

                    if map.get(next_y, next_x).unwrap() == 2 {
                        break;
                    }

                    (position_y, position_x) = (next_y, next_x);
                    facing = next_facing;

                    moved += 1
                }
            }
            Left | Right => facing = facing.turn(instruction)
        }
    }

    1000 * (position_y + 1) + 4 * (position_x + 1) + facing.as_usize()
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::day_22::Instruction::{Left, Move, Right};
    use crate::day_22::{actual_faces_map, apply_cube_delta, Facing, parse_input, walk_cube, walk_map};
    use crate::day_22::Facing::{DOWN, LEFT, RIGHT, UP};

    fn sample_input() -> String {
        vec![
            "        ...#",
            "        .#..",
            "        #...",
            "        ....",
            "...#.......#",
            "........#...",
            "..#....#....",
            "..........#.",
            "        ...#....",
            "        .....#..",
            "        .#......",
            "        ......#.",
            "",
            "10R5L5R10L4R5L5",
        ].join("\n")
    }

    fn sample_faces_map() -> HashMap<((usize, usize), Facing), ((usize, usize), Facing)> {
        vec![
            (((0, 2), LEFT), ((1, 1), DOWN)),
            (((0, 2), UP), ((1, 0), DOWN)),
            (((0, 2), RIGHT), ((2, 3), LEFT)),
            //
            (((1, 0), LEFT), ((2, 3), UP)),
            (((1, 0), UP), ((0, 2), DOWN)),
            (((1, 0), DOWN), ((2, 2), UP)),
            //
            (((1, 1), UP), ((0, 2), RIGHT)),
            (((1, 1), DOWN), ((2, 2), RIGHT)),
            //
            (((1, 2), RIGHT), ((2, 3), DOWN)),
            //
            (((2, 2), LEFT), ((1, 1), UP)),
            (((2, 2), DOWN), ((1, 0), UP)),
            //
            (((2, 3), UP), ((2, 2), LEFT)),
            (((2, 3), RIGHT), ((0, 2), LEFT)),
            (((2, 3), DOWN), ((1, 0), RIGHT)),
        ].into_iter().collect()
    }

    #[test]
    fn can_parse() {
        let (grid, instructions) = parse_input(&sample_input());

        assert_eq!(
            grid.print_with(|c| match c {
                0 => " ".to_string(),
                1 => ".".to_string(),
                2 => "#".to_string(),
                _ => unreachable!()
            }),
            vec![
                "        ...#    ",
                "        .#..    ",
                "        #...    ",
                "        ....    ",
                "...#.......#    ",
                "........#...    ",
                "..#....#....    ",
                "..........#.    ",
                "        ...#....",
                "        .....#..",
                "        .#......",
                "        ......#.",
            ].join("\n")
        );

        assert_eq!(
            instructions,
            vec![
                Move(10),
                Right,
                Move(5),
                Left,
                Move(5),
                Right,
                Move(10),
                Left,
                Move(4),
                Right,
                Move(5),
                Left,
                Move(5),
            ]
        )
    }

    #[test]
    fn can_walk_map() {
        let (grid, instructions) = parse_input(&sample_input());

        assert_eq!(
            walk_map(&grid, &instructions),
            6032
        )
    }

    #[test]
    fn can_apply_cube_delta() {
        assert_eq!(
            apply_cube_delta(
                (1, 8),
                UP,
                (12, 16),
                4,
                &sample_faces_map()
            ),
            (0, 8, UP)
        );

        assert_eq!(
            apply_cube_delta(
                (0, 8),
                UP,
                (12, 16),
                4,
                &sample_faces_map()
            ),
            (4, 3, DOWN)
        );

        assert_eq!(
            apply_cube_delta(
                (0, 8),
                LEFT,
                (12, 16),
                4,
                &sample_faces_map()
            ),
            (4, 4, DOWN)
        );

        assert_eq!(
            apply_cube_delta(
                (4, 4),
                UP,
                (12, 16),
                4,
                &sample_faces_map()
            ),
            (0, 8, RIGHT)
        );

        assert_eq!(
            apply_cube_delta(
                (0, 11),
                RIGHT,
                (12, 16),
                4,
                &sample_faces_map()
            ),
            (11, 15, LEFT)
        );

        let real_moves: Vec<((usize, usize, Facing), (usize, usize, Facing))> = vec![
            ((0, 99, RIGHT), (0, 100, RIGHT)), // A -> B
            ((49, 50, DOWN), (50, 50, DOWN)),  // A -> C
            ((0, 50, LEFT), (149, 0, RIGHT)), // A -> D
            ((0, 50, UP), (150, 0, RIGHT)), // A -> F
            //
            ((0, 149, RIGHT), (149, 99, LEFT)), // B -> A
            ((49, 100, DOWN), (50, 99, LEFT)),  // B -> C
            ((0, 100, LEFT), (0, 99, LEFT)), // B -> E
            ((0, 100, UP), (199, 0, UP)), // B -> F
            //
            ((50, 99, RIGHT), (49, 100, UP)), // C -> B
            ((99, 50, DOWN), (100, 50, DOWN)),  // C -> E
            ((50, 50, LEFT), (100, 0, DOWN)), // C -> D
            ((50, 50, UP), (49, 50, UP)), // C -> A
            //
            ((100, 49, RIGHT), (100, 50, RIGHT)), // D -> E
            ((149, 0, DOWN), (150, 0, DOWN)),  // D -> F
            ((100, 0, LEFT), (49, 50, RIGHT)), // D -> A
            ((100, 0, UP), (50, 50, RIGHT)), // D -> C
            //
            ((100, 99, RIGHT), (49, 149, LEFT)), // E -> B
            ((149, 50, DOWN), (150, 49, LEFT)),  // E -> F
            ((100, 50, LEFT), (100, 49, LEFT)), // E -> D
            ((100, 50, UP), (99, 50, UP)), // E -> C
            //
            ((150, 49, RIGHT), (149, 50, UP)), // F -> E
            ((199, 0, DOWN), (0, 100, DOWN)),  // F -> B
            ((150, 0, LEFT), (0, 50, DOWN)), // F -> A
            ((150, 0, UP), (149, 0, UP)), // F -> D
        ];

        for ((y, x, f), expected) in real_moves {
            assert_eq!(
                apply_cube_delta(
                    (y, x),
                    f,
                    (200, 150),
                    50,
                    &actual_faces_map()
                ),
                expected
            );
        }
    }

    #[test]
    fn can_walk_cube() {
        let (grid, instructions) = parse_input(&sample_input());

        assert_eq!(
            walk_cube(&grid, &instructions, 4, &sample_faces_map()),
            5031
        )
    }
}

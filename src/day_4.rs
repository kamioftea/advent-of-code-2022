//! This is my solution for [Advent of Code - Day 4 - _Camp Cleanup_](
//! https://adventofcode.com/2022/day/4)
//!
//!

type Range = (u32, u32);
type Pair = (Range, Range);

use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-4-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 4.
pub fn run() {
    let contents = fs::read_to_string("res/day-4-input").expect("Failed to read file");
    let pairs = parse_input(&contents);

    println!(
        "There are {} redundant pairs of elves",
        count_pairs_matching(&pairs, pair_has_redundant_elf)
    );

    println!(
        "There are {} overlapping pairs of elves",
        count_pairs_matching(&pairs, pair_overlaps)
    );
}

fn parse_input(input: &String) -> Vec<Pair> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Pair {
    let parts: Vec<&str> = line.split(',').collect();
    (
        parse_range(parts[0]),
        parse_range(parts[1])
    )
}

fn parse_range(spec: &str) -> Range {
    let limits: Vec<u32> =
        spec.split('-')
            .map(|str| str.parse::<u32>().unwrap())
            .collect();

    (limits[0], limits[1])
}

fn pair_has_redundant_elf(((elf1_start, elf1_end), (elf2_start, elf2_end)): Pair) -> bool {
    (elf1_start <= elf2_start && elf1_end >= elf2_end) ||
        (elf1_start >= elf2_start && elf1_end <= elf2_end)
}

fn pair_overlaps(((elf1_start, elf1_end), (elf2_start, elf2_end)): Pair) -> bool {
    elf1_start <= elf2_end && elf1_end >= elf2_start
}

fn count_pairs_matching(pairs: &Vec<Pair>, predicate: fn(Pair) -> bool) -> usize {
    pairs.iter().filter(|&&pair| predicate(pair)).count()
}

#[cfg(test)]
mod tests {
    use crate::day_4::{count_pairs_matching, Pair, pair_has_redundant_elf, pair_overlaps, parse_input};

    fn sample_pairs() -> Vec<Pair> {
        vec![
            ((2, 4), (6, 8)),
            ((2, 3), (4, 5)),
            ((5, 7), (7, 9)),
            ((2, 8), (3, 7)),
            ((6, 6), (4, 6)),
            ((2, 6), (4, 8)),
        ]
    }

    #[test]
    fn can_parse() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8".to_string();

        assert_eq!(parse_input(&input), sample_pairs());
    }

    #[test]
    fn can_find_redundant_pairs() {
        for (pair, expected) in sample_pairs().into_iter().zip(vec![false, false, false, true, true, false]) {
            assert_eq!(pair_has_redundant_elf(pair), expected, "Check pair {:?} redundancy", pair);
        }

        let boundaries = vec![
            ((4, 4), (4, 6)),
            ((6, 6), (4, 6)),
            ((4, 6), (4, 4)),
            ((4, 6), (6, 6)),
        ];
        for pair in boundaries {
            assert_eq!(pair_has_redundant_elf(pair), true, "Check pair {:?} redundancy", pair);
        }
    }

    #[test]
    fn can_find_overlaps_pairs() {
        let possibilities = vec![
            (((4, 6), (5, 5)), true),
            (((5, 5), (4, 6)), true),
            (((4, 5), (5, 6)), true),
            (((5, 6), (4, 5)), true),
            (((4, 5), (6, 7)), false),
            (((6, 7), (4, 5)), false),
        ];
        for (pair, expected) in possibilities {
            assert_eq!(pair_overlaps(pair), expected, "Check pair {:?} overlap", pair);
        }
    }

    #[test]
    fn can_count_pairs() {
        assert_eq!(count_pairs_matching(&sample_pairs(), pair_has_redundant_elf), 2);
        assert_eq!(count_pairs_matching(&sample_pairs(), pair_overlaps), 4);
    }
}

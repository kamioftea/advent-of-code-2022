//! This is my solution for [Advent of Code - Day 3 - _Rucksack Reorganization_](https://adventofcode.com/2022/day/3)
//!
//! Today's task is to find the intersection of various character strings, then use a scoring/priority system to get an
//! aggregate of the resulting singleton sets.

use std::collections::{BTreeSet};
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-3-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 1.
pub fn run() {
    let contents = fs::read_to_string("res/day-3-input").expect("Failed to read file");
    let rucksacks: Vec<String> = parse_input(&contents);

    println!(
        "The sum of the mismatched items' priorities is: {}",
        sum_mismatched_items(&rucksacks)
    );

    println!(
        "The sum of the group badge items' priorities is: {}",
        sum_group_badge_priorities(&rucksacks)
    )
}

/// Convert the input to a list of the lines as individual strings
fn parse_input(input: &String) -> Vec<String> {
    input.lines().map(str::to_string).collect()
}

/// Get the character(S) in the second string that are also in the first string
fn intersect_strings(a: &String, b: &String) -> String {
    let set: BTreeSet<char> = BTreeSet::from_iter(a.chars());

    b.chars().filter(|c| set.contains(c)).collect()
}

/// Map the inputs to two compartments, find the singleton intersecting character, map to a priority and sum.
fn sum_mismatched_items(rucksacks: &Vec<String>) -> u32 {
    rucksacks.iter()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(a, b)| intersect_strings(&a.to_string(), &b.to_string()))
        .map(|str| map_char_to_priority(&str.chars().next().unwrap()))
        .sum()
}

/// Chunk the input lines into groups of three, find the singleton intersection of each group and sum their priorities.
fn sum_group_badge_priorities(rucksacks: &Vec<String>) -> u32 {
    rucksacks.chunks(3)
        .map(|chunk| {
            let intermediate = intersect_strings(&chunk[0], &chunk[1]);
            intersect_strings(&intermediate, &chunk[2])
        })
        .map(|str| map_char_to_priority(&str.chars().next().unwrap()))
        .sum()
}

/// Map a character to a priority based on its position in the alphabet and whether it is uppercase.
fn map_char_to_priority(c: &char) -> u32 {
    let position = 0b11111 & *c as u32;

    if c.is_uppercase() { position + 26 } else { position }
}

#[cfg(test)]
mod tests {
    use crate::day_3::{intersect_strings, map_char_to_priority, parse_input, sum_group_badge_priorities, sum_mismatched_items};

    fn get_sample_data() -> String {
        return "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw".to_string();
    }

    #[test]
    fn can_intersect_strings() {
        assert_eq!(intersect_strings(&"abcd".to_string(), &"defg".to_string()), "d".to_string());
        assert_eq!(intersect_strings(&"abcd".to_string(), &"cdef".to_string()), "cd".to_string());
        assert_eq!(intersect_strings(&"cafH".to_string(), &"wHcl".to_string()), "Hc".to_string());
        assert_eq!(intersect_strings(&"ttgJtRGJ".to_string(), &"QctTZtZT".to_string()), "tt".to_string());
    }

    #[test]
    fn can_map_to_priorities() {
        assert_eq!(
            vec!['p', 'L', 'P', 'v', 't', 's']
                .iter()
                .map(map_char_to_priority)
                .collect::<Vec<u32>>(),
            vec![16, 38, 42, 22, 20, 19]
        );

        assert_eq!(
            vec!['a', 'z', 'A', 'Z']
                .iter()
                .map(map_char_to_priority)
                .collect::<Vec<u32>>(),
            vec![1, 26, 27, 52]
        )
    }

    #[test]
    fn can_sum_mismatched_items_priorities() {
        let rucksacks: Vec<String> = get_sample_data().lines().map(|line| line.to_string()).collect();
        assert_eq!(
            sum_mismatched_items(&rucksacks),
            157
        )
    }

    #[test]
    fn can_sum_badge_priorities() {
        let rucksacks: Vec<String> = parse_input(&get_sample_data());
        assert_eq!(
            sum_group_badge_priorities(&rucksacks),
            70
        )
    }
}

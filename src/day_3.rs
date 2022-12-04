//! This is my solution for [Advent of Code - Day 3 - _Calorie Counting_](https://adventofcode.com/2022/day/1)
//!
//! The task is to sum all the calories carried per elf on our expedition and find those that are carrying the most.

use std::collections::BTreeSet;
use std::fs;


type Rucksack = (BTreeSet<char>, BTreeSet<char>);

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-3-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 1.
pub fn run() {
    let contents = fs::read_to_string("res/day-3-input").expect("Failed to read file");
    let bags = parse_input(&contents);

    println!(
        "The sum of the mismatched items' priorities is: {}",
        sum_mismatched_items(&bags)
    );

    println!(
        "The sum of the group badge items' priorities is: {}",
        sum_group_badge_priorities(&bags)
    )
}

fn parse_input(input: &String) -> Vec<Rucksack> {
    input.lines()
         .map(parse_rucksack)
         .collect()
}

fn parse_rucksack(line: &str) -> Rucksack {
    let (a, b) = line.split_at(line.len() / 2);
    (parse_compartment(a), parse_compartment(b))
}

fn parse_compartment(line_half: &str) -> BTreeSet<char> {
    line_half.chars().collect()
}

fn find_item_to_rearrange((a, b): &Rucksack) -> &char {
    a.intersection(&b).into_iter().next().unwrap()
}

fn map_char_to_priority(c: &char) -> u32 {
    match c {
        'a'..='z' => 31 & *c as u32,
        'A'..='Z' => (31 & *c as u32) + 26,
        _ => unreachable!()
    }
}

fn sum_mismatched_items(bags: &Vec<Rucksack>) -> u32 {
    bags.iter()
        .map(find_item_to_rearrange)
        .map(map_char_to_priority)
        .sum()
}

fn sum_group_badge_priorities(bags: &Vec<Rucksack>) -> u32 {
    let mut sum = 0;
    let mut iter = bags.into_iter();
    while let Some(base) = iter.next() {
        let mut items: BTreeSet<char> = get_rucksack_items(&base);
        items = items.intersection(&get_rucksack_items(iter.next().unwrap())).into_iter().map(|&c| c).collect();
        items = items.intersection(&get_rucksack_items(iter.next().unwrap())).into_iter().map(|&c| c).collect();

        let c = items.iter().next().unwrap();
        sum = sum + map_char_to_priority(c)
    }
    sum
}

fn get_rucksack_items((a, b): &Rucksack) -> BTreeSet<char> {
    a.union(b).into_iter().map(|&c| c).collect()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use crate::day_3::{find_item_to_rearrange, map_char_to_priority, parse_input, parse_rucksack, sum_group_badge_priorities, sum_mismatched_items};

    fn get_sample_data() -> String {
        return "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw".to_string();
    }

    fn get_sample_items() -> Vec<char> {
        vec!['p', 'L', 'P', 'v', 't', 's']
    }

    #[test]
    fn can_parse_rucksack() {
        assert_eq!(
            parse_rucksack("ttgJtRGJQctTZtZT"),
            (
                vec!['t', 'g', 'J', 'R', 'G'].into_iter().collect::<BTreeSet<char>>(),
                vec!['Q', 'c', 't', 'T', 'Z'].into_iter().collect::<BTreeSet<char>>()
            )
        );
    }

    #[test]
    fn can_find_item_to_rearrange() {
        assert_eq!(
            parse_input(&get_sample_data())
                .iter()
                .map(find_item_to_rearrange)
                .collect::<Vec<&char>>(),
            get_sample_items().iter().collect::<Vec<&char>>()
        )
    }

    #[test]
    fn can_map_to_priorities() {
        assert_eq!(
            get_sample_items()
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
        assert_eq!(
            sum_mismatched_items(&parse_input(&get_sample_data())),
            157
        )
    }

//     #[test]
//     fn can_find_group_badges() {
//         let string = "vJrwpWtwJgWrhcsFMMfFFhFp
// jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
// PmmdzqPrVvPwwTWBwg".to_string();
//         assert_eq!(find_group_badge(&parse_input(&string)), &'r');
//
//         let string = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
// ttgJtRGJQctTZtZT
// CrZsJsPPZsGzwwsLwLmpwMDw".to_string();
//         assert_eq!(find_group_badge(&parse_input(&string)), &'Z')
//     }

    #[test]
    fn can_sum_badge_priorities() {
        assert_eq!(
            sum_group_badge_priorities(&parse_input(&get_sample_data())),
            70
        )
    }
}

//! This is my solution for [Advent of Code - Day 6 - _Title_](
//! https://adventofcode.com/2022/day/6)
//!
//!

use std::collections::BTreeSet;
use std::fs;
use itertools::Itertools;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-6-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 6.
pub fn run() {
    let _contents = fs::read_to_string("res/day-6-input").expect("Failed to read file");

    let start_of_packet = find_start_of_packet(&_contents);
    println!("The start of packet is detected after {} characters", start_of_packet);
}

fn find_start_of_packet(datastream: &String) -> usize {
    let (i, _) = datastream.chars().tuple_windows().enumerate().find(
        |(_, (a, b, c, d))| is_unique(a,b,c,d)
    ).unwrap();

    i + 4
}

fn is_unique(a: &char, b: &char, c:&char, d:&char) -> bool {
    let mut set = BTreeSet::new();
    set.insert(a);
    set.insert(b);
    set.insert(c);
    set.insert(d);

    set.len() == 4
}

#[cfg(test)]
mod tests {
    use crate::day_6::find_start_of_packet;

    #[test]
    fn can_parse() {
        assert_eq!(find_start_of_packet(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()), 7);
        assert_eq!(find_start_of_packet(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()), 5);
        assert_eq!(find_start_of_packet(&"nppdvjthqldpwncqszvftbrmjlhg".to_string()), 6);
        assert_eq!(find_start_of_packet(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()), 10);
        assert_eq!(find_start_of_packet(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()), 11);
    }
}

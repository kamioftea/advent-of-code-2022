//! This is my solution for [Advent of Code - Day 6 - _Title_](
//! https://adventofcode.com/2022/day/6)
//!
//!

use std::collections::BTreeSet;
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-6-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 6.
pub fn run() {
    let _contents = fs::read_to_string("res/day-6-input").expect("Failed to read file");

    let start_of_packet = find_non_repeating_string_of_length(&_contents, 4);
    println!("The start of packet is detected after {} characters", start_of_packet);

    let start_of_packet = find_non_repeating_string_of_length(&_contents, 14);
    println!("The start of packet is detected after {} characters", start_of_packet);
}

fn find_non_repeating_string_of_length(datastream: &String, window_size: usize) -> usize {
    let chars: Vec<char> = datastream.chars().collect();
    let (i, _) = chars.windows(window_size).enumerate().find(
        |(_, window)| is_unique(window)
    ).unwrap();

    i + window_size
}

fn is_unique(window: &[char]) -> bool {
    let mut set = BTreeSet::new();
    window.iter().for_each(|c| {set.insert(c);});

    set.len() == window.len()
}

#[cfg(test)]
mod tests {
    use crate::day_6::find_non_repeating_string_of_length;

    #[test]
    fn can_find_start_of_packet() {
        assert_eq!(find_non_repeating_string_of_length(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(), 4), 7);
        assert_eq!(find_non_repeating_string_of_length(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 4), 5);
        assert_eq!(find_non_repeating_string_of_length(&"nppdvjthqldpwncqszvftbrmjlhg".to_string(), 4), 6);
        assert_eq!(find_non_repeating_string_of_length(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 4), 10);
        assert_eq!(find_non_repeating_string_of_length(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 4), 11);
    }

    #[test]
    fn can_find_start_of_message() {
        assert_eq!(find_non_repeating_string_of_length(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(), 14), 19);
        assert_eq!(find_non_repeating_string_of_length(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 14), 23);
        assert_eq!(find_non_repeating_string_of_length(&"nppdvjthqldpwncqszvftbrmjlhg".to_string(), 14), 23);
        assert_eq!(find_non_repeating_string_of_length(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 14), 29);
        assert_eq!(find_non_repeating_string_of_length(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 14), 26);
    }
}

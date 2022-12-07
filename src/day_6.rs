//! This is my solution for [Advent of Code - Day 6 - _Tuning Trouble_](https://adventofcode.com/2022/day/6)
//!
//!

use std::collections::{HashMap};
use std::fs;
use itertools::Itertools;

struct Counts {
    counts: HashMap<char, usize>,
}

impl Counts {
    fn new(init: &str) -> Self {
        Self {
            counts: init.chars().counts_by(|c| c)
        }
    }

    fn add_and_remove(&mut self, to_add: &char, to_remove: &char) {
        if to_add == to_remove {
            return;
        }

        self.add(to_add);
        self.remove(to_remove);
    }

    fn add(&mut self, to_add: &char) {
        let new_to_add_count = self.counts.get(to_add).unwrap_or(&0) + 1;
        self.counts.insert(*to_add, new_to_add_count);
    }

    fn remove(&mut self, to_remove: &char) {
        let new_to_remove_count = self.counts.get(to_remove).unwrap() - 1;
        if new_to_remove_count == 0 {
            self.counts.remove(to_remove);
        } else {
            self.counts.insert(*to_remove, new_to_remove_count);
        }
    }

    fn len(&self) -> usize {
        self.counts.len()
    }
}

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

fn find_non_repeating_string_of_length(data_stream: &String, window_size: usize) -> usize {
    let (init, rest) = data_stream.split_at(window_size);
    let mut counts = Counts::new(init);

    for (i, (to_add, to_remove))
    in rest.chars()
           .zip(data_stream.chars())
           .enumerate()
    {
        counts.add_and_remove(&to_add, &to_remove);

        if counts.len() == window_size {
            return i + window_size + 1;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use crate::day_6::find_non_repeating_string_of_length;

    #[test]
    fn can_find_start_of_packet() {
        assert_eq!(find_non_repeating_string_of_length(
            &"mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(), 4),
                   7
        );
        assert_eq!(find_non_repeating_string_of_length(
            &"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 4),
                   5
        );
        assert_eq!(find_non_repeating_string_of_length(
            &"nppdvjthqldpwncqszvftbrmjlhg".to_string(), 4),
                   6
        );
        assert_eq!(find_non_repeating_string_of_length(
            &"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 4),
                   10
        );
        assert_eq!(find_non_repeating_string_of_length(
            &"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 4),
                   11
        );
    }

    #[test]
    fn can_find_start_of_message() {
        assert_eq!(find_non_repeating_string_of_length(
            &"mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(), 14),
                   19
        );
        assert_eq!(find_non_repeating_string_of_length(
            &"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 14),
                   23
        );
        assert_eq!(find_non_repeating_string_of_length(
            &"nppdvjthqldpwncqszvftbrmjlhg".to_string(), 14),
                   23
        );
        assert_eq!(find_non_repeating_string_of_length(
            &"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 14),
                   29
        );
        assert_eq!(find_non_repeating_string_of_length(
            &"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 14),
                   26
        );
    }
}

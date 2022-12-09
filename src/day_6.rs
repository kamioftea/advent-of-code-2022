//! This is my solution for [Advent of Code - Day 6 - _Tuning Trouble_](https://adventofcode.com/2022/day/6)
//!
//! Find substrings with a unique set of characters in a much larger string

use std::collections::{HashMap};
use std::fs;
use itertools::Itertools;

/// Represents a window of characters over a data stream by their counts
struct Counts {
    counts: HashMap<char, usize>,
}

impl Counts {
    /// Create a Counts instance for the first window  in the stream
    fn new(init: &str) -> Self {
        Self {
            counts: init.chars().counts_by(|c| c)
        }
    }

    /// Advance the window by one character by adding the next character in the stream and removing the one that
    /// falls out.
    fn add_and_remove(&mut self, to_add: &char, to_remove: &char) {
        // If the characters are the same this is a no-op
        if to_add == to_remove {
            return;
        }

        self.add(to_add);
        self.remove(to_remove);
    }

    /// Increment the count for `to_add` - adding it to the map if new
    fn add(&mut self, to_add: &char) {
        let new_to_add_count = self.counts.get(to_add).unwrap_or(&0) + 1;
        self.counts.insert(*to_add, new_to_add_count);
    }

    /// Decrement the count for `to_remove` - remove from the map if the count is now 0
    fn remove(&mut self, to_remove: &char) {
        let new_to_remove_count = self.counts.get(to_remove).unwrap() - 1;
        if new_to_remove_count == 0 {
            self.counts.remove(to_remove);
        } else {
            self.counts.insert(*to_remove, new_to_remove_count);
        }
    }

    /// The number of unique characters in the map is the same as its length
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

    let start_of_message = find_non_repeating_string_of_length(&_contents, 14);
    println!("The start of packet is detected after {} characters", start_of_message);
}

/// Find the first substring of unique consecutive characters with length `window_size`
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
        let examples = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];

        for (data_stream, expected) in examples {
            assert_eq!(
                find_non_repeating_string_of_length(&data_stream.to_string(), 4),
                expected
            )
        }
    }

    #[test]
    fn can_find_start_of_message() {
        let examples = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];

        for (data_stream, expected) in examples {
            assert_eq!(
                find_non_repeating_string_of_length(&data_stream.to_string(), 14),
                expected
            )
        }
    }
}

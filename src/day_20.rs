//! This is my solution for [Advent of Code - Day 20 - _Title_](https://adventofcode.com/2022/day/20)
//!
//!

use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

struct IndexedNumber {
    value: i64,
    mix_index: RefCell<usize>,
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-20-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 20.
pub fn run() {
    let contents = fs::read_to_string("res/day-20-input").expect("Failed to read file");
    let numbers = parse_input(&contents);

    println!(
        "The sm of the grove coordinates is {}",
        grove_coords_sum(&numbers, 1, 1)
    );

    println!(
        "The sm of the grove coordinates is {}",
        grove_coords_sum(&numbers, 10, 811589153)
    );
}

fn parse_input(input: &String) -> Vec<i64> {
    input.lines().map(|l| l.parse::<i64>().unwrap()).collect()
}

fn mix(numbers: &Vec<i64>, cycles: usize) -> Vec<i64> {
    let mut to_mix = Vec::new();
    let mut process_order = Vec::new();

    for (idx, &value) in numbers.into_iter().enumerate() {
        let list_item = Rc::new(
            IndexedNumber {
                value,
                mix_index: RefCell::new(idx),
            }
        );
        to_mix.push(list_item.clone());
        process_order.push(list_item);
    }

    for _ in 0..cycles {
        perform_cycle(&process_order, &mut to_mix)
    }

    to_mix.iter().map(|item| item.value).collect()
}

fn perform_cycle(process_order: &Vec<Rc<IndexedNumber>>, to_mix: &mut Vec<Rc<IndexedNumber>>) {
    let wrap_when_moving = process_order.len() - 1;

    for item in process_order {
        let mut current_position = item.mix_index.borrow_mut();
        let target_position = mod_add(*current_position, item.value, wrap_when_moving);

        if target_position < *current_position {
            for i in target_position..*current_position {
                to_mix[i].mix_index.replace_with(|&mut idx| idx + 1);
            }
        } else if target_position > *current_position {
            for i in (*current_position + 1)..=target_position {
                to_mix[i].mix_index.replace_with(|&mut idx| idx - 1);
            }
        }

        let to_move = to_mix.remove(*current_position);
        to_mix.insert(target_position, to_move);

        *current_position = target_position
    }
}

fn mod_add(start: usize, delta: i64, len: usize) -> usize {
    if delta == 0 {
        return start
    }

    let i_start = i64::try_from(start).unwrap();
    let unwrapped = i_start + delta;
    let i_len = i64::try_from(len).unwrap();
    let mut new_pos = unwrapped % i_len;
    if new_pos < 1 {
        new_pos = new_pos + i_len;
    }

    usize::try_from(new_pos).unwrap()
}

fn grove_coords_sum(encrypted: &Vec<i64>, cycles: usize, decryption_key: i64) -> i64 {
    let with_key = encrypted.iter().map(|&v| v * decryption_key).collect();
    let mixed = mix(&with_key, cycles);
    let zero = mixed.iter().position(|&v| v == 0).unwrap();

    mixed[(zero + 1000) % mixed.len()] +
        mixed[(zero + 2000) % mixed.len()] +
        mixed[(zero + 3000) % mixed.len()]
}

#[cfg(test)]
mod tests {
    use crate::day_20::{grove_coords_sum, mix, parse_input};

    fn sample_numbers() -> Vec<i64> {
        vec![1, 2, -3, 3, -2, 0, 4]
    }

    #[test]
    fn can_parse() {
        let input = "1
2
-3
3
-2
0
4".to_string();

        assert_eq!(parse_input(&input), sample_numbers());
    }

    #[test]
    fn can_mix_numbers() {
        assert_eq!(mix(&sample_numbers(), 1), vec![1, 2, -3, 4, 0, 3, -2]);

        let with_key = sample_numbers().iter().map(|&v| v * 811589153).collect();

        let round_1 = mix(&with_key, 1);
        let round_4 = mix(&with_key, 4);
        let round_10 = mix(&with_key, 10);

        assert_eq!(
            round_1,
            vec![0, -2434767459, 3246356612, -1623178306, 2434767459, 1623178306, 811589153]
        );

        assert_eq!(
            round_4,
            vec![0, 1623178306, -2434767459, 811589153, 2434767459, 3246356612, -1623178306]
        );

        assert_eq!(
            round_10,
            vec![0, -2434767459, 1623178306, 3246356612, -1623178306, 2434767459, 811589153]
        )
    }

    #[test]
    fn can_sum_coords() {
        assert_eq!(grove_coords_sum(&sample_numbers(), 1, 1), 3);
        assert_eq!(grove_coords_sum(&sample_numbers(), 10, 811589153), 1623178306);
    }
}

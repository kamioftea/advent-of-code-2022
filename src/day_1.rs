//! This is my solution for [Advent of Code - Day 1 - _Calorie Counting_](https://adventofcode.com/2022/day/1)
//!
//! The task is to sum all the calories carried per elf on our expedition and find those that are carrying the most.

use std::fs;

/// An elf represented by the total calories in their combined food items
type CalorieTotal = u32;

/// The whole expedition's supplies: a list of Elves' calorie totals
type Expedition = Vec<CalorieTotal>;


/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-1-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 1.
pub fn run() {
    let contents = fs::read_to_string("res/day-1-input").expect("Failed to read file");
    let elf_calorie_totals = parse_input(&contents);
    let (first, second, third) = find_top_three_calorie_totals(&elf_calorie_totals);

    println!(
        "The most calories carried by one elf is: {}",
        first
    );

    println!(
        "The total calories carried by the top three elves is: {}",
        first + second + third
    );
}

/// Turn the input text file into a list of elves by their total carried calories
fn parse_input(input: &String) -> Expedition {
    let mut expedition = Vec::new();
    let mut current_calorie_total = 0;

    input.lines().for_each(
        |line| match line.parse::<u32>() {
            Ok(calories) => current_calorie_total = current_calorie_total + calories,
            Err(_) => {
                expedition.push(current_calorie_total);
                current_calorie_total = 0;
            }
        }
    );

    if current_calorie_total > 0 {
        expedition.push(current_calorie_total)
    }

    expedition
}

/// Extract the calorie totals of just the three elves with the most food.
fn find_top_three_calorie_totals(expedition: &Expedition) -> (CalorieTotal, CalorieTotal, CalorieTotal) {
    expedition.iter().fold((0, 0, 0), bubble_calorie_total_into_top_three)
}

/// Insert the next elf into place in the current top three if its calorie total is high enough
fn bubble_calorie_total_into_top_three(
    top_3: (CalorieTotal, CalorieTotal, CalorieTotal),
    &next_elf: &CalorieTotal
) -> (CalorieTotal, CalorieTotal, CalorieTotal) {
    match top_3 {
        (a, b, _) if a < next_elf => (next_elf, a, b),
        (a, b, _) if b < next_elf => (a, next_elf, b),
        (a, b, c) if c < next_elf => (a, b, next_elf),
        _ => top_3
    }
}

#[cfg(test)]
mod tests {
    use crate::day_1::{Expedition, find_top_three_calorie_totals, parse_input};

    fn sample_expedition() -> Expedition {
        vec![6000, 4000, 11000, 24000, 10000]
    }

    #[test]
    fn can_parse_sample_input() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000".to_string();

        assert_eq!(parse_input(&input), sample_expedition());
    }

    #[test]
    fn can_find_top_three_calories() {
        assert_eq!(find_top_three_calorie_totals(&sample_expedition()), (24000, 11000, 10000))
    }
}

//! This is my solution for [Advent of Code - Day 25 - _Title_](https://adventofcode.com/2022/day/25)
//!
//!

use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-25-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 25.
pub fn run() {
    let contents = fs::read_to_string("res/day-25-input").expect("Failed to read file");
    let decimals = parse_input(&contents);

    println!(
        "The number to enter is: {}",
        sum_and_render(&decimals),
    );
}

fn to_snafu(decimal: isize) -> String {
    fn iter(decimal: isize, acc: String) -> String {
        if decimal == 0 {
            return acc;
        }

        let (digit, carry) = match decimal % 5 {
            0 => ('0', 0),
            1 => ('1', 0),
            2 => ('2', 0),
            3 => ('=', 1),
            4 => ('-', 1),
            _ => unreachable!("n % 5")
        };

        iter(
            decimal / 5 + carry,
            format!("{digit}{acc}"),
        )
    }

    iter(decimal, "".to_string())
}

fn from_snafu(snafu: String) -> isize {
    snafu.chars().fold(
        0,
        |acc, digit|
            acc * 5 + match digit {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '-' => -1,
                '=' => -2,
                d => unreachable!("{} is not a SNAFU digit", d)
            },
    )
}

fn sum_and_render(decimals: &Vec<isize>) -> String {
    to_snafu(decimals.into_iter().sum())
}

fn parse_input(input: &String) -> Vec<isize> {
    input.lines()
         .map(|line| from_snafu(line.to_string()))
         .collect()
}

#[cfg(test)]
mod tests {
    use crate::day_25::{from_snafu, parse_input, sum_and_render, to_snafu};

    fn sample_decimals() -> Vec<isize> {
        vec![
            1747,
            906,
            198,
            11,
            201,
            31,
            1257,
            32,
            353,
            107,
            7,
            3,
            37,
        ]
    }

    #[test]
    fn can_convert_snafus() {
        let examples = vec![
            (1, "1".to_string()),
            (2, "2".to_string()),
            (3, "1=".to_string()),
            (4, "1-".to_string()),
            (5, "10".to_string()),
            (6, "11".to_string()),
            (7, "12".to_string()),
            (8, "2=".to_string()),
            (9, "2-".to_string()),
            (10, "20".to_string()),
            (15, "1=0".to_string()),
            (20, "1-0".to_string()),
            (2022, "1=11-2".to_string()),
            (12345, "1-0---0".to_string()),
            (314159265, "1121-1110-1=0".to_string()),
        ];

        for (decimal, snafu) in examples {
            assert_eq!(to_snafu(decimal), snafu);
            assert_eq!(from_snafu(snafu), decimal);
        }
    }

    #[test]
    fn can_parse() {
        let input = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122".to_string();

        assert_eq!(parse_input(&input), sample_decimals());
    }

    #[test]
    fn can_sum_to_snafu() {
        assert_eq!(
            sum_and_render(&sample_decimals()),
            "2=-1=0".to_string()
        )
    }
}

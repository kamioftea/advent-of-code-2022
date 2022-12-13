//! This is my solution for [Advent of Code - Day 13 - _Distress Signal_](https://adventofcode.com/2022/day/13)
//!
//!

use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::fs;
use itertools::Itertools;
use crate::day_13::NestedList::{List, Value};

#[derive(Eq, PartialEq, Debug, Clone)]
enum NestedList {
    Value(u32),
    List(Vec<NestedList>),
}

impl NestedList {
    fn push(&mut self, item: NestedList) {
        match self {
            List(l) => l.push(item),
            Value(_) => panic!("Push not impl for Value")
        }
    }
}

impl Ord for NestedList {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value(va), Value(vb)) => va.cmp(vb),
            (&Value(va), List(_)) => List(vec![Value(va)]).cmp(other),
            (List(_), &Value(vb)) => self.cmp(&List(vec![Value(vb)])),
            (List(la), List(lb)) => {
                let res = la.iter().zip(lb)
                    .map(|(a, b)| a.cmp(b))
                    .find_or_last(|&o| o != Equal)
                    .unwrap_or(Equal);

                if res == Equal {
                    la.len().cmp(&lb.len())
                } else {
                    res
                }
            }
        }
    }
}

impl PartialOrd for NestedList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-13-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 13.
pub fn run() {
    let contents = fs::read_to_string("res/day-13-input").expect("Failed to read file");
    let pairs = parse_input(&contents);

    println!(
        "The sum of in order indices is: {}",
        find_in_order_index_sum(&pairs)
    );

    println!(
        "The decoder key is: {}",
        find_decoder_key(&pairs)
    );
}

fn parse_input(input: &String) -> Vec<(NestedList, NestedList)> {
    input.split("\n\n").map(parse_pair).collect()
}

fn parse_pair(spec: &str) -> (NestedList, NestedList) {
    let (left, right) = spec.split_once("\n").unwrap();
    (parse_list(left), parse_list(right))
}

fn parse_list(spec: &str) -> NestedList {
    let mut stack = Vec::new();
    let mut current = List(Vec::new());
    for c in spec.trim().chars() {
        match c {
            '[' => {
                stack.push(current);
                current = List(Vec::new());
            }
            ']' => {
                match current {
                    List(l) => {
                        current = stack.pop().unwrap();
                        current.push(List(l));
                    }
                    Value(v) => {
                        current = stack.pop().unwrap();
                        current.push(Value(v));

                        let temp = current.clone();
                        current = stack.pop().unwrap();
                        current.push(temp.clone());
                    }
                }
            }
            ',' => {
                if let Value(v) = current {
                    current = stack.pop().unwrap();
                    current.push(Value(v));
                }
            }
            c if c.is_digit(10) => {
                match current {
                    List(l) => {
                        stack.push(List(l));
                        current = Value(c.to_digit(10).unwrap())
                    }
                    Value(v) => {
                        current = Value(v * 10 + c.to_digit(10).unwrap())
                    }
                }
            }
            _ => unreachable!()
        }
    }

    match current {
        List(l) => l.first().unwrap().clone(),
        _ => unreachable!()
    }
}

fn in_order(a: &NestedList, b: &NestedList) -> bool {
    a < b
}

fn find_in_order_index_sum(pairs: &Vec<(NestedList, NestedList)>) -> usize {
    pairs
        .iter()
        .enumerate()
        .filter(|(_, (a, b))| in_order(a, b))
        .map(|(i, _)| i + 1)
        .sum()
}

fn find_decoder_key(pairs: &Vec<(NestedList, NestedList)>) -> usize {
    let divider_packets = vec![
        List(vec![List(vec![Value(2)])]),
        List(vec![List(vec![Value(6)])]),
    ];

    pairs.iter()
         .flat_map(|(a, b)| vec![a, b])
         .chain(&divider_packets)
         .sorted()
         .enumerate()
         .filter(|(_, list)| divider_packets.contains(list))
         .map(|(i, _)| i + 1)
         .reduce(|a, b| a * b)
         .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day_13::{find_decoder_key, find_in_order_index_sum, in_order, NestedList, parse_input};
    use crate::day_13::NestedList::{List, Value};

    #[test]
    fn can_parse() {
        let input = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]".to_string();

        let actual = parse_input(&input);
        for (a, e) in actual.iter().zip(sample_pairs()) {
            assert_eq!(*a, e);
        }
    }

    fn sample_pairs() -> Vec<(NestedList, NestedList)> {
        vec![
            (
                List(vec![Value(1), Value(1), Value(3), Value(1), Value(1)]),
                List(vec![Value(1), Value(1), Value(5), Value(1), Value(1)])
            ),
            (
                List(vec![List(vec![Value(1)]), List(vec![Value(2), Value(3), Value(4)])]),
                List(vec![List(vec![Value(1)]), Value(4)])
            ),
            (
                List(vec![Value(9)]),
                List(vec![List(vec![Value(8), Value(7), Value(6)])])
            ),
            (
                List(vec![List(vec![Value(4), Value(4)]), Value(4), Value(4)]),
                List(vec![List(vec![Value(4), Value(4)]), Value(4), Value(4), Value(4)])
            ),
            (
                List(vec![Value(7), Value(7), Value(7), Value(7)]),
                List(vec![Value(7), Value(7), Value(7)])
            ),
            (
                List(vec![]),
                List(vec![Value(3)])
            ),
            (
                List(vec![List(vec![List(vec![])])]),
                List(vec![List(vec![])])
            ),
            (
                List(vec![Value(1), List(vec![Value(2), List(vec![Value(3), List(vec![Value(4), List(vec![Value(5), Value(6), Value(7)])])])]), Value(8), Value(9)]),
                List(vec![Value(1), List(vec![Value(2), List(vec![Value(3), List(vec![Value(4), List(vec![Value(5), Value(6), Value(0)])])])]), Value(8), Value(9)])
            ),
        ]
    }

    #[test]
    fn can_compare() {
        let expected = vec![true, true, false, true, false, true, false];

        for ((a, b), expected_result) in sample_pairs().iter().zip(expected) {
            assert_eq!(in_order(a, b), expected_result, "in_order({:?}, {:?}) should be {}", a, b, expected_result);
        }
    }

    #[test]
    fn can_find_in_order() {
        assert_eq!(find_in_order_index_sum(&sample_pairs()), 13)
    }

    #[test]
    fn can_find_decoder_key() {
        assert_eq!(find_decoder_key(&sample_pairs()), 140)
    }
}

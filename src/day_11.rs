//! This is my solution for [Advent of Code - Day 11 - _Title_](https://adventofcode.com/2022/day/11)
//!
//!

use std::fs;
use itertools::Itertools;
use crate::day_11::Operand::Value;
use crate::day_11::Operation::{Add, Mul};

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Operand {
    Value(isize),
    Old,
}

impl Operand {
    fn apply(&self, item: isize) -> isize {
        match self {
            &Operand::Old => item,
            &Value(val) => val,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Operation {
    Mul(Operand, Operand),
    Add(Operand, Operand),
}

impl Operation {
    fn apply(&self, item: isize) -> isize {
        match self {
            &Mul(a, b) => a.apply(item) * b.apply(item),
            &Add(a, b) => a.apply(item) + b.apply(item),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Test {
    divisor: isize,
    if_true: usize,
    if_false: usize,
}

impl Test {
    fn apply(&self, worry: isize) -> usize {
        if worry % self.divisor == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Monkey {
    items: Vec<isize>,
    operation: Operation,
    test: Test,
    handling_count: usize,
}

impl From<&str> for Monkey {
    fn from(spec: &str) -> Self {
        let mut lines = spec.lines();

        // Monkey: <id>
        lines.next();

        //   Starting items: 79, 60, 97
        let (_, item_spec) = lines.next().unwrap().split_once(": ").unwrap();
        let items: Vec<isize> =
            item_spec.split(", ")
                     .map(|item| item.parse::<isize>().unwrap())
                     .collect();

        // Operation: new = old * 19
        let (_, op_spec) = lines.next().unwrap().split_once("new = ").unwrap();
        let operation = op_spec.into();

        //Test: divisible by 19
        let divisor =
            lines.next().unwrap()
                 .split_whitespace()
                 .dropping(3).next().unwrap()
                 .parse::<isize>().unwrap();

        // If true: throw to monkey 2
        let if_true = parse_branch(lines.next().unwrap());
        // If false: throw to monkey 3
        let if_false = parse_branch(lines.next().unwrap());

        Monkey {
            items,
            operation,
            test: Test { divisor, if_true, if_false },
            handling_count: 0,
        }
    }
}

impl From<&str> for Operation {
    fn from(spec: &str) -> Self {
        let mut parts = spec.split_whitespace();
        let a = parse_operand(parts.next().unwrap());
        let op = parts.next().unwrap();
        let b = parse_operand(parts.next().unwrap());

        match op {
            "+" => Add(a, b),
            "*" => Mul(a, b),
            _ => unreachable!()
        }
    }
}

fn parse_operand(spec: &str) -> Operand {
    match spec {
        "old" => Operand::Old,
        i => Value(i.parse().unwrap())
    }
}

fn parse_branch(spec: &str) -> usize {
    spec.split_whitespace()
        .dropping(5).next().unwrap()
        .parse::<usize>().unwrap()
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-11-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 11.
pub fn run() {
    let contents =
        fs::read_to_string("res/day-11-input").expect("Failed to read file");
    let mut monkeys = parse_input(&contents);

    println!(
        "After twenty rounds the top two monkeys have a monkey business score of: {}",
        get_monkey_business_level(&mut monkeys.clone(), 20, 3),
    );

    println!(
        "After 10,000 rounds without worry reduction, the top two monkeys have a score of: {}",
        get_monkey_business_level(&mut monkeys, 10000, 1),
    )
}

fn parse_input(input: &String) -> Vec<Monkey> {
    input.split("\n\n").map_into().collect()
}

fn simulate_round(monkeys: &mut Vec<Monkey>, worry_divisor: isize, common_denominator: isize) {
    for i in 0..monkeys.len() {
        let mut monkey = monkeys.get_mut(i).unwrap();
        let current_items = monkey.items.clone();
        let operation = monkey.operation;
        let test = monkey.test;

        monkey.items = Vec::new();
        monkey.handling_count = monkey.handling_count + current_items.len();

        for item in current_items {
            let worry = (operation.apply(item) / worry_divisor) % common_denominator;
            monkeys.get_mut(test.apply(worry)).unwrap().items.push(worry);
        }
    }
}

fn get_monkey_business_level(
    mut monkeys: &mut Vec<Monkey>,
    rounds: usize,
    worry_divisor: isize,
) -> usize {
    let common_denominator =
        monkeys
            .iter()
            .map(|m| m.test.divisor)
            .reduce(|acc, div| acc * div).unwrap();

    for _ in 0..rounds {
        simulate_round(&mut monkeys, worry_divisor, common_denominator)
    }

    monkeys
        .iter()
        .map(|m| m.handling_count)
        .sorted().rev().take(2)
        .reduce(|acc, monkey| acc * monkey).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day_11::{get_monkey_business_level, Monkey, parse_input, simulate_round, Test};
    use crate::day_11::Operand::{Old, Value};
    use crate::day_11::Operation::{Add, Mul};

    fn sample_monkeys() -> Vec<Monkey> {
        vec![
            Monkey {
                items: vec![79, 98],
                operation: Mul(Old, Value(19)),
                test: Test { divisor: 23, if_true: 2, if_false: 3 },
                handling_count: 0,
            },
            Monkey {
                items: vec![54, 65, 75, 74],
                operation: Add(Old, Value(6)),
                test: Test { divisor: 19, if_true: 2, if_false: 0 },
                handling_count: 0,
            },
            Monkey {
                items: vec![79, 60, 97],
                operation: Mul(Old, Old),
                test: Test { divisor: 13, if_true: 1, if_false: 3 },
                handling_count: 0,
            },
            Monkey {
                items: vec![74],
                operation: Add(Old, Value(3)),
                test: Test { divisor: 17, if_true: 0, if_false: 1 },
                handling_count: 0,
            },
        ]
    }

    #[test]
    fn can_parse() {
        let sample_input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1".to_string();

        assert_eq!(parse_input(&sample_input), sample_monkeys())
    }

    fn get_sample_common_denominator() -> isize {
        sample_monkeys()
            .iter()
            .map(|m| m.test.divisor)
            .reduce(|acc, div| acc * div).unwrap()
    }

    #[test]
    fn can_simulate_round() {
        let mut monkeys = sample_monkeys();
        simulate_round(&mut monkeys, 3, get_sample_common_denominator());
        let item_lists: Vec<Vec<isize>> = monkeys.iter().map(|m| m.items.clone()).collect();
        assert_eq!(
            item_lists,
            vec![
                vec![20, 23, 27, 26],
                vec![2080, 25, 167, 207, 401, 1046],
                vec![],
                vec![],
            ]
        )
    }

    #[test]
    fn can_find_monkey_business_level() {
        let mut monkeys = sample_monkeys();
        assert_eq!(
            get_monkey_business_level(&mut monkeys.clone(), 20, 3),
            10605,
        );

        assert_eq!(
            get_monkey_business_level(&mut monkeys, 10000, 1),
            2713310158,
        )
    }
}

//! This is my solution for [Advent of Code - Day 21 - _Title_](https://adventofcode.com/2022/day/21)
//!
//!

use std::collections::HashMap;
use std::fs;
use crate::day_21::Monkey::{Op, Value};
use crate::day_21::Operation::{Left, Right};
use crate::day_21::OperationChain::{Chain, Operand};
use crate::day_21::Operator::{Add, Div, Mul, Sub};

#[derive(Eq, PartialEq, Debug, Clone)]
enum Monkey {
    Value(isize),
    Op(String, Operator, String),
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl From<&str> for Operator {
    fn from(op: &str) -> Self {
        match op {
            "+" => Add,
            "-" => Sub,
            "*" => Mul,
            "/" => Div,
            _ => unreachable!()
        }
    }
}

impl Operator {
    fn apply(&self, left: isize, right: isize) -> isize {
        match self {
            Add => left + right,
            Sub => left - right,
            Mul => left * right,
            Div => left / right,
        }
    }

    fn inverse(&self) -> Self {
        match self {
            Add => Sub,
            Sub => Add,
            Mul => Div,
            Div => Mul,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Operation {
    Left(Operator, isize),
    Right(Operator, isize),
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum OperationChain {
    Operand(isize),
    Chain(Vec<Operation>)
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-21-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 21.
pub fn run() {
    let contents =
        fs::read_to_string("res/day-21-input").expect("Failed to read file");
    let monkeys = parse_input(&contents);

    println!(
        "The root monkey yells: {}",
        resolve(&monkeys, &"root".to_string())
    );

    println!(
        "I need to yell: {}",
        determine_value_to_shout(&monkeys)
    );
}

fn parse_input(input: &String) -> HashMap<String, Monkey> {
    let mut monkeys = HashMap::new();

    for line in input.lines() {
        let (id, spec) = line.split_once(": ").unwrap();
        monkeys.insert(id.to_string(), parse_monkey(spec));
    }

    monkeys
}

fn parse_monkey(spec: &str) -> Monkey {
    let parts: Vec<&str> = spec.split_whitespace().collect();

    if parts.len() == 1 {
        return Value(parts[0].parse::<isize>().unwrap());
    }

    Op(parts[0].into(), parts[1].into(), parts[2].into())
}

fn resolve(monkeys: &HashMap<String, Monkey>, monkey_id: &String) -> isize {
    match monkeys.get(monkey_id).unwrap() {
        Value(v) => *v,
        Op(a, op, b) => op.apply(resolve(monkeys, a), resolve(monkeys, b)),
    }
}

fn unwrap_equation(monkeys: &HashMap<String, Monkey>, monkey_id: &String) -> OperationChain {
    if monkey_id == &"humn".to_string() {
        return Chain(Vec::new());
    }

    match monkeys.get(monkey_id).unwrap() {
        Value(v) => Operand(v.clone()),
        Op(a, op, b) => match (unwrap_equation(monkeys, a), unwrap_equation(monkeys, b)) {
            (Operand(va), Operand(vb)) => Operand(op.apply(va, vb)),
            (Chain(chain), Operand(v)) => Chain([vec![Left(*op, v)], chain].concat()),
            (Operand(v), Chain(chain)) => Chain([vec![Right(*op, v)], chain].concat()),
            _ => unreachable!("Only one human")
        }
    }
}

fn determine_value_to_shout(monkeys: &HashMap<String, Monkey>) -> isize {
    if let Op(left, _, right) = monkeys.get(&"root".to_string()).unwrap() {
        match (unwrap_equation(monkeys, left), unwrap_equation(monkeys, right)) {
            (Chain(chain), Operand(operand)) |
            (Operand(operand), Chain(chain)) => {
                return chain.into_iter().fold::<isize, _>(
                    operand,
                    |acc, operation| match operation {
                        Left(op, val) => op.inverse().apply(acc, val),
                        Right(op, val) => match op {
                            Add | Mul => op.inverse().apply(acc, val),
                            Sub => Sub.apply(val, acc),
                            Div => Div.apply(val, acc)
                        }
                    }
                );
            },
            _ => unreachable!("Exactly one human")
        }
    }

    unreachable!("Root is always an Op")
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use Monkey::*;
    use crate::day_21::{determine_value_to_shout, Monkey, parse_input, resolve, unwrap_equation};
    use crate::day_21::Operation::{Left, Right};
    use crate::day_21::OperationChain::{Chain, Operand};
    use crate::day_21::Operator::*;

    fn sample_monkeys() -> HashMap<String, Monkey> {
        vec![
            ("root".to_string(), Op("pppw".to_string(), Add, "sjmn".to_string())),
            ("dbpl".to_string(), Value(5)),
            ("cczh".to_string(), Op("sllz".to_string(), Add, "lgvd".to_string())),
            ("zczc".to_string(), Value(2)),
            ("ptdq".to_string(), Op("humn".to_string(), Sub, "dvpt".to_string())),
            ("dvpt".to_string(), Value(3)),
            ("lfqf".to_string(), Value(4)),
            ("humn".to_string(), Value(5)),
            ("ljgn".to_string(), Value(2)),
            ("sjmn".to_string(), Op("drzm".to_string(), Mul, "dbpl".to_string())),
            ("sllz".to_string(), Value(4)),
            ("pppw".to_string(), Op("cczh".to_string(), Div, "lfqf".to_string()), ),
            ("lgvd".to_string(), Op("ljgn".to_string(), Mul, "ptdq".to_string())),
            ("drzm".to_string(), Op("hmdt".to_string(), Sub, "zczc".to_string())),
            ("hmdt".to_string(), Value(32)),
        ].into_iter().collect()
    }

    #[test]
    fn can_parse() {
        let input = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
".to_string();

        assert_eq!(parse_input(&input), sample_monkeys())
    }

    #[test]
    fn can_resolve_monkeys() {
        assert_eq!(
            resolve(&mut sample_monkeys(), &"root".to_string()),
            152
        )
    }

    #[test]
    fn can_unwrap_tree() {
        assert_eq!(
            unwrap_equation(&sample_monkeys(), &"sjmn".to_string()),
            Operand(150)
        );

        assert_eq!(
            unwrap_equation(&sample_monkeys(), &"pppw".to_string()),
            Chain(vec![
                Left(Div, 4),
                Right(Add, 4),
                Right(Mul, 2),
                Left(Sub, 3),
            ])
        )
    }

    #[test]
    fn can_determine_value_to_shout() {
        assert_eq!(
            determine_value_to_shout(&sample_monkeys()),
            301
        )
    }
}

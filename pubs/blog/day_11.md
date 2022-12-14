---
day: 11
tags: post
header: 'Monkey in the Middle'
---

Still swimming upstream I notice some very predictable monkeys have taken some things from my rucksack. 

## Modelling Monkeys

Today's puzzle input is quite complex, so I've broken down the model of a `Monkey` into the different parts and 
implemented a method to parse each of them. An example specification of a monkey looks like.

```text
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
```

The monkeys are listed in order, so I can ignore the first line and use their position in the vector. The list of 
items can be a `Vec<isize>`, so no custom type needed. For the operation the two operands will be `old` or a number, 
and the operator will be `+` of `*`.

```rust
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Operand {
    Value(isize),
    Old,
}

impl From<&str> for Operand {
    fn from(spec: &str) -> Self {
        match spec {
            "old" => Operand::Old,
            i => Value(i.parse().unwrap())
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Operation {
    Mul(Operand, Operand),
    Add(Operand, Operand),
}

impl From<&str> for Operation {
    fn from(spec: &str) -> Self {
        let mut parts = spec.split_whitespace();
        let a = parts.next().unwrap().into();
        let op = parts.next().unwrap();
        let b = parts.next().unwrap().into();

        match op {
            "+" => Add(a, b),
            "*" => Mul(a, b),
            _ => unreachable!()
        }
    }
}
```

Finally, it is easiest to parse the divisor test as part of parsing the monkey, but I do still need to define its 
shape, and parsing the `If true:` and `If false:` lines can be abstracted.

```rust
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Test {
    divisor: isize,
    if_true: usize,
    if_false: usize,
}

fn parse_branch(spec: &str) -> usize {
    spec.split_whitespace()
        .dropping(5).next().unwrap()
        .parse::<usize>().unwrap()
}
```

I now can build the full `Monkey` representation. I note from the task I also need to keep track of how many times 
each monkey has handled an item. I'll initialise a counter to track that for each monkey.

```rust
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

        // Ignore Monkey: <id>
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
```

Finally, the puzzle input needs to be split into monkeys, delimited by blank lines, and collected into a `Vec`.

```rust
fn parse_input(input: &String) -> Vec<Monkey> {
    input.split("\n\n").map_into().collect()
}
```

The test example is quite long, but having the monkeys available for later tests will be useful.

```rust
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
```

## Part 1 - Simian Simulation

The monkeys have very predictable behaviour: they examine each item in a worrying way, then pass it to another 
monkey depending on how worried I look. Though there is some relief when the item survives (divide the new worry by 
three)

I need to be able to apply the monkey's operation to each item.

```rust
impl Operand {
    fn apply(&self, item: isize) -> isize {
        match self {
            &Operand::Old => item,
            &Value(val) => val,
        }
    }
}
// ...
impl Operation {
    fn apply(&self, item: isize) -> isize {
        match self {
            &Mul(a, b) => a.apply(item) * b.apply(item),
            &Add(a, b) => a.apply(item) + b.apply(item),
        }
    }
}
```

Similarly, I need to ba able to get a monkey index from the newly increased worry value.

```rust
impl Test {
    fn apply(&self, worry: isize) -> usize {
        if worry % self.divisor == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}
```

Now I need to loop over each monkey's items in turn. The awkwardness here is I can only have one mutable reference into 
the `Monkey`s list at a time, so I can't modify monkeys whilst iterating over the list, nor keep a mutable reference to
the current monkey whilst passing items the item lists of other monkeys. To work around this I need to:

1. Iterate over the monkey indices, rather than the list and get a mutable reference only while I need it within the 
   loop.
2. Store a copy of the item list (it's a short list of integers, so cheap to copy), and the other parts of the monkey 
   needed when iterating over the list.
3. Do the updates to the current monkey before iterating over the items, so that the mutable reference is done with 
   by the time I'm updating the other monkeys.

```rust
fn simulate_round(monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        let mut monkey = monkeys.get_mut(i).unwrap();
        let current_items = monkey.items.clone();
        let operation = monkey.operation;
        let test = monkey.test;

        monkey.items = Vec::new();
        monkey.handling_count = monkey.handling_count + current_items.len();

        for item in current_items {
            let worry = operation.apply(item) / 3;
            monkeys.get_mut(test.apply(worry)).unwrap().items.push(worry);
        }
    }
}
```

I can use the example given to check the items in the updated monkey array after one round match expectations.

```rust
#[test]
fn can_simulate_round() {
    let mut monkeys = sample_monkeys();
    simulate_round(&mut monkeys, 3, get_sample_common_denominator());
    let item_lists: Vec<Vec<isize>> = 
        monkeys.iter().map(|m| m.items.clone()).collect();
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
```

Finally, I need to loop through that process for twenty rounds, extract the number of times the two most active 
monkeys have handled an item, and multiply them together to give a metric for the level of monkey business going on.

```rust
fn get_monkey_business_level(mut monkeys: &mut Vec<Monkey>) -> usize {
    let common_denominator =
        monkeys
            .iter()
            .map(|m| m.test.divisor)
            .reduce(|acc, div| acc * div).unwrap();

    for _ in 0..20 {
        simulate_round(&mut monkeys)
    }

    monkeys
        .iter()
        .map(|m| m.handling_count)
        .sorted().rev().take(2)
        .reduce(|acc, monkey| acc * monkey).unwrap()
}
// ...
#[test]
fn can_find_monkey_business_level() {
    let mut monkeys = sample_monkeys();
    assert_eq!(
        get_monkey_business_level(&mut monkeys.clone()),
        10605,
    );
}
```

I can now get the monkey business value for the puzzle input:

```rust
pub fn run() {
    let contents =
        fs::read_to_string("res/day-11-input").expect("Failed to read file");
    let mut monkeys = parse_input(&contents);

    println!(
        "After twenty rounds the top two monkeys have a monkey business score of: {}",
        get_monkey_business_level(&mut monkeys.clone()),
    );
}
```

## Part 2 - Exponential Exasperation

Now I need to re-run the simulation for 10,000 rounds, also given how long it is taking to get my items back, I no 
longer feel the same sense of relief when they aren't broken. I suspect the problem here is the worry will outgrow 
what can be stored in an integer, but I plug in the number to be sure.

```text
attempt to multiply with overflow
thread 'day_11::tests::can_find_monkey_business_level' panicked at 
  'attempt to multiply with overflow', src\day_11.rs:58:27
stack backtrace:
 0: std::panicking::begin_panic_handler
    at /rustc/897...120/library\std\src\panicking.rs:584
 1: core::panicking::panic_fmt
    at /rustc/897...120/library\core\src\panicking.rs:142
 2: core::panicking::panic
    at /rustc/897...120/library\core\src\panicking.rs:48
 3: enum2$<advent_of_code_2022::day_11::Operation>::apply
    at .\src\day_11.rs:58
 4: advent_of_code_2022::day_11::simulate_round
    at .\src\day_11.rs:169
 5: advent_of_code_2022::day_11::get_monkey_business_level
    at .\src\day_11.rs:187
 6: advent_of_code_2022::day_11::tests::can_find_monkey_business_level
    at .\src\day_11.rs:297
```

I note that the key for how the items move around is what they're divisible by. So if I multiply the divisors 
together, this will give a common denominator that I can use to limit how large the worry value gets. The divisors 
are all unique primes, so there isn't a smaller divisor that will work.

I'll calculate the common denominator once and pass it into each. I will also need to make the number of 
rounds and the worry level decrease configurable.

```rust
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
// ...
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
```

The configuration parameters need to be passed into by the test and the run method, and both can be updated to 
include part two.

```rust
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
// ...
pub fn run() {
    let contents =
        fs::read_to_string("res/day-11-input").expect("Failed to read file");
    let mut monkeys = parse_input(&contents);

    println!(
        "After twenty rounds the top two monkeys have a monkey business score of: {}",
        get_monkey_business_level(&mut monkeys.clone(), 20, 3),
    );

    println!(
        "After 10,000 rounds without worry reduction, the top two monkeys have \
         a score of: {}",
        get_monkey_business_level(&mut monkeys, 10000, 1),
    )
}
// After twenty rounds the top two monkeys have a monkey business score of: 61503
// After 10,000 rounds without worry reduction, the top two monkeys have 
// a score of: 14081365540
// 
// Finished in 72.90ms
```

It takes by far the longest of any day so far (~75ms out of ~125ms total run time for all 11 puzzles). I can't think 
of a way to make it more efficient, so I'll take the hit.

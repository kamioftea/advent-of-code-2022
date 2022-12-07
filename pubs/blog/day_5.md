---
day: 5
tags: post
header: 'Day 5: Supply Stacks'
---
Today I need to predict how the crates of supplies will end up once their crane has completed its list of planned crate
movements, towers of Hanoi style. The puzzle input is divided into a diagram of the starting position and the planned
list of crate moves.

## Supply Stacks Initialisation

Given each move encodes multiple moves between two stacks, I've decided to implement the Supply Stacks as a custom
`struct`, with attached methods for applying a whole move. 

First I'll implement loading the initial state from the diagram in the input.

```text
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
```

Given I'll eventually want the top of each stack, I'll set things up so that the first element in the list is the top.
DequeVec has `push_front`/`pop_front` methods that will let me do this. Parsing the diagram from the bottom up means 
I can use the numbers line to get the number of stacks to create. If I split each of the remaining lines into chunks 
of four characters (`[#] `), rust won't complain that the last chunk is only three long, and it means the character 
I care about is always the 2nd in the chunk. I enumerate the chunks so that as the stacks top out, I can drop the gaps
and still know which stack the letter needs to go to.

```rust
use std::collections::VecDeque;

#[derive(Eq, PartialEq, Debug, Clone)]
struct SupplyStacks {
    stacks: Vec<VecDeque<char>>
}

impl From<&str> for SupplyStacks {
    fn from(input: &str) -> Self {
        let mut stacks: Vec<VecDeque<char>> = Vec::new();
        let mut lines = input.lines().rev();
        let numbers = lines.next().unwrap();

        for _ in numbers.split_whitespace() {
            stacks.push(VecDeque::new())
        };

        for line in lines {
            for (i, chunk) in line.chars().chunks(4).into_iter().enumerate() {
                // Chunk is either `[A] ` ore `    `, the last in each line 
                // will be missing the final space
                let character: char = chunk.dropping(1).next().unwrap();
                if character.is_alphabetic() {
                    stacks[i].push_front(character)
                }
            }
        }

        SupplyStacks { stacks }
    }
}
```

I can also do the rest of the parsing now as well. I split the input on the blank line between the two sections, and
map each move line to a tuple of the numbers.

```rust
type Move = (usize, usize, usize);

fn parse_input(input: &String) -> (SupplyStacks, Vec<Move>) {
    let (stack_spec, moves_spec) = input.split_once("\n\n").unwrap();

    (SupplyStacks::from(stack_spec), parse_moves(moves_spec))
}

fn parse_moves(input: &str) -> Vec<Move> {
    input.lines().map(parse_move).collect()
}

fn parse_move(line: &str) -> Move {
    let parts: Vec<usize> = 
        line.split_whitespace()
            .flat_map(|str| str.parse::<usize>())
            .collect();

    (parts[0], parts[1], parts[2])
}
```

The sample input can be a test case.

```rust
fn sample_moves() -> Vec<Move> {
    vec![
        (1, 2, 1),
        (3, 1, 3),
        (2, 2, 1),
        (1, 1, 2)
    ]
}

fn sample_stacks() -> SupplyStacks {
    SupplyStacks {
        stacks: vec![
            vec!['N', 'Z'].into_iter().collect(),
            vec!['D', 'C', 'M'].into_iter().collect(),
            vec!['P'].into_iter().collect(),
        ]
    }
}

#[test]
fn can_parse() {
    let sample_input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2".to_string();

    let (actual_stacks, actual_moves) = parse_input(&sample_input);

    assert_eq!(actual_stacks, sample_stacks());
    assert_eq!(actual_moves, sample_moves());
}
```

## Part 1 - Moving crates

The first task is to apply each move, with the crates moved one at a time. So if the crates are stacked A on B, then 
C on its own. If I `move 2 from 1 to 2` then `A` is moved onto `C` then `B` onto `A`:

```text
                                  [B]
[A]      --->      [A]  --->      [A]             
[B] [C]        [B] [C]            [C]
 1   2          1   2          1   2         
```

I will first implement applying a single move to a SupplyStack.

```rust
impl CargoStacks {
    fn do_move(&mut self, (count, from, to): Move) {
        for _ in 0..count {
            let cargo = self.stacks[from - 1].pop_front().unwrap();
            self.stacks[to - 1].push_front(cargo)
        }
    }
}
```

This then needs to be applied for all moves in the list, and add a test case from the puzzle example.

```rust
impl CargoStacks {
    // ...
    fn do_moves(&mut self, mvs: &Vec<Move>) {
        for &mv in mvs {
            self.do_move(mv)
        }
    }
}
// ...
fn sample_stacks_after_moves() -> SupplyStacks {
    SupplyStacks {
        stacks: vec![
            vec!['C'].into_iter().collect(),
            vec!['M'].into_iter().collect(),
            vec!['Z', 'N', 'D', 'P'].into_iter().collect(),
        ]
    }
}

#[test]
fn can_apply_moves() {
    let mut stacks = sample_stacks();

    stacks.do_moves(&sample_moves());
    assert_eq!(
        stacks,
        sample_stacks_after_moves()
    );
}
```

Once all the moves are complete, the puzzle solution is the string made from the characters at the top of each stack.

```rust
impl CargoStacks {
    // ...
    fn get_top_crates(&self) -> String {
        self.stacks.to_owned().into_iter().map(|stack| stack[0]).join("")
    }
}
// ...
#[test]
fn can_get_stack_tops() {
    assert_eq!(sample_stacks().get_top_crates(), "NDP");
    assert_eq!(sample_stacks_after_moves().get_top_crates(), "CMZ");
}
```

Everything is now in place, and all that is left is to apply the puzzle input.

```rust
pub fn run() {
    let contents = 
        fs::read_to_string("res/day-5-input").expect("Failed to read file");
    let (mut stacks, moves) = parse_input(&contents);

    stacks.do_moves(&moves);
    println!(
        "After moving one crate at a time, the top of the stacks are: {}",
        stacks.get_top_crates()
    );
}
// After moving one crate at a time, the top of the stacks are: CNSZFDVLJ
```

## Part 2 - Moving stacks

For part two, I notice the crane is a newer model that can move whole stacks at once, i.e. the example move from 
part one would instead move `A` and `B` together onto `C:

```text
                   [A]
[A]      --->      [B]             
[B] [C]            [C]
 1   2          1   2         
```

The easiest way to adapt the existing code was to pass the mode in as a boolean flag. To represent the whole move I'll 
need to add the crates to the target stack in reverse order, which means removing them all before adding. The first 
part can also be implemented in this way, so the only thing the mode flag needs to change is if the temporary list 
of crates being moved needs reversing.

```rust
impl CargoStacks {
    fn do_move(&mut self, (count, from, to): Move, all_at_once: bool) {
        let mut temp = Vec::new();
        for _ in 0..count {
            temp.push(self.stacks[from - 1].pop_front().unwrap());
        }

        if all_at_once {
            temp.reverse()
        }

        for c in temp {
            self.stacks[to - 1].push_front(c)
        }
    }
    // ...
}
```

The flag needs to be added to `do_moves`, tests and implementations. To avoid needing to parse the input twice I 
plan to clone the initial state of the tasks, so I will also do this in the test to verify it works as expected.

```rust
impl CargoStacks {
    // ...
    fn do_moves(&mut self, mvs: &Vec<Move>, all_at_once: bool) {
        for &mv in mvs {
            self.do_move(mv, all_at_once)
        }
    }
}
// ...
fn sample_stacks_after_moving_in_bulk() -> SupplyStacks {
    SupplyStacks {
        stacks: vec![
            vec!['M'].into_iter().collect(),
            vec!['C'].into_iter().collect(),
            vec!['D', 'N', 'Z', 'P'].into_iter().collect(),
        ]
    }
}

#[test]
fn can_apply_moves() {
    let mut stacks_singly = sample_stacks();
    let mut stacks_bulk = stacks_singly.clone();

    stacks_singly.do_moves(&sample_moves(), false);
    assert_eq!(
        stacks_singly,
        sample_stacks_after_moving_one_at_a_time()
    );

    stacks_bulk.do_moves(&sample_moves(), true);
    assert_eq!(
        stacks_bulk,
        sample_stacks_after_moving_in_bulk()
    );
}
// ...
pub fn run() {
    let contents = 
        fs::read_to_string("res/day-5-input").expect("Failed to read file");
    let (mut part_1_stacks, moves) = parse_input(&contents);
    let mut part_2_stacks = part_1_stacks.clone();

    part_1_stacks.do_moves(&moves, false);
    println!(
        "After moving one crate at a time, the top of the stacks are: {}",
        part_1_stacks.get_top_crates()
    );

    part_2_stacks.do_moves(&moves, true);
    println!(
        "After moving the crates in bulk, the top of the stacks are: {}",
        part_2_stacks.get_top_crates()
    );
}
// After moving one crate at a time, the top of the stacks are: CNSZFDVLJ
// After moving the crates in bulk, the top of the stacks are: QNDWLMGNS
// 
// Finished in 1.58ms
```

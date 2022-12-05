---
day: 3
tags: post
header: 'Day 3: Rucksack Reorganization'
---
First of all a shout-out to GitHub. I was at wargaming tournament today, but was able to get most of the parsing 
work done during the lunch break on my phone using [GitHub.dev](https://github.dev/github/dev). If you replace the 
`.com` in a repositories GitHub URL with `.dev` it starts up an instance of VSCode you can access from a web browser,
with that repository checked out. Using it on a phone was awkward, definitely not what it was designed for, but it 
did work. If I'd thought to pack my tablet I might have even been able to complete part one.

Today's task is to help the elves sort out some backpack packing mishaps, that mostly involves finding the intersection
of two sets of characters. The input is a list of rucksacks' contents. Each type of item has a single letter ID, and 
the contents are shown as a string of those IDs.

## Part 1: Mismatched items

Each rucksack has two compartments, each having the same number of items, and all from the left compartment are listed 
before the right. So for the sample input, it splits like so:

```text
                                  | Left compartment | Right compartment |
                                  | ---------------- | ----------------- |
vJrwpWtwJgWrhcsFMMfFFhFp          | vJrwpWtwJgWr     | hcsFMMfFFhFp      |
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL  | jqHRNqRjqzjGDLGL | rsFMfFZSrLrFZsSL  |
PmmdzqPrVvPwwTWBwg                | PmmdzqPrV        | vPwwTWBwg         |
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn    | wMqvLMZHhHMvwLH  | jbvcjnnSBnvTQFn   |
ttgJtRGJQctTZtZT                  | ttgJtRGJ         | QctTZtZT          |
CrZsJsPPZsGzwwsLwLmpwMDw          | CrZsJsPPZsGz     | wwsLwLmpwMDw      |
```

Each item type should only be packed into one compartment, but packing mishap number one is that for each bag, 
one item type has been split between the compartments. This is if I take the intersection of the sets of item 
type IDs (characters) in the two compartments, I will get a singleton ID for each bag. For the example these are in 
order: `p`, `L`, `P`, `v`, `t`, and `s`.

Given I need to find an intersection of two sets, it makes sense to me to store the strings we need to intersect as 
`Set`s. 

```rust
use std::collections::BTreeSet;

type Compartment = BTreeSet<char>;
type Rucksack = (Compartment, Compartment);
```

Turning the input into structured data is then three layers: mapping the list of lines to list of backpacks, which 
requires splitting each line into two equal compartments, which requires turning a substring into the set of characters
it contains.

```rust
fn parse_input(input: &String) -> Vec<Rucksack> {
    input.lines()
         .map(parse_rucksack)
         .collect()
}

fn parse_rucksack(line: &str) -> Rucksack {
    let (a, b) = line.split_at(line.len() / 2);
    (parse_compartment(a), parse_compartment(b))
}

fn parse_compartment(line_half: &str) -> Compartment {
    line_half.chars().collect()
}
// ...
#[test]
fn can_parse_rucksack() {
    assert_eq!(
        parse_rucksack("ttgJtRGJQctTZtZT"),
        (
            vec!['t', 'g', 'J', 'R', 'G'].into_iter().collect::<Compartment>(),
            vec!['Q', 'c', 't', 'T', 'Z'].into_iter().collect::<Compartment>()
        )
    );
}
```

The sets created, the intersection will provide the set of characters in both compartments, which I can assume will 
only ever be a single character because of the puzzle constraints.

```rust
fn find_item_to_rearrange((a, b): &Rucksack) -> &char {
    a.intersection(&b).into_iter().next().unwrap()
}
// ...
#[test]
fn can_find_item_to_rearrange() {
    assert_eq!(
        parse_input(&get_sample_data())
            .iter()
            .map(find_item_to_rearrange)
            .collect::<Vec<&char>>(),
        get_sample_items().iter().collect::<Vec<&char>>()
    )
}
```

Each item type ID has a priority, based on its position in the alphabet:

* Lowercase `a` - `z`: &nbsp;1 - 26
* Uppercase `A` - `Z`: 27 - 52

There's a neat trick here I picked up from [a talk on Unicode by Dylan Beattie](https://youtu.be/gd5uJ7Nlvvo), which 
is that the ASCII, and therefore Unicode for the roman alphabet's letters are 5 binary digits representing 1 - 26, 
prefixed with `10` for uppercase, and `11` for lowercase. So to map the letter to its position in the alphabet, I can 
just bitwise & the character's integer representaion with `0b11111`. The puzzle examples can be used to make some tests.

```rust
fn map_char_to_priority(c: &char) -> u32 {
    match c {
        'a'..='z' => 0b11111 & *c as u32,
        'A'..='Z' => (0b11111 & *c as u32) + 26,
        _ => unreachable!()
    }
}

fn sum_mismatched_items(bags: &Vec<Rucksack>) -> u32 {
    bags.iter()
        .map(find_item_to_rearrange)
        .map(map_char_to_priority)
        .sum()
}
// ...
fn get_sample_data() -> String {
    return "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw".to_string();
}

fn get_sample_items() -> Vec<char> {
    vec!['p', 'L', 'P', 'v', 't', 's']
}

#[test]
fn can_map_to_priorities() {
    assert_eq!(
        get_sample_items()
            .iter()
            .map(map_char_to_priority)
            .collect::<Vec<u32>>(),
        vec![16, 38, 42, 22, 20, 19]
    );

    assert_eq!(
        vec!['a', 'z', 'A', 'Z']
            .iter()
            .map(map_char_to_priority)
            .collect::<Vec<u32>>(),
        vec![1, 26, 27, 52]
    )
}

#[test]
fn can_sum_mismatched_items_priorities() {
    assert_eq!(
        sum_mismatched_items(&parse_input(&get_sample_data())),
        157
    )
}
```

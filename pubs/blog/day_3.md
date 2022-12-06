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

## Part 1: Intersecting compartments

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
    let position = 0b11111 & *c as u32;

    if c.is_uppercase() { position + 26 } else { position }
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

These parts together can now give me the part one solution:

```rust
pub fn run() {
    let contents = 
        fs::read_to_string("res/day-3-input").expect("Failed to read file");
    let bags = parse_input(&contents);

    println!(
        "The sum of the mismatched items' priorities is: {}",
        sum_mismatched_items(&bags)
    );
}
// The sum of the mismatched items' priorities is: 7727
```

## Part 2: Intersecting backpacks

It also turns out that each group of three elves share an item type they use as an ID badge for their group. These 
were packed without this year's stickers, so need to be identified so this can be fixed. 

This is again intersecting sets of characters. With the types we currently have the steps are:

1. Group the elves into sets of three.
2. Union the two compartments of each bag in the group to get the full set of items in the bag. 
3. Get the intersection of all three bags, which by the puzzle constraints is again guaranteed to be a single character.
4. Sum the priorities using the same algorithm as before.

I ran into a bunch of borrow checker issues here, Rust's sets don't implement copy which makes them hard to wrap in 
other iterators, and I'm very much out of practice with the idiomatic ways to do this. I was able to implement 
the steps
above, but it's pretty messy code, so I'm looking to refactor this once I have a working solution.

```rust
fn sum_group_badge_priorities(bags: &Vec<Rucksack>) -> u32 {
    let mut sum = 0;
    let mut iter = bags.into_iter();
    while let Some(base) = iter.next() {
        let mut items: BTreeSet<char> = get_rucksack_items(&base);
        
        items = items.intersection(&get_rucksack_items(iter.next().unwrap()))
                     .into_iter().map(|&c| c)
                     .collect();
                     
        items = items.intersection(&get_rucksack_items(iter.next().unwrap()))
                     .into_iter()
                     .map(|&c| c).collect();

        let c = items.iter().next().unwrap();
        sum = sum + map_char_to_priority(c)
    }
    sum
}

fn get_rucksack_items((a, b): &Rucksack) -> BTreeSet<char> {
    a.union(b).into_iter().map(|&c| c).collect()
}
// ...
#[test]
fn can_sum_badge_priorities() {
    assert_eq!(
        sum_group_badge_priorities(&parse_input(&get_sample_data())),
        70
    )
}
// ...
pub fn run() {
    let contents = 
        fs::read_to_string("res/day-3-input").expect("Failed to read file");
    let bags = parse_input(&contents);

    println!(
        "The sum of the mismatched items' priorities is: {}",
        sum_mismatched_items(&bags)
    );

    println!(
        "The sum of the group badge items' priorities is: {}",
        sum_group_badge_priorities(&bags)
    )
}
// The sum of the mismatched items' priorities is: 7727
// The sum of the group badge items' priorities is: 2609
```

## Changing the model

Having seen both parts of the puzzle, it's clear that the model chosen for part 1 is not the best fit for part 2. 
Further using Sets as the main representation of the rucksack that is passed around is awkward, and the benefits of 
being able to re-use sets that were costly to build isn't really realised as each set is only really used once. It 
might be better to pass around the strings, and just make the sets we need on the fly. The core operation then becomes
intersecting two strings to a string with all the characters in both.

```rust
fn intersect_strings(a: &String, b: &String) -> String {
    let set: BTreeSet<char> = BTreeSet::from_iter(a.chars());

    b.chars().filter(|c| set.contains(c)).collect()
}

// ...
fn can_intersect_strings() {
    assert_eq!(
        intersect_strings(&"abcd".to_string(), &"defg".to_string()), 
        "d".to_string()
    );
    assert_eq!(
        intersect_strings(&"abcd".to_string(), &"cdef".to_string()), 
        "cd".to_string()
    );
    assert_eq!(
        intersect_strings(&"cafH".to_string(), &"wHcl".to_string()), 
        "Hc".to_string()
    );
    assert_eq!(
        intersect_strings(&"cafH".to_string(), &"wHclH".to_string()),
        "HcH".to_string()
    );
}
```

This is taking some shortcuts allowed by the puzzle constraints:

1. The strings are always going to be emitted in the order of the second string.
2. Repeated characters in the second string will show up multiple times if they're part of the intersection.

In both cases this is mitigated by the puzzle always reducing the sets down to a single character (possibly repeated) 
and I can just take the first one.

The replacements for parts 1 and 2 can now be built through chained iterators.

```rust
fn sum_mismatched_items(rucksacks: &Vec<String>) -> u32 {
    rucksacks.iter()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(a, b)| intersect_strings(&a.to_string(), &b.to_string()))
        .map(|str| map_char_to_priority(&str.chars().next().unwrap()))
        .sum()
}

fn sum_group_badge_priorities(rucksacks: &Vec<String>) -> u32 {
    rucksacks.chunks(3)
        .map(|chunk| {
            let intermediate = intersect_strings(&chunk[0], &chunk[1]);
            intersect_strings(&intermediate, &chunk[2])
        })
        .map(|str| map_char_to_priority(&str.chars().next().unwrap()))
        .sum()
}
```

They pass the previous tests, and produce the same puzzle outputs, but it was much easier to keep the compilter happier,
and I think it's clearer what is going on. I've cleaned up the moddule by removing the methods, types, and tests no 
longer needed.


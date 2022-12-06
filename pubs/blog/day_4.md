---
day: 4
tags: post
header: 'Day 4: Camp Cleanup'
---
Elves have been paired up to clean up the base camp before we head out on the star fruit expedition. Quite a few 
have been assigned redundant tasks with one of the pair fully or partially overlapping with the other. For example the
sample input:

```text
2-4,6-8           | No overlap
2-3,4-5           | No overlap
5-7,7-9           | 7 overlaps
2-8,3-7           | 3-7 is entirely within 2-8
6-6,4-6           | 6 is entirely within 4-6
2-6,4-8           | 4-6 overlap
```

## Parsing the input

First some type aliases to help communicate intent:

```rust
type Range = (u32, u32);
type Pair = (Range, Range);
```

I've yet to find a performant regex library for Rust, so I'll use the `,` to split the pairs and `-` to split the 
range bounds.

```rust
fn parse_input(input: &String) -> Vec<Pair> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Pair {
    let parts: Vec<&str> = line.split(',').collect();
    (
        parse_range(parts[0]),
        parse_range(parts[1])
    )
}

fn parse_range(spec: &str) -> Range {
    let limits: Vec<u32> =
        spec.split('-')
            .map(|str| str.parse::<u32>().unwrap())
            .collect();

    (limits[0], limits[1])
}
// ...
fn sample_pairs() -> Vec<Pair> {
    vec![
        ((2, 4), (6, 8)),
        ((2, 3), (4, 5)),
        ((5, 7), (7, 9)),
        ((2, 8), (3, 7)),
        ((6, 6), (4, 6)),
        ((2, 6), (4, 8)),
    ]
}

#[test]
fn can_parse() {
    let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8".to_string();

    assert_eq!(parse_input(&input), sample_pairs());
}
```

# Part 1 - Wholly redundant

First task, I have to count the pairs where one elf is wholly redundant, e.g. lines 4 and 5 in the sample. The 
hard work was all done when parsing the input, I now just need to filter to only the pairs where one elf starts on or
before the other, and finishes on or after the other.  here, and as those
are passing I can 

```rust
fn count_redundant_pairs(pairs: &Vec<Pair>) -> usize {
    pairs.iter().filter(|&&pair| pair_has_redundant_elf(pair)).count()
}

fn pair_has_redundant_elf(
    ((elf1_start, elf1_end), (elf2_start, elf2_end)): Pair
) -> bool {
    (elf1_start <= elf2_start && elf1_end >= elf2_end) ||
        (elf1_start >= elf2_start && elf1_end <= elf2_end)
}
```

The sample data can also be used as a test cases:

```rust
#[test]
fn can_find_redundant_pairs() {
    for (pair, expected) 
    in sample_pairs().into_iter().zip(vec![false, false, false, true, true, false]) 
    {
        assert_eq!(
            pair_has_redundant_elf(pair), 
            expected, 
            "Check pair {:?} redundancy", pair
        );
    }
    
    let boundaries = vec![
        ((4, 4), (4, 6)),
        ((6, 6), (4, 6)),
        ((4, 6), (4, 4)),
        ((4, 6), (6, 6)),
    ];
    
    for pair in boundaries {
        assert_eq!(
            pair_has_redundant_elf(pair), 
            true, 
            "Check pair {:?} redundancy", pair
        );
    }
}

#[test]
fn can_count_pairs() {
    assert_eq!(count_redundant_pairs(&sample_pairs()), 2);
}
```

With those passing I can apply `count_redundant_pairs` to the puzzle input;

```rust
pub fn run() {
    let contents = 
        fs::read_to_string("res/day-4-input").expect("Failed to read file");
    let pairs = parse_input(&contents);

    println!(
        "There are {} redundant pairs of elves",
        count_redundant_pairs(&pairs)
    );
}
```

## Part 2 - Some overlap

Part two is very similar to part 1, but has a more leinient filter. I need to count the pairs that have any overlap.

First a quick refactor to extract the filter to a parameter of the count method

```rust
fn count_pairs_matching(pairs: &Vec<Pair>, predicate: fn(Pair) -> bool) -> usize {
    pairs.iter().filter(|&&pair| predicate(pair)).count()
}
// ...
#[test]
fn can_count_pairs() {
    assert_eq!(count_pairs_matching(&sample_pairs(), pair_has_redundant_elf), 2);
}
// ...
pub fn run() {
    let contents = 
        fs::read_to_string("res/day-4-input").expect("Failed to read file");
    let pairs = parse_input(&contents);

    println!(
        "There are {} redundant pairs of elves",
        count_pairs_matching(&pairs, pair_has_redundant_elf)
    );
}
// There are 513 redundant pairs of elves 
```

Now I can add and test the more lenient version of the predicate. The sample data doesn't cover the full spectrum of 
edge cases, so I wrote some extra.

```rust
fn pair_overlaps(((elf1_start, elf1_end), (elf2_start, elf2_end)): Pair) -> bool {
    elf1_start <= elf2_end && elf1_end >= elf2_start
}
// ...
#[test]
fn can_find_overlaps_pairs() {
    let possibilities = vec![
        (((4, 6), (5, 5)), true),
        (((5, 5), (4, 6)), true),
        (((4, 5), (5, 6)), true),
        (((5, 6), (4, 5)), true),
        (((4, 5), (6, 7)), false),
        (((6, 7), (4, 5)), false),
    ];
    for (pair, expected) in possibilities {
        assert_eq!(pair_overlaps(pair), expected, "Check pair {:?} overlap", pair);
    }
}
```

This passing, I can plug the second predicate into the count function test and the run method.

```rust
#[test]
fn can_count_pairs() {
    assert_eq!(count_pairs_matching(&sample_pairs(), pair_has_redundant_elf), 2);
    assert_eq!(count_pairs_matching(&sample_pairs(), pair_overlaps), 4);
}
// ...
pub fn run() {
    let contents = 
        fs::read_to_string("res/day-4-input").expect("Failed to read file");
    let pairs = parse_input(&contents);

    println!(
        "There are {} redundant pairs of elves",
        count_pairs_matching(&pairs, pair_has_redundant_elf)
    );

    println!(
        "There are {} overlapping pairs of elves",
        count_pairs_matching(&pairs, pair_overlaps)
    );
}
// There are 513 redundant pairs of elves  
// There are 878 overlapping pairs of elves
// 
// Finished in 2.76ms
```

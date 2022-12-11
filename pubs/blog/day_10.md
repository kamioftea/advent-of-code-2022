---
day: 10
tags: post
header: 'Day 9: Cathode-Ray Tube'
---
Today I'm modelling simplified rope movement. There is a list of moves one end of the rope will follow, and I'm 
tasked with tracking the other and as it's dragged along. This reduces to multiple step of mapping iterators to other 
iterators.

## Parse the list of moves

The moves the head of the rope follows are specified with a list of direction and distance pairs.

```text
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
```

I'll first define some types.

```rust
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

type Motion = (Direction, usize);
type Position = (isize, isize);
```

Parsing the lines can be done with pattern matching. Split the pair using the white space, map the letter to the 
enum value and turn the number string into an integer. I opted to follow the wording in the puzzle and call the pairs
motions rather than the more natural `Move`, mostly because `move` is a reserved word in rust which makes naming 
variables awkward.

```rust
fn parse_input(input: &String) -> Vec<Motion> {
    input.lines().map(parse_motion).collect()
}

fn parse_motion(line: &str) -> Motion {
    let (letter, number) = line.split_once(" ").unwrap();

    let direction = match letter {
        "U" => UP,
        "D" => DOWN,
        "L" => LEFT,
        "R" => RIGHT,
        _ => unreachable!()
    };

    let distance = number.parse().unwrap();

    (direction, distance)
}
```

Testing this requires me to write out the example data as the internal representation.

```rust
fn sample_motions() -> Vec<Motion> {
    vec![
        (RIGHT, 4),
        (UP, 4),
        (LEFT, 3),
        (DOWN, 1),
        (RIGHT, 4),
        (DOWN, 1),
        (LEFT, 5),
        (RIGHT, 2),
    ]
}

#[test]
fn can_parse() {
    let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2".to_string();

    assert_eq!(parse_input(&input), sample_motions());
}
```

## Part 1 - Short rope

First I need to turn the moves into a list of positions the head of the rope follows.

```rust
fn apply_motion((x, y): Position, (direction, distance): Motion) -> Vec<Position> {
    let mut positions = Vec::new();
    for d in 1..=distance {
        let d_i = isize::try_from(d).unwrap();
        positions.push(
            match direction {
                UP => (x, y - d_i),
                DOWN => (x, y + d_i),
                LEFT => (x - d_i, y),
                RIGHT => (x + d_i, y),
            }
        )
    }

    positions
}
// ...
#[test]
fn can_apply_motions() {
    assert_eq!(
        apply_motions((0, 0), &vec![(RIGHT, 4), (UP, 2)]),
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (4, -1), (4, -2)]
    );
}
```

The tail following can be broken down into two parts. First updating the position of the tail based on the new 
position of the head of the rope. If the head is still touching the tail, no update is needed otherwise move the tail 
x and y towards the head x and y. The head has only moved one step, and so the tail will also only need to move at 
most one step also.

```rust
fn update_tail((head_x, head_y): Position, (tail_x, tail_y): Position) -> Position {
    if (head_x - tail_x).abs() <= 1 && (head_y - tail_y).abs() <= 1 {
        (tail_x, tail_y)
    } else {
        (
            if tail_x < head_x { tail_x + 1 } 
            else if tail_x > head_x { tail_x - 1 } 
            else { tail_x },
            
            if tail_y < head_y { tail_y + 1 }
            else if tail_y > head_y { tail_y - 1 } 
            else { tail_y },
        )
    }
}
// ...
#[test]
fn can_update_tail_for_head() {
    assert_eq!(update_tail((0, 0), (0, 0)), (0, 0));
    assert_eq!(update_tail((0, 1), (0, 0)), (0, 0));
    assert_eq!(update_tail((1, 1), (0, 0)), (0, 0));
    assert_eq!(update_tail((2, 2), (0, 0)), (1, 1));
    assert_eq!(update_tail((-2, 0), (0, 0)), (-1, 0));
    assert_eq!(update_tail((-2, 1), (0, 0)), (-1, 1));
}
```

To then generate a list of tail positions, map the iterator of head positions with the previous tail position 
using `update_tail`.

```rust
fn follow_head(origin: Position, head_positions: Vec<Position>) -> Vec<Position> {
    let mut tail_positions = Vec::new();
    tail_positions.push(origin);

    for head_position in head_positions {
        tail_positions.push(
            update_tail(
                head_position,
                *tail_positions.last().unwrap(),
            )
        );
    }

    tail_positions
}
// ...
#[test]
fn can_follow_head() {
    assert_eq!(
        follow_head(
            (0, 0),
            vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (4, -1), (4, -2)],
        ),
        vec![(0, 0), (0, 0), (0, 0), (1, 0), (2, 0), (3, 0), (3, 0), (4, -1)]
    )
}
```

The details of the implementation mean that the tail stays at the origin for multiple redundant steps, but as the 
eventual goal is to count unique positions this isn't an issue. The only remaining step is to reduce the 
path of the tail positions to a list of unique positions, and count it.

```rust
fn count_tail_locations(head_motions: Vec<Motion>) -> usize {
    let head_positions = apply_motions((0, 0), head_motions);
    follow_head((0, 0), head_positions).iter().unique().count()
}
// ...
#[test]
fn can_count_tail_locations() {
    assert_eq!(count_tail_positions(&sample_motions()), 13);
}
```

This done, I can apply it to the puzzle input.

```rust
pub fn run() {
    let contents =
        fs::read_to_string("res/day-9-input").expect("Failed to read file");

    let motions = parse_input(&contents);

    println!(
        "The tail of the rope with one knot passes through {} unique positions",
        count_tail_positions(&motions)
    );
}
// The tail of the rope with one knot passes through 5874 unique positions
```

## Part two - A longer rope

The task for part two is to do the same, but for a rope of length nine. Most of what I've done so far still works. 
The existing map of a tail following a list of positions can be re-applied to the list of positions from the first 
tail to get the list of positions the second part of the rope follows, then the third, and so on until the path of 
the ninth part is generated, and it can be reduced to unique positions and counted as before. 

First I'll refactor `count_tail_locations` to take a rope length, repeatedly applying the previous sections list of 
positions to the next.

```rust
fn count_tail_positions(head_motions: &Vec<Motion>, rope_length: usize) -> usize {
    (0..rope_length)
        .fold(
            apply_motions((0, 0), head_motions),
            |previous_knot, _| follow_head((0, 0), previous_knot),
        )
        .iter().unique().count()
}
```

The test needs updating, and the original sample input is not enough to move the end of a nine-length rope, so a 
bigger example is provided.

```rust
#[test]
fn can_count_tail_locations() {
    assert_eq!(count_tail_positions(&sample_motions(), 1), 13);
    assert_eq!(count_tail_positions(&sample_motions(), 9), 1);

    let larger_input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20".to_string();
    let larger_example = parse_input(&larger_input);
    assert_eq!(count_tail_positions(&larger_example, 9), 36);

}
```

I can add length nine rope to the run function and complete part two.

```rust
pub fn run() {
    let contents =
        fs::read_to_string("res/day-9-input").expect("Failed to read file");

    let motions = parse_input(&contents);

    println!(
        "The tail of the rope with one knot passes through {} unique positions",
        count_tail_positions(&motions, 1)
    );

    println!(
        "The tail of the rope with 9 knots passes through {} unique positions",
        count_tail_positions(&motions, 9)
    );
}
// The tail of the rope with one knot passes through 5874 unique positions
// The tail of the rope with 9 knots passes through 2467 unique positions
//
// Finished in 5.86ms
```

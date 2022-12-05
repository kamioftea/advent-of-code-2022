---
day: 2
tags: post
header: 'Day 2: Rock Paper Scissors'
---
Today I'm competing in a rock, paper, scissors with a _lot_ of elves, but I do have an "encrypted" strategy guide that
knows what the other elves will throw, and has calculated the best strategy without looking suspicious. The guide is in
the format 

```text
A X
B Y
C Z
...
```

Where the first column is what my opponent is expected to throw (`A` = Rock, `B` = Paper, `C` = Scissors.) The 
second column is the advice the book is giving. The tournament also has a scoring system based on:

* Symbol - Rock = 1, paper = 2, scissors = 3
* Outcome - Win = 6, draw = 3, loss = 0

## Part 1 - Calculate outcome from throws

For part 1 this the strategy advice is assumed to be (`X` = Rock, `Y` = Paper, `Z` = Scissors.)

First I'll define some types:

```rust
enum Move {
    Rock,
    Paper,
    Scissors,
}

type Round = (Move, Move);
type Tournament = Vec<Round>;
```

The parsing then needs to turn each line of the input into a round with two moves.

```rust
fn parse_strategy(strategy: &String) -> Tournament {
    strategy.lines()
            .map(parse_line)
            .collect()
}

fn parse_line(line: &str) -> Round {
    let (part_1, part_2) = line.split_at(1);
    (
        parse_move(part_1).unwrap(),
        parse_move(part_2).unwrap()
    )
}

fn parse_move(chr: &str) -> Option<Move> {
    match chr {
        "A" | " X" => Some(Rock),
        "B" | " Y" => Some(Paper),
        "C" | " Z" => Some(Scissors),
        _ => None
    }
}
```

There are some examples in the puzzle to use as test cases:

```rust
#[cfg(test)]
mod tests {
    use crate::day_2::{parse_strategy, Tournament};
    use crate::day_2::Move::{Paper, Rock, Scissors};

    fn sample_tournament() -> Tournament {
        vec![
            (Rock, Paper),
            (Paper, Rock),
            (Scissors, Scissors),
        ]
    }

    #[test]
    fn can_parse() {
        let example_guide = "A Y
B X
C Z".to_string();

        assert_eq!(
            parse_strategy(&example_guide),
            sample_tournament()
        );
    }
}
```

The puzzle solution is what I would score if I followed the strategy, so I need to implement the scoring system too.

```rust
fn score_tournament(tournament: &Tournament) -> u32 {
    tournament.into_iter().map(score_round).sum()
}

fn score_round(round: &Round) -> u32 {
    score_result(round) + score_move(round)
}

fn score_result(round: &Round) -> u32 {
    match round {
        (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => 6,
        (their_move, my_move) if their_move == my_move => 3,
        (_, _) => 0
    }
}

fn score_move((_, my_move): &Round) -> u32 {
    match my_move {
        Rock => 1,
        Paper => 2,
        Scissors => 3
    }
}
```

And add some tests from the puzzle outcomes

```rust
#[test]
fn can_score_round() {
    assert_eq!(score_round(&(Rock, Paper)), 8);
    assert_eq!(score_round(&(Paper, Rock)), 1);
    assert_eq!(score_round(&(Scissors, Scissors)), 6);
}

#[test]
fn can_score_tournament() {
    assert_eq!(
        score_tournament(&sample_moves_tournament()),
        15
    );
}
```

With all that in place I can now run this with the puzzle data

```rust
pub fn run() {
    let contents = 
        fs::read_to_string("res/day-2-input").expect("Failed to read file");
    let tournament = parse_strategy(&contents);

    println!(
        "Following the guide, my score would be: {}",
        score_tournament(&tournament)
    );
}

/// Following the guide, my score would be: 13809
```

## Part 2 - Calculate move from outcome

I now find out I assumed wrong, and the second column should have been the desired outcome (`X` = Loss, `Y` = Draw, 
`Z` = Win.)

Rather than duplicate work, I can pass the strategy for building a round from an input line into the parser. So I'll 
first refactor to rename `parse_line` to `parse_move_line`, and the existing `parse_strategy` to accept the line parser
as an argument.

```rust
fn parse_strategy(strategy: &String, syntax: fn(&str) -> Round) -> Tournament {
    strategy.lines()
            .map(syntax)
            .collect()
}
// ...
#[test]
fn can_parse() {
    let example_guide = "A Y
B X
C Z".to_string();

    assert_eq!(
        parse_strategy(&example_guide, parse_moves_line),
        sample_moves_tournament() // Also rename this function
    );
}
// ...
pub fn run() {
    let contents = 
        fs::read_to_string("res/day-2-input").expect("Failed to read file");
    let part_1_tournament = parse_strategy(&contents, parse_moves_line);

    println!(
        "Following the guide assuming moves, my score would be: {}",
        score_tournament(&part_1_tournament)
    );
}
```

I'll add a type for the outcome and a parsing matcher in the style of `parse_move`

```rust
enum Outcome {
    Win,
    Loss,
    Draw,
}

fn parse_outcome(chr: &str) -> Option<Outcome> {
    match chr {
        " X" => Some(Loss),
        " Y" => Some(Draw),
        " Z" => Some(Win),
        _ => None
    }
}
```

To work with the existing scoring system, parsing a line by outcome should still return a round in the form `(Move, 
Move)`. For this I'll need to be able to map an opponents move and desired outcome to a `Move`.

```rust
fn resolve_outcome(their_move: Move, outcome: Outcome) -> Round {
    let my_move = match outcome {
        Loss => loss_for(their_move),
        Draw => draw_for(their_move),
        Win => win_for(their_move)
    };

    (their_move, my_move)
}

fn win_for(mv: Move) -> Move {
    match mv {
        Rock => Paper,
        Paper => Scissors,
        Scissors => Rock,
    }
}

fn draw_for(mv: Move) -> Move {
    mv
}

fn loss_for(mv: Move) -> Move {
    match mv {
        Rock => Scissors,
        Paper => Rock,
        Scissors => Paper,
    }
}
```

The parsing strategy for part two splits the line and delegates to the functions added above. Once that is written the
tests and puzzle runner can be updated.

```rust
fn parse_outcome_line(line: &str) -> Round {
    let (part_1, part_2) = line.split_at(1);
    resolve_outcome(
        parse_move(part_1).unwrap(),
        parse_outcome(part_2).unwrap(),
    )
}
// ...
fn sample_outcome_tournament() -> Tournament {
    vec![
        (Rock, Rock),
        (Paper, Rock),
        (Scissors, Rock),
    ]
}

#[test]
fn can_parse() {
    let example_guide = "A Y
B X
C Z".to_string();

    assert_eq!(
        parse_strategy(&example_guide, parse_moves_line),
        sample_moves_tournament()
    );

    assert_eq!(
        parse_strategy(&example_guide, parse_outcome_line),
        sample_outcome_tournament()
    )
}
// ...
pub fn run() {
    let contents = 
        fs::read_to_string("res/day-2-input").expect("Failed to read file");
    let part_1_tournament = parse_strategy(&contents, parse_moves_line);

    println!(
        "Following the guide assuming moves, my score would be: {}",
        score_tournament(&part_1_tournament)
    );

    let part_2_tournament = parse_strategy(&contents, parse_outcome_line);

    println!(
        "Following the guide assuming outcomes, my score would be: {}",
        score_tournament(&part_2_tournament)
    );
}
// Following the guide assuming moves, your score would be: 13809
// Following the guide assuming outcomes, your score would be: 12316
```

## Final refactor

The code is already performing well, but I did notice that the `win_for`, `draw_for`, and `loss_for` functions could 
be reused to make the match `score_result` a bit clearer in its intent.

```rust
fn score_result(round: &Round) -> u32 {
    match round {
        &(their_move, my_move) if win_for(their_move) == my_move => 6,
        &(their_move, my_move) if draw_for(their_move) == my_move => 3,
        (_, _) => 0
    }
}
```

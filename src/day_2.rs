//! This is my solution for [Advent of Code - Day 2 - _Title_](https://adventofcode.com/2021/day/2)
//!
//!

use std::fs;
use crate::day_2::Move::{Paper, Rock, Scissors};
use crate::day_2::Outcome::{Draw, Loss, Win};

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Eq, PartialEq, Debug)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

type Round = (Move, Move);
type Tournament = Vec<Round>;

/// The entry point for running the solutions with the 'real' puzzle input.
//
/// - The puzzle input is expected to be at `<project_root>/res/day-2-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 2.
pub fn run() {
    let contents = fs::read_to_string("res/day-02-input").expect("Failed to read file");
    let tournament = parse_strategy(&contents, parse_moves_line);

    println!(
        "Following the guide assuming moves, your score would be: {}",
        score_tournament(&tournament)
    );

    let tournament2 = parse_strategy(&contents, parse_outcome_line);

    println!(
        "Following the guide assuming outcomes, your score would be: {}",
        score_tournament(&tournament2)
    );
}

fn parse_strategy(strategy: &String, syntax: fn(&str) -> Round) -> Tournament {
    strategy.lines()
            .map(syntax)
            .collect()
}

fn parse_moves_line(line: &str) -> Round {
    let (part_1, part_2) = line.split_at(1);
    (
        parse_move(part_1).unwrap(),
        parse_move(part_2).unwrap()
    )
}

fn parse_outcome_line(line: &str) -> Round {
    let (part_1, part_2) = line.split_at(1);
    resolve_outcome(
        parse_move(part_1).unwrap(),
        parse_outcome(part_2).unwrap(),
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

fn parse_outcome(chr: &str) -> Option<Outcome> {
    match chr {
        " X" => Some(Loss),
        " Y" => Some(Draw),
        " Z" => Some(Win),
        _ => None
    }
}

fn resolve_outcome(their_move: Move, outcome: Outcome) -> Round {
    let your_move = match outcome {
        Loss => loss_for(their_move),
        Draw => draw_for(their_move),
        Win => win_for(their_move)
    };

    (their_move, your_move)
}

fn score_round(round: &Round) -> u32 {
    score_result(round) + score_move(round)
}

fn score_result(round: &Round) -> u32 {
    match round {
        &(their, your) if win_for(their) == your => 6,
        &(their, your) if draw_for(their) == your => 3,
        (_, _) => 0
    }
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

fn score_move((_, your_move): &Round) -> u32 {
    match your_move {
        Rock => 1,
        Paper => 2,
        Scissors => 3
    }
}

fn score_tournament(tournament: &Tournament) -> u32 {
    tournament.into_iter().map(score_round).sum()
}

#[cfg(test)]
mod tests {
    use crate::day_2::{parse_strategy, parse_moves_line, parse_outcome_line, score_round, score_tournament, Tournament};
    use crate::day_2::Move::{Paper, Rock, Scissors};

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

    fn sample_moves_tournament() -> Tournament {
        vec![
            (Rock, Paper),
            (Paper, Rock),
            (Scissors, Scissors),
        ]
    }

    fn sample_outcome_tournament() -> Tournament {
        vec![
            (Rock, Rock),
            (Paper, Rock),
            (Scissors, Rock),
        ]
    }

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

        assert_eq!(
            score_tournament(&sample_outcome_tournament()),
            12
        );
    }
}

//! This is my solution for [Advent of Code - Day 19 - _Title_](https://adventofcode.com/2022/day/19)
//!
//!

use std::collections::{HashSet, VecDeque};
use std::fs;
use crate::day_19::Resource::{CLAY, GEODE, OBSIDIAN, ORE};

#[derive(Eq, PartialEq, Debug, Ord, PartialOrd, Copy, Clone)]
enum Resource {
    ORE,
    CLAY,
    OBSIDIAN,
    GEODE,
}

#[derive(Eq, PartialEq, Debug)]
struct Blueprint {
    ore: usize,
    clay: usize,
    obsidian: (usize, usize),
    geode: (usize, usize),
}

impl From<&str> for Blueprint {
    // Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore.
    // Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
    fn from(line: &str) -> Self {
        let mut numbers =
            line.split_whitespace()
                // Note also filter `:<id>` as it is not all digits
                .filter(|&word| word.chars().all(|c| c.is_digit(10)))
                .map(|word| word.parse::<usize>().unwrap());

        Blueprint {
            ore: numbers.next().unwrap(),
            clay: numbers.next().unwrap(),
            obsidian: (numbers.next().unwrap(), numbers.next().unwrap()),
            geode: (numbers.next().unwrap(), numbers.next().unwrap()),
        }
    }
}

impl Blueprint {
    fn cost(&self, res: &Resource) -> Counts {
        match res {
            ORE => Counts { ore: self.ore, ..Counts::new() },
            CLAY => Counts { ore: self.clay, ..Counts::new() },
            OBSIDIAN => Counts { ore: self.obsidian.0, clay: self.obsidian.1, ..Counts::new() },
            GEODE => Counts { ore: self.geode.0, obsidian: self.geode.1, ..Counts::new() },
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct Counts {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Counts {
    fn new() -> Counts {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    fn one(res: &Resource) -> Self {
        match res {
            ORE => Self { ore: 1, ..Counts::new() },
            CLAY => Self { clay: 1, ..Counts::new() },
            OBSIDIAN => Self { obsidian: 1, ..Counts::new() },
            GEODE => Self { geode: 1, ..Counts::new() },
        }
    }

    fn has(&self, other: &Counts) -> bool {
        self.ore >= other.ore &&
            self.clay >= other.clay &&
            self.obsidian >= other.obsidian &&
            self.geode >= other.geode
    }

    fn merge(&self, other: &Self) -> Self {
        Self {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geode: self.geode + other.geode,
        }
    }

    fn diff(&self, other: &Self) -> Self {
        Self {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct State {
    resources: Counts,
    robots: Counts,
    time: usize,
}

impl State {
    fn new(time: usize) -> State {
        return Self {
            resources: Counts::new(),
            robots: Counts { ore: 1, ..Counts::new() },
            time,
        };
    }

    fn produce(&self) -> Self {
        Self {
            resources: self.resources.merge(&self.robots),
            robots: self.robots.clone(),
            time: self.time - 1,
        }
    }

    fn build_robot(&self, blueprint: &Blueprint, resource: &Resource) -> Self {
        let cost = blueprint.cost(resource);

        Self {
            resources: self.resources.diff(&cost).merge(&self.robots),
            robots: self.robots.merge(&Counts::one(resource)),
            time: self.time - 1,
        }
    }
}


/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-19-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 19.
pub fn run() {
    let contents = fs::read_to_string("res/day-19-input").expect("Failed to read file");
    let blueprints = parse_input(&contents);

    println!(
        "The sum of the blueprint's quality level is {}",
        get_quality_level_sum(&blueprints, 24)
    );

    println!(
        "The sum of the blueprint's quality level is {}",
        get_max_geode_product(&blueprints.into_iter().take(3).collect(), 32)
    );
}

fn parse_input(input: &String) -> Vec<Blueprint> {
    input.lines().map(Blueprint::from).collect()
}

fn get_max_geodes(blueprint: &Blueprint, time_limit: usize) -> usize {
    let mut unseen = VecDeque::new();
    let starting_state = State::new(time_limit);
    let mut best_geodes = 0;
    let mut most_geode_robots = 0;
    unseen.push_back(starting_state);

    let mut seen_states = HashSet::new();

    // Don't make more ore robots than can the max ore cost of any robot
    let max_ore_cost = *[
        blueprint.ore,
        blueprint.clay,
        blueprint.obsidian.0,
        blueprint.geode.0
    ].iter().max().unwrap();

    while let Some(state) = unseen.pop_front() {
        let mut cacheable_state = state;
        cacheable_state.time = time_limit;
        if state.robots.geode + 1 < most_geode_robots || seen_states.contains(&cacheable_state) || state.time == 0 {
            best_geodes = best_geodes.max(state.resources.geode);
            continue;
        }
        most_geode_robots = most_geode_robots.max(state.robots.geode);

        seen_states.insert(cacheable_state);

        if state.resources.has(&blueprint.cost(&GEODE)) {
            unseen.push_back(state.build_robot(blueprint, &GEODE));
            continue;
        }

        if state.resources.has(&blueprint.cost(&ORE)) && state.robots.ore < max_ore_cost {
            unseen.push_back(state.build_robot(blueprint, &ORE));
        }
        if state.resources.has(&blueprint.cost(&CLAY)) && state.robots.clay < blueprint.obsidian.1 {
            unseen.push_back(state.build_robot(blueprint, &CLAY));
        }
        if state.resources.has(&blueprint.cost(&OBSIDIAN)) && state.robots.obsidian < blueprint.geode.1 {
            unseen.push_back(state.build_robot(&blueprint, &OBSIDIAN));
        }

        unseen.push_back(state.produce());
    }

    best_geodes
}

fn get_quality_level_sum(blueprints: &Vec<Blueprint>, time_limit: usize) -> usize {
    blueprints.into_iter().enumerate()
              .map(|(i, bp)| (i + 1) * get_max_geodes(bp, time_limit))
              .sum()
}

fn get_max_geode_product(blueprints: &Vec<Blueprint>, time_limit: usize) -> usize {
    blueprints.into_iter()
              .map(|bp| get_max_geodes(bp, time_limit))
              .fold(
                  1,
                  |acc, geodes| acc * geodes,
              )
}

#[cfg(test)]
mod tests {
    use crate::day_19::{Blueprint, get_max_geode_product, get_max_geodes, get_quality_level_sum, parse_input};

    fn sample_blueprints() -> Vec<Blueprint> {
        vec![
            Blueprint { ore: 4, clay: 2, obsidian: (3, 14), geode: (2, 7) },
            Blueprint { ore: 2, clay: 3, obsidian: (3, 8), geode: (3, 12) },
        ]
    }

    #[test]
    fn can_parse() {
        let input = "Blueprint 1: \
  Each ore robot costs 4 ore. \
  Each clay robot costs 2 ore. \
  Each obsidian robot costs 3 ore and 14 clay. \
  Each geode robot costs 2 ore and 7 obsidian.
\
Blueprint 2: \
  Each ore robot costs 2 ore. \
  Each clay robot costs 3 ore. \
  Each obsidian robot costs 3 ore and 8 clay. \
  Each geode robot costs 3 ore and 12 obsidian.".to_string();

        assert_eq!(parse_input(&input), sample_blueprints());
    }

    #[test]
    fn can_get_max_geodes_for_blue_print() {
        let blueprints = sample_blueprints();
        assert_eq!(get_max_geodes(&blueprints[0], 24), 9);
        assert_eq!(get_max_geodes(&blueprints[1], 24), 12);

        assert_eq!(get_max_geodes(&blueprints[0], 32), 56);
        assert_eq!(get_max_geodes(&blueprints[1], 32), 62);
    }

    #[test]
    fn can_sum_quality_level() {
        assert_eq!(get_quality_level_sum(&sample_blueprints(), 24), 33)
    }

    #[test]
    fn can_get_geode_product() {
        assert_eq!(get_max_geode_product(&sample_blueprints(), 32), 62 * 56)
    }
}

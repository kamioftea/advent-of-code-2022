//! This is my solution for [Advent of Code - Day 16 - _Proboscidea Volcanium_](https://adventofcode.com/2022/day/16)
//!
//!

use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use itertools::Itertools;

#[derive(Eq, PartialEq, Debug)]
struct Valve {
    flow: usize,
    links: Vec<usize>,
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-16-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 16.
pub fn run() {
    let contents = fs::read_to_string("res/day-16-input").expect("Failed to read file");
    let valves = parse_input(&contents);

    println!(
        "The maximal flow rate alone is: {}",
        find_best_flow(&valves, 27, 30),
    );

    println!(
        "The maximal flow rate with an elephant is: {}",
        find_best_flow_with_elephant(&valves, 27, 26),
    );
}

fn parse_input(input: &String) -> HashMap<usize, Valve> {
    input.lines().map(parse_valve).collect()
}

fn parse_valve(line: &str) -> (usize, Valve) {
    //Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    let mut parts = line.split_whitespace().dropping(1);
    let id = id_to_usize(parts.next().unwrap());

    parts = parts.dropping(2);
    let flow =
        parts.next().unwrap()
             .split_once("=")
             .and_then(|(_, flow)| flow.trim_matches(';').parse::<usize>().ok())
             .unwrap();

    let links: Vec<usize> = parts.dropping(4).map(|id| id.trim_matches(',')).map(id_to_usize).collect();

    (id, Valve { flow, links })
}

fn id_to_usize(id: &str) -> usize {
    id.chars().fold(0, |acc, c| 26 * acc + usize::from(u8::try_from(c).unwrap() & 0b11111))
}

fn search_best_path(
    current_id: usize,
    shortest_paths: &HashMap<(usize, usize), usize>,
    valves: &HashMap<usize, Valve>,
    time_left: usize,
    flow: usize,
    active: HashSet<usize>,
) -> usize {
    let flow_rates: Vec<usize> = shortest_paths.into_iter()
        .filter(|((s, e), &d)| s == &current_id && !active.contains(e) && d < time_left)
        .map(|(&(_, e), d)| {
            let mut new_active = active.clone();
            new_active.insert(e);

            search_best_path(
                e,
                &shortest_paths,
                &valves,
                time_left - d - 1,
                flow + valves.get(&e).unwrap().flow * (time_left - d - 1),
                new_active,
            )
        }).collect();

    flow_rates.into_iter().max().unwrap_or(flow)
}

fn find_best_flow(valves: &HashMap<usize, Valve>, start: usize, time: usize) -> usize {
    let shortest_paths = build_shortest_paths(start, &valves);

    search_best_path(start, &shortest_paths, &valves, time, 0, HashSet::new())
}

fn search_best_path_with_elephant(
    current_ids: (usize, usize),
    shortest_paths: &HashMap<(usize, usize), usize>,
    valves: &HashMap<usize, Valve>,
    times_left: (usize, usize),
    flow: usize,
    active: HashSet<usize>,
) -> usize {
    let mut flow_rates = vec![flow];
    let (current_id, time_left, is_elephant) = if times_left.0 >= times_left.1 {
        (current_ids.0, times_left.0, false)
    } else {
        (current_ids.1, times_left.1, true)
    };

    shortest_paths
        .into_iter()
        .filter(|(&(s, e), &d)| s == current_id && !active.contains(&e) && d < time_left)
        .map(|(&(_, e), &d)| {
            let mut new_active = active.clone();
            new_active.insert(e);

            search_best_path_with_elephant(
                if is_elephant {(current_ids.0, e)} else {(e, current_ids.1)},
                &shortest_paths,
                &valves,
                if is_elephant {(times_left.0, time_left - d - 1)} else {(time_left - d - 1, times_left.1)},
                flow + valves.get(&e).unwrap().flow * (time_left - d - 1),
                new_active,
            )
        })
        .for_each(|f| flow_rates.push(f));

    flow_rates.into_iter().max().unwrap()
}

fn find_best_flow_with_elephant(valves: &HashMap<usize, Valve>, start: usize, time: usize) -> usize {
    let shortest_paths = build_shortest_paths(start, &valves);

    search_best_path_with_elephant(
        (start, start),
        &shortest_paths,
        &valves,
        (time, time),
        0,
        HashSet::new(),
    )
}

fn build_shortest_paths(start: usize, valves: &HashMap<usize, Valve>) -> HashMap<(usize, usize), usize> {
    let mut openable_valves: HashSet<usize> =
        valves.into_iter()
              .flat_map(|(&id, v)| if v.flow == 0 { None } else { Some(id) })
              .collect();

    let mut permutations: HashMap<(usize, usize), usize> =
        build_shortest_paths_from_valve(start, &openable_valves, valves)
            .into_iter()
            .collect();

    let mut nodes_to_build: Vec<usize> = openable_valves.iter().map(|&s| s).collect();

    let start_valve = valves.get(&start).unwrap();
    if start_valve.flow == 0 {
        nodes_to_build.push(start);
    }

    while let Some(id) = nodes_to_build.pop() {
        openable_valves.remove(&id);
        for ((a, b), v) in build_shortest_paths_from_valve(id, &openable_valves, &valves) {
            permutations.insert((a, b), v);
            if start_valve.flow != 0 || a != start {
                permutations.insert((b, a), v);
            }
        }
    }

    permutations
}

fn build_shortest_paths_from_valve(start: usize, target_nodes: &HashSet<usize>, valves: &HashMap<usize, Valve>)
                                   -> Vec<((usize, usize), usize)> {
    let mut nodes: HashMap<usize, usize> = HashMap::new();
    let mut to_visit: VecDeque<(usize, usize)> = VecDeque::new();
    to_visit.push_back((start, 0));
    let mut visited = HashSet::new();
    visited.insert(start);

    while let Some((id, depth)) = to_visit.pop_front() {
        let valve = valves.get(&id).unwrap();

        for &link in &valve.links {
            if !visited.contains(&link) {
                if target_nodes.contains(&link) {
                    nodes.insert(link, depth + 1);
                }
                to_visit.push_back((link, depth + 1));
                visited.insert(link);
            }
        }
    }

    nodes.into_iter().map(|(end, path_len)| ((start, end), path_len)).collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::day_16::{build_shortest_paths, find_best_flow, find_best_flow_with_elephant, parse_input, Valve};

    fn sample_valves() -> HashMap<usize, Valve> {
        let list: Vec<(usize, Valve)> = vec![
            (27, Valve { flow: 0, links: vec![108, 243, 54] }),
            (54, Valve { flow: 13, links: vec![81, 27] }),
            (81, Valve { flow: 2, links: vec![108, 54] }),
            (108, Valve { flow: 20, links: vec![81, 27, 135] }),
            (135, Valve { flow: 3, links: vec![162, 108] }),
            (162, Valve { flow: 0, links: vec![135, 189] }),
            (189, Valve { flow: 0, links: vec![162, 216] }),
            (216, Valve { flow: 22, links: vec![189] }),
            (243, Valve { flow: 0, links: vec![27, 270] }),
            (270, Valve { flow: 21, links: vec![243] }),
        ];

        list.into_iter().collect()
    }

    #[test]
    fn can_parse() {
        let sample_input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II".to_string();

        let map = parse_input(&sample_input);

        for (id, valve) in sample_valves() {
            assert_eq!(
                map.get(&id),
                Some(&valve),
                "id: {} gives the correct valve", id
            );
        }

        assert_eq!(map, sample_valves());
    }

    #[test]
    fn can_find_shortest_paths() {
        let expected_map: HashMap<(usize, usize), usize> = vec![
            ((27, 54), 1),
            ((27, 81), 2),
            ((27, 108), 1),
            ((27, 135), 2),
            ((27, 216), 5),
            ((27, 270), 2),
            ((54, 81), 1),
            ((81, 54), 1),
            ((54, 108), 2),
            ((108, 54), 2),
            ((54, 135), 3),
            ((135, 54), 3),
            ((54, 216), 6),
            ((216, 54), 6),
            ((54, 270), 3),
            ((270, 54), 3),
            ((81, 108), 1),
            ((108, 81), 1),
            ((81, 135), 2),
            ((135, 81), 2),
            ((81, 216), 5),
            ((216, 81), 5),
            ((81, 270), 4),
            ((270, 81), 4),
            ((108, 135), 1),
            ((135, 108), 1),
            ((108, 216), 4),
            ((216, 108), 4),
            ((108, 270), 3),
            ((270, 108), 3),
            ((135, 216), 3),
            ((216, 135), 3),
            ((135, 270), 4),
            ((270, 135), 4),
            ((216, 270), 7),
            ((270, 216), 7),
        ].into_iter().collect();

        let map = build_shortest_paths(27, &sample_valves());

        for (id, valve) in &expected_map {
            assert_eq!(
                map.get(id),
                Some(valve),
                "id: {:?} gives the correct valve", id
            );
        }

        assert_eq!(
            map,
            expected_map
        );
    }

    #[test]
    fn can_find_best_flow() {
        assert_eq!(
            find_best_flow(&sample_valves(), 27, 30),
            1651
        );
    }

    #[test]
    fn can_find_best_flow_with_elephant() {
        assert_eq!(
            find_best_flow_with_elephant(&sample_valves(), 27, 26),
            1707
        );
    }
}

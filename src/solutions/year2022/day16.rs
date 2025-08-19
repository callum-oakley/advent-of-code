use std::collections::BTreeMap;

use regex::Regex;

use crate::part::Part;

fn parse(input: &str) -> (BTreeMap<&str, usize>, BTreeMap<&str, Vec<&str>>) {
    let mut valves = BTreeMap::new();
    let mut tunnels = BTreeMap::new();
    let re = Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? ([\w, ]+)")
        .unwrap();
    for captures in re.captures_iter(input) {
        let valve = captures.get(1).unwrap().as_str();
        let flow: usize = captures[2].parse().unwrap();
        if flow > 0 {
            valves.insert(valve, flow);
        }
        tunnels.insert(
            valve,
            captures.get(3).unwrap().as_str().split(", ").collect(),
        );
    }
    (valves, tunnels)
}

// Distance from a to b along the shortest route.
fn distance(tunnels: &BTreeMap<&str, Vec<&str>>, a: &str, b: &str) -> usize {
    crate::search::breadth_first(
        (a, 0),
        |(valve, steps), push| {
            for &tunnel in &tunnels[valve] {
                push((tunnel, steps + 1));
            }
        },
        crate::search::hash_filter(|&(valve, _)| valve),
    )
    .find(|&(valve, _)| valve == b)
    .unwrap()
    .1
}

fn part_(part: Part, input: &str) -> usize {
    #[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    struct Worker<'a> {
        pos: &'a str,
        minutes: usize,
    }

    #[derive(Clone, PartialEq, Eq, Hash)]
    struct State<'a> {
        workers: Vec<Worker<'a>>,
        pressure: usize,
        valves: BTreeMap<&'a str, usize>,
    }

    let (valves, tunnels) = parse(input);

    let mut distances: BTreeMap<&str, BTreeMap<&str, usize>> = BTreeMap::new();
    for &a in tunnels.keys() {
        for &b in tunnels.keys() {
            distances
                .entry(a)
                .or_default()
                .insert(b, distance(&tunnels, a, b));
        }
    }

    crate::search::branch_and_bound_max(
        State {
            workers: match part {
                Part::One => vec![Worker {
                    pos: "AA",
                    minutes: 0,
                }],
                Part::Two => vec![
                    Worker {
                        pos: "AA",
                        minutes: 4
                    };
                    2
                ],
            },
            pressure: 0,
            valves,
        },
        |state, push| {
            for i in 0..state.workers.len() {
                for (&valve, &flow) in &state.valves {
                    let mut state = state.clone();
                    state.workers[i].minutes += distances[state.workers[i].pos][valve] + 1;
                    if state.workers[i].minutes < 30 {
                        state.valves.remove(valve);
                        state.workers[i].pos = valve;
                        state.pressure += flow * (30 - state.workers[i].minutes);
                        push(state);
                    }
                }
            }
        },
        crate::search::hash_filter(|state: &State| {
            let mut state = state.clone();
            state.workers.sort_unstable();
            state
        }),
        |state| state.pressure,
        // For our bound, suppose that we open valves in order from highest flow rate to lowest, and
        // that each valve we open happens to also be the closest valve to the worker with the most
        // time remaining.
        |state| {
            let mut state = state.clone();
            while let Some((&valve, &flow)) = state.valves.iter().max_by_key(|&(_, flow)| flow) {
                let i = (0..state.workers.len())
                    .min_by_key(|&i| state.workers[i].minutes)
                    .unwrap();
                state.workers[i].minutes += state
                    .valves
                    .keys()
                    .map(|valve| distances[state.workers[i].pos][valve])
                    .min()
                    .unwrap()
                    + 1;
                state.valves.remove(valve);
                if state.workers[i].minutes >= 30 {
                    break;
                }
                state.workers[i].pos = valve;
                state.pressure += flow * (30 - state.workers[i].minutes);
            }
            state.pressure
        },
    )
    .pressure
}

pub fn part1(input: &str) -> usize {
    part_(Part::One, input)
}

pub fn part2(input: &str) -> usize {
    part_(Part::Two, input)
}

pub fn tests() {
    let example = "
        Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II
    ";
    assert_eq!(part1(example), 1651);
    assert_eq!(part2(example), 1707);
}

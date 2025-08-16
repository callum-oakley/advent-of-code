use std::{cmp::Reverse, collections::BTreeMap};

use regex::Regex;

fn parse(input: &str) -> (BTreeMap<&str, usize>, BTreeMap<&str, Vec<&str>>) {
    let mut valves = BTreeMap::new();
    let mut tunnels = BTreeMap::new();
    let re = Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? ([\w, ]+)")
        .unwrap();
    for captures in re.captures_iter(input) {
        let valve = captures.get(1).unwrap().as_str();
        valves.insert(valve, captures[2].parse().unwrap());
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

pub fn part1(input: &str) -> usize {
    #[derive(Clone, PartialEq, Eq, Hash)]
    struct State<'a> {
        pos: &'a str,
        minutes: usize,
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

    crate::search::branch_and_bound(
        State {
            pos: "AA",
            minutes: 0,
            pressure: 0,
            valves,
        },
        |state, push| {
            for &valve in state.valves.keys() {
                let minutes = state.minutes + distances[state.pos][valve] + 1;
                if minutes < 30 {
                    let mut valves = state.valves.clone();
                    let flow = valves.remove(valve).unwrap();
                    push(State {
                        pos: valve,
                        minutes,
                        pressure: state.pressure + flow * (30 - minutes),
                        valves,
                    });
                }
            }
        },
        crate::search::id_filter(),
        |state| {
            let mut valves: Vec<&str> = state.valves.keys().copied().collect();
            valves.sort_unstable_by_key(|valve| Reverse(state.valves[valve]));
            let mut pressure = state.pressure;
            let mut minutes = state.minutes;
            for valve in valves {
                // Best case the valve we want to open is one step away.
                minutes += 2;
                if minutes >= 30 {
                    break;
                }
                pressure += state.valves[valve] * (30 - minutes);
            }
            Reverse(pressure)
        },
        |state| Reverse(state.pressure),
    )
    .pressure
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
    // assert_eq!(part2(example), todo!());
}

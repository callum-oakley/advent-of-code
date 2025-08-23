use std::{
    collections::{BTreeMap, BTreeSet},
    sync::LazyLock,
};

use regex::Regex;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Blueprint<'a> {
    id: usize,
    costs: BTreeMap<&'a str, BTreeMap<&'a str, usize>>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct State<'a> {
    minutes: usize,
    robots: BTreeMap<&'a str, usize>,
    resources: BTreeMap<&'a str, usize>,
    skipped: BTreeSet<&'a str>,
}

fn parse(input: &str) -> impl Iterator<Item = Blueprint> {
    static ID: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"Blueprint (\d+):").unwrap());
    static ROBOT: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"Each (\w+) robot costs ([^\.]+)\.").unwrap());
    static COST: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\d+) (\w+)").unwrap());
    input.lines().map(|line| Blueprint {
        id: ID.captures(line).unwrap()[1].parse().unwrap(),
        costs: ROBOT
            .captures_iter(line)
            .map(|robot_captures| {
                (
                    robot_captures.get(1).unwrap().as_str(),
                    COST.captures_iter(robot_captures.get(2).unwrap().as_str())
                        .map(|cost_captures| {
                            (
                                cost_captures.get(2).unwrap().as_str(),
                                cost_captures[1].parse().unwrap(),
                            )
                        })
                        .collect(),
                )
            })
            .collect(),
    })
}

fn buildable(blueprint: &Blueprint, state: &State, robot: &str) -> bool {
    blueprint.costs[robot].iter().all(|(&resource, &need)| {
        state
            .resources
            .get(resource)
            .is_some_and(|&have| have >= need)
    })
}

fn build<'a>(blueprint: &Blueprint<'a>, state: &mut State<'a>, robot: &'a str) {
    for (resource, need) in &blueprint.costs[robot] {
        *state.resources.get_mut(resource).unwrap() -= need;
    }
    *state.robots.entry(robot).or_default() += 1;
    state.skipped = BTreeSet::new();
}

fn geodes(minutes: usize, blueprint: &Blueprint) -> usize {
    crate::search::branch_and_bound_max(
        State {
            minutes: 0,
            robots: BTreeMap::from([("ore", 1)]),
            resources: BTreeMap::new(),
            skipped: BTreeSet::new(),
        },
        |state, push| {
            if state.minutes == minutes {
                return;
            }

            let options: BTreeSet<&str> = blueprint
                .costs
                .keys()
                .copied()
                .filter(|robot| buildable(blueprint, state, robot))
                .collect();

            let mut state = state.clone();
            state.minutes += 1;
            for (&robot, &n) in &state.robots {
                *state.resources.entry(robot).or_default() += n;
            }

            // OPTIMISATION
            // If we can afford a geode robot always build it immediately.
            if options.contains("geode") {
                build(blueprint, &mut state, "geode");
                push(state);
                return;
            }

            for &robot in &options {
                // OPTIMISATION
                // If we've already got a lot of some resource, don't bother building more robots
                // for that resource.
                if !state.skipped.contains(robot)
                    && state.resources.get(robot).is_none_or(|&have| have <= 22)
                {
                    let mut state = state.clone();
                    build(blueprint, &mut state, robot);
                    push(state);
                }
            }

            // OPTIMISATION
            // Even if we can afford to build a robot, it might be better to wait. If we do
            // wait take note of the robots we skipped building, because there's no point
            // building them next turn when we could have built them this turn.
            {
                state.skipped.extend(options);
                push(state);
            }
        },
        crate::search::id_filter(),
        |state| state.resources.get("geode").copied().unwrap_or_default(),
        // For a crude upper bound, suppose we can build a geode robot in every remaining
        // minute.
        |state| {
            let m = minutes - state.minutes;
            let g = state.resources.get("geode").copied().unwrap_or_default();
            let r = state.robots.get("geode").copied().unwrap_or_default();
            g + m * r + m * (m - 1) / 2
        },
    )
    .resources
    .get("geode")
    .copied()
    .unwrap_or_default()
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .map(|blueprint| geodes(24, &blueprint) * blueprint.id)
        .sum()
}

pub fn part2(input: &str) -> usize {
    parse(input)
        .take(3)
        .map(|blueprint| geodes(32, &blueprint))
        .product()
}

pub fn tests() {
    let example = concat!(
        "Blueprint 1: ",
        "Each ore robot costs 4 ore. ",
        "Each clay robot costs 2 ore. ",
        "Each obsidian robot costs 3 ore and 14 clay. ",
        "Each geode robot costs 2 ore and 7 obsidian.\n",
        "Blueprint 2: ",
        "Each ore robot costs 2 ore. ",
        "Each clay robot costs 3 ore. ",
        "Each obsidian robot costs 3 ore and 8 clay. ",
        "Each geode robot costs 3 ore and 12 obsidian.\n",
    );
    assert_eq!(part1(example), 33);
    assert_eq!(geodes(32, &parse(example).next().unwrap()), 56);
    assert_eq!(geodes(32, &parse(example).nth(1).unwrap()), 62);
}

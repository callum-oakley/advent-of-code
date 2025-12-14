use std::collections::{BTreeSet, HashMap};

use crate::{
    grid::{Adjacent, E, Grid, N, NE, NW, S, SE, SW, Vector, W, Z},
    search,
};

fn is_open(tile: char) -> bool {
    tile != '#'
}

fn is_key(tile: char) -> bool {
    ['@', '0', '1', '2', '3'].contains(&tile) || tile.is_lowercase()
}

fn is_door(tile: char) -> bool {
    tile.is_uppercase()
}

fn key(door: char) -> char {
    door.to_ascii_lowercase()
}

// Path represents a walk to another key, the steps and keys required.
struct Path {
    dest: char,
    steps: usize,
    keys: BTreeSet<char>,
}

fn reachable(map: &Grid<char>, pos: Vector) -> Vec<Path> {
    #[derive(Clone)]
    struct State {
        pos: Vector,
        steps: usize,
        keys: BTreeSet<char>,
        found_key: bool,
    }

    search::breadth_first(
        State {
            pos,
            steps: 0,
            keys: BTreeSet::new(),
            found_key: false,
        },
        |state, push| {
            if state.found_key {
                return;
            }
            state
                .pos
                .adjacent4()
                .filter(|&v| map.get(v).is_some_and(|&tile| is_open(tile)))
                .for_each(|pos| {
                    let mut state = state.clone();
                    state.pos = pos;
                    state.steps += 1;
                    if is_door(map[pos]) {
                        state.keys.insert(key(map[pos]));
                    } else if is_key(map[pos]) {
                        state.found_key = true;
                    }
                    push(state);
                });
        },
        search::hash_filter(|state: &State| state.pos),
    )
    .filter(|state| state.found_key)
    .map(|state| Path {
        dest: map[state.pos],
        steps: state.steps,
        keys: state.keys,
    })
    .collect()
}

fn key_graph(map: &Grid<char>) -> HashMap<char, Vec<Path>> {
    map.iter()
        .filter(|&(_, &tile)| is_key(tile))
        .map(|(pos, &tile)| (tile, reachable(map, pos)))
        .collect()
}

fn part_(map: &Grid<char>, robots: Vec<char>) -> usize {
    #[derive(Clone, PartialOrd, Ord, PartialEq, Eq)]
    struct State {
        robots: Vec<char>,
        steps: usize,
        keys: BTreeSet<char>,
    }

    // First precompute the paths between keys (the number of steps and the keys required to
    // traverse them).
    let key_graph = key_graph(map);

    let mut initial_keys = BTreeSet::new();
    initial_keys.extend(&robots);

    let final_key_count = map.values().filter(|&&tile| is_key(tile)).count();
    let min_path_steps = key_graph
        .values()
        .flatten()
        .map(|path| path.steps)
        .min()
        .unwrap();

    // Then A* on this higher level graph is fast enough.
    search::a_star(
        State {
            robots,
            steps: 0,
            keys: initial_keys,
        },
        move |state, push| {
            for i in 0..state.robots.len() {
                for path in key_graph[&state.robots[i]]
                    .iter()
                    .filter(|path| path.keys.is_subset(&state.keys))
                {
                    let mut state = state.clone();
                    state.robots[i] = path.dest;
                    state.steps += path.steps;
                    state.keys.insert(path.dest);
                    push(state);
                }
            }
        },
        search::hash_filter(|state: &State| (state.robots.clone(), state.keys.clone())),
        |state| state.steps,
        // For each key left to collect, we'll have to move at least min_path_steps.
        move |state| (final_key_count - state.keys.len()) * min_path_steps,
    )
    .find(|state| state.keys.len() == final_key_count)
    .unwrap()
    .steps
}

pub fn part1(input: &str) -> usize {
    part_(&Grid::parse(input, |_, tile| tile), vec!['@'])
}

pub fn part2(input: &str) -> usize {
    let mut map = Grid::parse(input, |_, tile| tile);
    let (start, _) = map.iter().find(|&(_, &tile)| tile == '@').unwrap();
    for dir in [Z, N, E, S, W] {
        map[start + dir] = '#';
    }
    map[start + NE] = '0';
    map[start + SE] = '1';
    map[start + SW] = '2';
    map[start + NW] = '3';
    part_(&map, vec!['0', '1', '2', '3'])
}

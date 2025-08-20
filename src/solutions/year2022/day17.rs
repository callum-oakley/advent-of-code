use std::{
    collections::{HashMap, HashSet},
    sync::LazyLock,
};

use crate::grid::{IntoVector, Vector, E, N, S};

static ROCKS: LazyLock<Vec<Vec<Vector>>> = LazyLock::new(|| {
    "####\n\n.#.\n###\n.#.\n\n..#\n..#\n###\n\n#\n#\n#\n#\n\n##\n##"
        .split("\n\n")
        .map(|s| {
            let mut rock: Vec<Vector> = crate::grid::scan(s)
                .filter(|&(_, c)| c == '#')
                .map(|(v, _)| v)
                .collect();
            let max_y = rock.iter().map(|v| v.y).max().unwrap();
            for v in &mut rock {
                v.y = max_y - v.y;
            }
            rock
        })
        .collect()
});

fn parse(input: &str) -> Vec<Vector> {
    input.chars().map(IntoVector::into_vector).collect()
}

fn push(tower: &HashSet<Vector>, rock: &[Vector], dir: Vector) -> Option<Vec<Vector>> {
    let r: Vec<Vector> = rock.iter().map(|&v| v + dir).collect();
    if r.iter()
        .all(|v| v.x >= 0 && v.x < 7 && v.y >= 0 && !tower.contains(v))
    {
        Some(r)
    } else {
        None
    }
}

#[derive(Clone, Copy, Debug)]
struct State {
    height: i64,
    r: usize,
    j: usize,
}

fn next<'a, T>(items: &'a [T], i: &mut usize) -> &'a T {
    let item = &items[*i % items.len()];
    *i += 1;
    item
}

fn simulate(jets: &[Vector]) -> impl Iterator<Item = State> {
    let mut tower = HashSet::new();
    std::iter::successors(
        Some(State {
            height: 0,
            r: 0,
            j: 0,
        }),
        move |&(mut state)| {
            let rock = next(&ROCKS, &mut state.r);
            let mut rock = push(&tower, rock, 2 * E + (state.height + 3) * S).unwrap();
            loop {
                let jet = *next(jets, &mut state.j);
                if let Some(r) = push(&tower, &rock, jet) {
                    rock = r;
                }
                if let Some(r) = push(&tower, &rock, N) {
                    rock = r;
                } else {
                    break;
                }
            }
            state.height = state
                .height
                .max(rock.iter().map(|v| v.y).max().unwrap() + 1);
            tower.extend(rock);
            Some(state)
        },
    )
}

pub fn part1(input: &str) -> i64 {
    simulate(&parse(input)).nth(2022).unwrap().height
}

// After looking at states where r is a multiple of 5 and grouping by j it's clear that the height
// settles in to a linear sequence. We can extrapolate from the first pair where the period evenly
// divides the target.
pub fn part2(input: &str) -> i64 {
    let jets = parse(input);
    let mut seen: HashMap<usize, State> = HashMap::new();
    for state in simulate(&jets) {
        if state.r % ROCKS.len() == 0 {
            if let Some(prev) = seen.get(&(state.j % jets.len())) {
                let r_diff = state.r - prev.r;
                if (1_000_000_000_000 - state.r) % r_diff == 0 {
                    let height_diff = state.height - prev.height;
                    let cycles = (1_000_000_000_000 - state.r) / r_diff;
                    return state.height + i64::try_from(cycles).unwrap() * height_diff;
                }
            }
            seen.insert(state.j % jets.len(), state);
        }
    }
    unreachable!()
}

pub fn tests() {
    let example = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    assert_eq!(part1(example), 3068);
    assert_eq!(part2(example), 1_514_285_714_288);
}

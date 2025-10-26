use std::collections::HashMap;

use crate::grid::{Adjacent, Grid};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Open,
    Trees,
    Lumberyard,
}

fn parse(input: &str) -> Grid<Tile> {
    Grid::parse(input, |_, c| match c {
        '.' => Tile::Open,
        '|' => Tile::Trees,
        '#' => Tile::Lumberyard,
        _ => unreachable!(),
    })
}

fn tick(state: &Grid<Tile>) -> Grid<Tile> {
    let mut res = state.clone();
    for pos in state.keys() {
        let count_adj = |tile| {
            pos.adjacent8()
                .filter(|&v| state.get(v).is_some_and(|&t| t == tile))
                .count()
        };
        match state[pos] {
            Tile::Open => {
                if count_adj(Tile::Trees) >= 3 {
                    res[pos] = Tile::Trees;
                }
            }
            Tile::Trees => {
                if count_adj(Tile::Lumberyard) >= 3 {
                    res[pos] = Tile::Lumberyard;
                }
            }
            Tile::Lumberyard => {
                if count_adj(Tile::Lumberyard) < 1 || count_adj(Tile::Trees) < 1 {
                    res[pos] = Tile::Open;
                }
            }
        }
    }
    res
}

fn find_cycle(mut state: Grid<Tile>) -> (usize, usize) {
    let mut seen = HashMap::new();
    let mut min = 0;
    while !seen.contains_key(&state) {
        let next_state = tick(&state);
        seen.insert(state, min);
        state = next_state;
        min += 1;
    }
    (seen[&state], min)
}

fn part_(mins: usize, mut state: Grid<Tile>) -> usize {
    for _ in 0..mins {
        state = tick(&state);
    }
    let trees = state.values().filter(|&&tile| tile == Tile::Trees);
    let lumberyards = state.values().filter(|&&tile| tile == Tile::Lumberyard);
    trees.count() * lumberyards.count()
}

pub fn part1(input: &str) -> usize {
    part_(10, parse(input))
}

pub fn part2(input: &str) -> usize {
    let state = parse(input);
    let (start, end) = find_cycle(state.clone());
    part_(start + (1_000_000_000 - start) % (end - start), state)
}

pub fn tests() {
    let example = [
        ".#.#...|#.",
        ".....#|##|",
        ".|..|...#.",
        "..|#.....#",
        "#.#|||#|#|",
        "...#.||...",
        ".|....|...",
        "||...#|.#|",
        "|.||||..|.",
        "...#.|..|.",
    ]
    .join("\n");
    assert_eq!(part1(&example), 1147);
}

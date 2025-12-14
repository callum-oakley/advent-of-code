use std::collections::{HashMap, HashSet};

use crate::grid::{E, Grid, S, Vector, W};

enum Tile {
    Splitter,
    Empty,
}

fn parse(input: &str) -> (Grid<Tile>, Vector) {
    let mut start = None;
    let g = Grid::parse(input, |v, c| match c {
        'S' => {
            start = Some(v);
            Tile::Empty
        }
        '^' => Tile::Splitter,
        _ => Tile::Empty,
    });
    (g, start.unwrap())
}

fn timelines(cache: &mut HashMap<Vector, usize>, g: &Grid<Tile>, start: Vector) -> usize {
    if !cache.contains_key(&start) {
        let res = match g.get(start) {
            Some(Tile::Splitter) => timelines(cache, g, start + W) + timelines(cache, g, start + E),
            Some(Tile::Empty) => timelines(cache, g, start + S),
            None => 1,
        };
        cache.insert(start, res);
    }
    cache[&start]
}

pub fn part1(input: &str) -> usize {
    let (g, start) = parse(input);

    let mut split_count = 0;
    let mut stack = vec![start];
    let mut beam = HashSet::new();

    while let Some(v) = stack.pop() {
        if beam.contains(&v) {
            continue;
        }
        beam.insert(v);

        match g.get(v) {
            Some(Tile::Splitter) => {
                split_count += 1;
                stack.extend([v + W, v + E]);
            }
            Some(Tile::Empty) => {
                stack.push(v + S);
            }
            None => {}
        }
    }

    split_count
}

pub fn part2(input: &str) -> usize {
    let (g, start) = parse(input);
    timelines(&mut HashMap::new(), &g, start)
}

pub fn tests() {
    let example = concat!(
        ".......S.......\n",
        "...............\n",
        ".......^.......\n",
        "...............\n",
        "......^.^......\n",
        "...............\n",
        ".....^.^.^.....\n",
        "...............\n",
        "....^.^...^....\n",
        "...............\n",
        "...^.^...^.^...\n",
        "...............\n",
        "..^...^.....^..\n",
        "...............\n",
        ".^.^.^.^.^...^.\n",
        "...............\n",
    );
    assert_eq!(part1(example), 21);
    assert_eq!(part2(example), 40);
}

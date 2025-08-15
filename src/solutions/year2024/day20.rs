use std::collections::HashMap;

use crate::{
    freqs::Freqs,
    grid::{Adjacent, Grid, Vector},
    search,
};

fn parse(input: &str) -> (Grid<bool>, Vector) {
    let mut start = None;
    let grid = Grid::parse(input, |v, c| match c {
        '.' | 'E' => true,
        '#' => false,
        'S' => {
            start = Some(v);
            true
        }
        _ => unreachable!(),
    });
    (grid, start.unwrap())
}

fn run(grid: &Grid<bool>, start: Vector) -> HashMap<Vector, i64> {
    search::breadth_first(
        (start, 0),
        |&(pos, picoseconds), push| {
            pos.adjacent4()
                .filter(|&a| grid.get(a).is_some_and(|&t| t))
                .for_each(|a| {
                    push((a, picoseconds + 1));
                });
        },
        search::hash_filter(|&(pos, _)| pos),
    )
    .collect()
}

fn cheats(track: &HashMap<Vector, i64>, max_duration: i64) -> impl Iterator<Item = i64> + '_ {
    track
        .keys()
        .flat_map(|&a| track.keys().map(move |&b| (a, b, (b - a).abs().sum())))
        .filter(move |&(a, b, duration)| {
            duration <= max_duration && track[&b] > track[&a] + duration
        })
        .map(move |(a, b, duration)| track[&b] - track[&a] - duration)
}

pub fn part1(input: &str) -> usize {
    let (grid, start) = parse(input);
    cheats(&run(&grid, start), 2)
        .filter(|&saving| saving >= 100)
        .count()
}

pub fn part2(input: &str) -> usize {
    let (grid, start) = parse(input);
    cheats(&run(&grid, start), 20)
        .filter(|&saving| saving >= 100)
        .count()
}

pub fn tests() {
    let example = [
        "###############",
        "#...#...#.....#",
        "#.#.#.#.#.###.#",
        "#S#...#.#.#...#",
        "#######.#.#.###",
        "#######.#.#...#",
        "#######.#.###.#",
        "###..E#...#...#",
        "###.#######.###",
        "#...###...#...#",
        "#.#####.#.###.#",
        "#.#...#.#.#...#",
        "#.#.#.#.#.#.###",
        "#...#...#...###",
        "###############",
    ]
    .join("\n");
    let (grid, start) = parse(&example);

    assert_eq!(
        cheats(&run(&grid, start), 2).freqs(),
        HashMap::from([
            (2, 14),
            (4, 14),
            (6, 2),
            (8, 4),
            (10, 2),
            (12, 3),
            (20, 1),
            (36, 1),
            (38, 1),
            (40, 1),
            (64, 1),
        ])
    );
    assert_eq!(
        cheats(&run(&grid, start), 20)
            .filter(|&saving| saving >= 50)
            .freqs(),
        HashMap::from([
            (50, 32),
            (52, 31),
            (54, 29),
            (56, 39),
            (58, 25),
            (60, 23),
            (62, 20),
            (64, 19),
            (66, 12),
            (68, 14),
            (70, 12),
            (72, 22),
            (74, 4),
            (76, 3),
        ])
    );
}
